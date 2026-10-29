# Workflows

| Workflow | Runs on | What it does |
| --- | --- | --- |
| [`ci.yml`](ci.yml) | Push to `main` and pull requests | First looks for a `Cargo.toml`. Until the first game's code arrives there is none, so the Rust jobs are skipped and the run passes. Once there is one: on Linux, macOS and Windows it checks formatting, runs Clippy with warnings as errors and runs the tests. It also builds the Intel macOS target and the static x86-64 and ARM64 Linux targets, since Lua is compiled from source with each target's C compiler |
| [`release.yml`](release.yml) | A tag starting with `v`, such as `v0.1.0` | Builds every binary in the workspace for Apple silicon and Intel Macs, static Linux for x86-64 and ARM64 (musl) and 64 bit Windows with the C runtime linked statically (set in `.cargo/config.toml`). Each archive holds the games with the README, licence, notice, changelog, accessibility statement and guides plus a SHA-256 file. The files are attached to the GitHub release of that tag. The release is made by a maintainer from a signed tag, so the workflow waits up to twenty minutes for it to exist and never creates one itself. Rerunning it replaces the files. Attaching files to a published release needs immutable releases to stay off for this repository |
| [`markdownlint.yml`](markdownlint.yml) | Push to `main` and pull requests, also run by hand | Lints every markdown file with the rules in `.markdownlint.json` |

Every action is pinned to a full commit SHA with its version in a comment. Dependabot proposes updates
weekly for the actions and, once a `Cargo.toml` exists, the Cargo dependencies.
