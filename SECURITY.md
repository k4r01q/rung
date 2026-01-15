# Security Policy

## Supported Versions

We take the security of rung seriously. The following versions are currently being supported with security updates:

| Version  | Supported          |
| -------- | ------------------ |
| Latest   | :white_check_mark: |
| < Latest | :x:                |

We recommend always using the latest version of rung to ensure you have all security patches and improvements.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability in rung, please report it to us privately. This allows us to assess the issue and provide a fix before public disclosure.

### How to Report

Use GitHub's private vulnerability reporting feature:

1. Go to https://github.com/auswm85/rung/security/advisories/new
2. Click "Report a vulnerability"
3. Fill out the advisory form with the following information:

- **Description**: A clear description of the vulnerability
- **Impact**: What kind of security issue is this? (e.g., code execution, information disclosure, privilege escalation)
- **Steps to Reproduce**: Detailed steps to reproduce the vulnerability
- **Affected Versions**: Which versions of rung are affected
- **Proof of Concept**: If possible, include code or commands that demonstrate the vulnerability
- **Suggested Fix**: If you have ideas on how to fix the issue, please share them

### What to Expect

After you submit a report, here's what you can expect:

1. **Acknowledgment**: We will acknowledge receipt of your vulnerability report within 48 hours
2. **Initial Assessment**: We will provide an initial assessment within 5 business days
3. **Updates**: We will keep you informed about our progress as we work on a fix
4. **Disclosure**: Once a fix is available, we will coordinate with you on the disclosure timeline

### Our Commitments

- We will respond to your report in a timely manner
- We will keep you informed about our progress
- We will credit you for your discovery (unless you prefer to remain anonymous)
- We will not take legal action against researchers who:
  - Make a good faith effort to avoid privacy violations and data destruction
  - Only interact with their own accounts or test accounts
  - Do not exploit the vulnerability beyond what is necessary to demonstrate it

## Security Best Practices

When using rung, we recommend the following security practices:

### GitHub Authentication

- **Use Personal Access Tokens (PATs)**: Rung requires a GitHub Personal Access Token for GitHub operations
- **Minimal Permissions**: Create tokens with only the permissions necessary for your workflow (typically `repo` scope)
- **Token Storage**: Store tokens securely using your system's credential manager
- **Rotate Tokens**: Regularly rotate your GitHub tokens
- **Never Commit Tokens**: Never commit tokens to version control

### Git Configuration

- **SSH Keys**: Use SSH keys for Git authentication when possible
- **Signed Commits**: Consider using GPG-signed commits for verification
- **Review Changes**: Always review the changes rung makes to your repository state

### General Security

- **Keep Updated**: Always use the latest version of rung to benefit from security patches
- **Verify Downloads**: Verify the integrity of downloaded binaries using checksums
- **Review Code**: Review the source code and changes, especially if building from source
- **Limit Scope**: Use rung only in repositories you trust and control

## Security Features

Rung implements the following security features:

- **Local State Management**: Branch relationships and stack information are stored locally in Git configuration
- **No Data Collection**: Rung does not collect or transmit usage data
- **Open Source**: The entire codebase is open source and available for security review
- **Minimal Dependencies**: We minimize external dependencies to reduce the attack surface

## Known Security Considerations

### Git Operations

Rung performs Git operations on your local repository, including:

- Creating and switching branches
- Rebasing branches
- Pushing to remote repositories

**Recommendation**: Always review the changes rung proposes before executing commands that modify your repository state.

### GitHub API Access

Rung requires a GitHub Personal Access Token to:

- Create and update pull requests
- Add comments to pull requests
- Query repository information

**Recommendation**: Use fine-grained tokens with minimal required permissions when available.

### File System Access

Rung reads and writes to:

- Git configuration files (`.git/config`)
- Local Git objects and references
- Temporary files during operations

**Recommendation**: Only use rung in directories you trust and control.

## Vulnerability Disclosure Policy

When we receive a security vulnerability report, we follow this process:

1. **Confirmation**: Confirm the vulnerability and determine its severity
2. **Development**: Develop and test a fix
3. **Release**: Release a patched version
4. **Disclosure**: Publicly disclose the vulnerability after users have had time to update (typically 7-14 days after patch release)
5. **Credit**: Credit the reporter in the release notes and security advisory (if they wish)

### CVE Assignment

For critical vulnerabilities, we will:

- Request a CVE identifier
- Publish a GitHub Security Advisory
- Update documentation with mitigation steps

## Security Audit History

We welcome security audits of rung. If you're interested in performing a security audit, please contact us.

- No formal security audits have been conducted as of this writing
- Community contributions and reviews are always welcome

## Contact

For security-related questions or concerns that are not vulnerability reports, you can:

- Open a discussion on [GitHub Discussions](https://github.com/auswm85/rung/discussions) (for general security questions)
- Open an issue on [GitHub Issues](https://github.com/auswm85/rung/issues) (for non-sensitive questions)

## Updates to This Policy

This security policy may be updated from time to time. Significant changes will be announced through:

- GitHub release notes
- Project README
- Security advisories (if applicable)

---

Thank you for helping keep rung and its users safe!
