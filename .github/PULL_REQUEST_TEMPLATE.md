## What this changes

<!-- Describe the problem and resulting behavior. Link the issue if there is one. -->

## Component

<!-- herdr_usage, skills/<name>, procedures/<name>, or repository infrastructure. -->

## Validation

- [ ] Component documentation and instructions are updated where needed
- [ ] Required checks for the affected component were run; blockers are documented
- [ ] No credentials or private session content are included

<!-- For herdr_usage, run from herdr_usage/:
     cargo fmt --all -- --check
     cargo clippy --release --all-targets --all-features --locked -- -D warnings
     cargo test --all-targets --all-features --locked
     Parser changes need representative fixtures in herdr_usage/tests/fixtures/. -->
