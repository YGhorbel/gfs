<!--
Thank you for contributing to GFS!

Please follow the PR title conventions:
🎉 New Database Support: [name]
✨ Feature: add [description]
🐛 Fix: [description]
📝 Documentation update
🚨 Breaking change
-->

## Related Issue
<!-- 
Link the issue this PR addresses. If no issue exists, consider creating one first.
Closes #(issue number)
-->

## What
<!--
Describe what problem this change solves and why it's needed.
Provide background context for reviewers unfamiliar with this area.
-->

## How
<!--
Explain how your code changes achieve the solution.
Highlight key implementation decisions and any trade-offs made.
-->

## Review Guide
<!--
Help reviewers navigate your changes:
1. Start with `src/main.rs` - main logic changes
2. `src/lib.rs` - supporting changes
3. Skip `tests/integration.rs` - just test updates
-->

## Testing
<!--
How was this tested? What scenarios were covered?
- [ ] Manual testing performed
- [ ] Unit tests added/updated
- [ ] E2E tests added/updated
-->

## Documentation
<!--
- [ ] Code comments added for complex logic
- [ ] README or docs updated if needed
- [ ] CHANGELOG.md updated (if applicable)
-->

## User Impact
<!--
What's the end result for users?
- What improves?
- Are there any breaking changes or side effects?
-->

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Tests pass locally (`cargo test`)
- [ ] Clippy passes (`cargo clippy --all-targets --all-features -- -D warnings`)
- [ ] Format check passes (`cargo fmt --check`)
- [ ] This PR can be safely reverted if needed