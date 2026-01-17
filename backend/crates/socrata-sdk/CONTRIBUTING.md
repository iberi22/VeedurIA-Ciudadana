# Contributing by Socrata SDK

Thank you for your interest in contributing to the **Socrata SDK for Rust**! This library aims to be the standard Rust client for the Socrata Open Data API (SODA).

## Getting Started

1.  **Fork and Clone**: Fork this repository and clone it locally.
2.  **Environment Setup**: Ensure you have a recent version of Rust (stable).
3.  **Dependencies**: Run `cargo build -p socrata-sdk` to fetch dependencies.

## Development Constraints

*   **No Unnecessary Dependencies**: Keep the dependency graph lightweight. Focus on `reqwest`, `serde`, and `tokio`.
*   **Async First**: All I/O operations must be asynchronous.
*   **Typed Builders**: Use the Builder pattern for constructing queries (`SocrataQuery`).

## Running Tests

We prioritize high test coverage.

```bash
# Run unit tests
cargo test -p socrata-sdk --lib

# Run integration tests (Requires SOCRATA_APP_TOKEN env var for higher limits)
cargo test -p socrata-sdk --test integration_tests
```

## Pull Request Process

1.  Create a feature branch: `git checkout -b feat/my-feature`.
2.  Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/).
3.  Ensure `cargo fmt` and `cargo clippy` pass cleanly.
4.  Open a Pull Request against the `master` branch.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
