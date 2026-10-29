# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- A release now tells zacess.com to rebuild. Once every platform's files are attached, the release workflow sends a `games-release` repository dispatch naming the tag to `zacesshq/website`, so the site no longer waits for its daily deploy.

## [0.1.0] - 2026-10-29

### Added

- The repository's groundwork: the README, the MIT licence and a licensing notice for the Zacess name and mark, the accessibility requirements every game has to meet, the contributing guide, the security policy, the support guide and the code of conduct.
- Issue forms for a bug, a game idea and an accessibility problem, a pull request template, CODEOWNERS and Dependabot for the actions and the Cargo dependencies.
- The fourteen games in one Cargo package, `zacess-games`, built on Zacess Scenes v0.4.0 for the themes, palette roles, flash meter, motion settings and SVG stills. Stackline joins wire across two wells of falling circuit parts. Merge Gates pushes four-bit tiles together through a track of logic gates. Termle is a daily electronics or programming term with arrow feedback and a daily streak. Signal is a calm rhythm game. Trace routes wire to light every node without crossing. Drift steers a lantern through a landscape with no way to lose. Rooms is a short roguelike through a broken server. Relay moves a packet through timed gates and portals. Through Ball is turn-based football about passing lanes and a keeper's reach. Slipstream is a calm road race against the fuel gauge. Array Guard clears drifting debris from a row of satellite dishes. Heatsink is a typing game that keeps a chip cool. Touch Typing is a trainer with an on-screen keyboard that shows the finger for every key and follows the keyboard in use (UK Mac, UK PC or US). Learn to Code has fifteen tracks from first steps to job-ready with levels checked inside the game.
- A menu, `list`, `sounds` to hear every effect and each game's music, `snap` for SVG stills and `--headless` to play keys without a terminal.
- Every game has a How to play page, synthesised effects levelled like the Scenes tones and its own calm music loop, with separate volumes, mute and settings that are remembered. Colours keep the theme's contrast floor and shapes back up every colour. Frames pass the flash meter and reduced motion slows the games down. Every game fits the window from 60 by 20 up, with a large layout from 160 by 45.
- The Trace and Relay levels and the Learn to Code tracks as Lua files in `levels/`, with extra files read from `~/.config/zacess/games` in a sandbox.
- 125 tests covering each game's rules, every built-in level being solvable, every Learn to Code answer and the contrast, flash and sound limits in every theme.
- `docs/how-to-play.md` and `docs/controls.md` for every game and every key, flag, variable and setting, with 47 stills in `docs/images` that `scripts/snapshots.sh` makes again from the program.
- CI: markdown linting, formatting, Clippy, the tests and a headless run of every game on Linux, macOS and Windows, a check that the stills are current, the release target builds, a check on the minimum Rust version (1.88) and a release workflow that attaches built binaries to the release a maintainer makes from a signed tag.

[Unreleased]: https://github.com/zacesshq/games/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/zacesshq/games/releases/tag/v0.1.0
