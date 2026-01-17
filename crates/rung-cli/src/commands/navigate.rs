//! `rung nxt` and `rung prv` commands - Navigate the stack.

use super::utils::open_repo_and_state;
use crate::output;
use anyhow::{Result, bail};

/// Navigate to the next (child) branch in the stack.
pub fn run_next() -> Result<()> {
    let (repo, state) = open_repo_and_state()?;

    let current = repo.current_branch()?;
    let stack = state.load_stack()?;

    // Find children of current branch
    let children = stack.children_of(&current);

    match children.len() {
        0 => {
            output::info(&format!("'{current}' has no children in the stack"));
            Ok(())
        }
        1 => {
            let child = &children[0].name;
            repo.checkout(child)?;
            output::success(&format!("Switched to '{child}'"));
            Ok(())
        }
        _ => {
            output::warn(&format!("'{current}' has multiple children. Choose one:"));
            for child in children {
                println!("  → {}", child.name);
            }
            bail!("Use `git checkout <branch>` to switch to the desired branch");
        }
    }
}

/// Navigate to the previous (parent) branch in the stack.
pub fn run_prev() -> Result<()> {
    let (repo, state) = open_repo_and_state()?;

    let current = repo.current_branch()?;
    let stack = state.load_stack()?;

    // Find current branch in stack
    let branch = stack.find_branch(&current);

    if let Some(parent) = branch.and_then(|b| b.parent.as_ref()) {
        repo.checkout(parent)?;
        output::success(&format!("Switched to '{parent}'"));
    } else {
        output::info(&format!(
            "'{current}' has no parent in the stack (it's a root branch)"
        ));
    }
    Ok(())
}
