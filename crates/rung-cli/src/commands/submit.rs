//! `rung submit` command - Push branches and create/update PRs.

use anyhow::{bail, Context, Result};
use rung_core::State;
use rung_git::Repository;
use rung_github::{Auth, CreatePullRequest, GitHubClient, UpdatePullRequest};

use crate::output;

/// Run the submit command.
pub fn run(draft: bool, force: bool) -> Result<()> {
    // Open repository
    let repo = Repository::open_current().context("Not inside a git repository")?;

    // Get state manager
    let workdir = repo.workdir().context("Cannot run in bare repository")?;
    let state = State::new(workdir)?;

    // Ensure initialized
    if !state.is_initialized() {
        bail!("Rung not initialized - run `rung init` first");
    }

    // Ensure working directory is clean
    repo.require_clean()?;

    // Load stack
    let mut stack = state.load_stack()?;

    if stack.is_empty() {
        output::info("No branches in stack - nothing to submit");
        return Ok(());
    }

    // Get remote info
    let origin_url = repo.origin_url().context("No origin remote configured")?;
    let (owner, repo_name) =
        Repository::parse_github_remote(&origin_url).context("Could not parse GitHub remote URL")?;

    output::info(&format!("Submitting to {owner}/{repo_name}..."));

    // Create GitHub client
    let auth = Auth::auto();
    let client = GitHubClient::new(&auth).context("Failed to authenticate with GitHub")?;

    // Use tokio runtime for async operations
    let rt = tokio::runtime::Runtime::new()?;

    // Track results
    let mut created = 0;
    let mut updated = 0;

    // Process each branch
    for i in 0..stack.branches.len() {
        let branch = &stack.branches[i];
        let branch_name = branch.name.clone();
        let parent_name = branch.parent.clone();
        let existing_pr = branch.pr;

        output::info(&format!("Processing {}...", branch_name));

        // Push the branch
        output::info(&format!("  Pushing {}...", branch_name));
        repo.push(&branch_name, force)
            .with_context(|| format!("Failed to push {}", branch_name))?;

        // Determine base branch for PR
        let base_branch = parent_name.as_deref().unwrap_or("main");

        // Generate PR title (use branch name, replacing - and _ with spaces)
        let title = branch_name
            .split('/')
            .last()
            .unwrap_or(&branch_name)
            .replace(['-', '_'], " ");
        let title = title
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        // Generate stack navigation body
        let body = generate_pr_body(&stack.branches, i);

        if let Some(pr_number) = existing_pr {
            // Update existing PR
            output::info(&format!("  Updating PR #{}...", pr_number));

            let update = UpdatePullRequest {
                title: None, // Don't change title
                body: Some(body),
                base: Some(base_branch.to_string()),
            };

            rt.block_on(client.update_pr(&owner, &repo_name, pr_number, update))
                .with_context(|| format!("Failed to update PR #{}", pr_number))?;

            updated += 1;
        } else {
            // Check if PR already exists for this branch
            let existing = rt
                .block_on(client.find_pr_for_branch(&owner, &repo_name, &branch_name))
                .context("Failed to check for existing PR")?;

            if let Some(pr) = existing {
                // PR exists but we didn't know about it - update stack
                output::info(&format!("  Found existing PR #{}...", pr.number));
                stack.branches[i].pr = Some(pr.number);

                // Update the PR body
                let update = UpdatePullRequest {
                    title: None,
                    body: Some(body),
                    base: Some(base_branch.to_string()),
                };

                rt.block_on(client.update_pr(&owner, &repo_name, pr.number, update))
                    .with_context(|| format!("Failed to update PR #{}", pr.number))?;

                updated += 1;
            } else {
                // Create new PR
                output::info(&format!("  Creating PR ({} → {})...", branch_name, base_branch));

                let create = CreatePullRequest {
                    title,
                    body,
                    head: branch_name.clone(),
                    base: base_branch.to_string(),
                    draft,
                };

                let pr = rt
                    .block_on(client.create_pr(&owner, &repo_name, create))
                    .with_context(|| format!("Failed to create PR for {}", branch_name))?;

                output::success(&format!("  Created PR #{}: {}", pr.number, pr.html_url));
                stack.branches[i].pr = Some(pr.number);
                created += 1;
            }
        }
    }

    // Save updated stack with PR numbers
    state.save_stack(&stack)?;

    // Summary
    if created > 0 || updated > 0 {
        let mut parts = vec![];
        if created > 0 {
            parts.push(format!("{} created", created));
        }
        if updated > 0 {
            parts.push(format!("{} updated", updated));
        }
        output::success(&format!("Done! PRs: {}", parts.join(", ")));
    } else {
        output::info("No changes to submit");
    }

    Ok(())
}

/// Generate PR body with stack navigation links.
fn generate_pr_body(branches: &[rung_core::stack::StackBranch], current_idx: usize) -> String {
    let mut body = String::new();

    // Stack context section
    body.push_str("## Stack\n\n");

    for (i, branch) in branches.iter().enumerate() {
        let is_current = i == current_idx;
        let marker = if is_current { "→" } else { " " };

        if let Some(pr_num) = branch.pr {
            body.push_str(&format!("{} #{} - `{}`\n", marker, pr_num, branch.name));
        } else {
            body.push_str(&format!("{} (pending) - `{}`\n", marker, branch.name));
        }
    }

    body.push_str("\n---\n");
    body.push_str("*Managed by [rung](https://github.com/amcshan/rung)*\n");

    body
}
