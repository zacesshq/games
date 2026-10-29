# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- The repository's groundwork ahead of the first game: the README with the planned games, the MIT licence and a licensing notice for the Zacess name and mark, the accessibility requirements every game has to meet, the contributing guide, the security policy, the support guide and the code of conduct.
- Skeleton guides in `docs/how-to-play.md` and `docs/controls.md` holding the keys every game shares.
- Issue forms for a bug, a game idea and an accessibility problem, a pull request template, CODEOWNERS and Dependabot for the actions.
- CI: markdown linting, a Rust workflow (formatting, Clippy and tests on Linux, macOS and Windows plus release target builds) that skips cleanly until the first `Cargo.toml` arrives and a release workflow that attaches built binaries to the release a maintainer makes from a signed tag.

[Unreleased]: https://github.com/zacesshq/games/commits/main
