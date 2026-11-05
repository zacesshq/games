# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- The Zacess Games mark in `assets/` in light, dark, automatic and single colour versions, with a light and a dark lockup at the head of the README. A dark and a light banner and a 1280 by 640 social preview image sit beside them.

## [0.4.0] - 2026-11-04

### Changed

- The Homebrew formula and the winget manifests under `packaging/` are kept at the current version with one `SHA256_` placeholder per platform, named after the release target. `packaging/fill-checksums.sh` writes copies with the sums from a release's `.sha256` files filled in. The packaging and release steps in `packaging/README.md` and `CONTRIBUTING.md` follow.

## [0.3.0] - 2026-11-03

### Added

- Learn to Code edits inside a blank. The thin bar `▏` is the cursor: `←` and `→` move it, `Home` and `End` jump to the ends, a typed character goes in at the cursor, `Backspace` deletes before it and `Delete` after it. A blank still holds 40 characters at most.
- Learn to Code moves between levels. `PgUp` opens the level before and `PgDn` the level after, as far as the track's progress reaches, so no level is skipped. `→` on a track in the track list opens its level list, with the levels done ticked, the next one marked and the rest shown as locked. `Enter` there opens any level reached again.
- A Try it line after every solved Learn to Code level: the shell command that runs that language, such as `python3 main.py` or `go run main.go`, with the file to save the code as. The game itself still runs nothing.
- `docs/scores.md`, the reference for the scores file: where it lives, its format and every key the games write.
- `packaging/` with the Homebrew formula template and the winget manifests, with the steps a maintainer follows after a release.
- `home`, `end`, `del`, `pgup` and `pgdn` as key words for `--keys` and recordings.

### Changed

- `zacess-games` is published to crates.io, so `cargo install --locked zacess-games` works without a git tag. Zacess Scenes comes from crates.io too (0.6) instead of a git tag. The README's install section covers Homebrew, winget, Cargo and the release archives per platform, with a diagram of how a game is drawn.
- A Learn to Code level played again keeps its tick and scores nothing more; the 100 points are for the first solve.
- Community files follow the [AI policy](https://github.com/zacesshq/.github/blob/main/AI_POLICY.md): pull requests declare AI use and issue forms ask whether AI helped. Security reports go to security@zacess.com.
- Routine tidy-up.

## [0.2.0] - 2026-10-29

### Added

- Showcase recordings of all fourteen games in `docs/frames`, seven seconds of real play each at ten frames a second, 80 by 24, in Zacess Deep Contrast Dark. They are JSON in the same format as the Zacess Scenes showcase frames, so the website plays them with the same player as the scenes. Stackline clears a row, Merge Gates combines tiles into the target, Termle solves the daily term, Signal catches notes on the beat, Trace wires two levels, Drift gathers motes, Rooms fetches a part and fixes a fault, Relay passes a timed gate, Through Ball passes and scores, Slipstream takes a pit stop, Array Guard clears debris, Heatsink and Touch Typing are typed and Learn to Code fills in a blank.
- `snap --frames N` with `--fps`, `--keys` and `--script`. It replays a scripted key sequence with a fixed seed against the real game logic and renderer without a terminal, samples frames on the game's own clock and passes each one through the flash meter, holding back any frame that would change a large area too often.
- `scripts/recordings.txt`, the keys, seed and start of every recording, so a recording changes without touching the code. It is built into the binary.
- Tests that every recording is the same on every run, keeps the flash limit in all eight themes and shows play rather than a still, plus tests of the recording script and of the recorder's timing.
- `scripts/snapshots.sh` makes the frames as well as the stills and CI fails when either is out of date.

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

[Unreleased]: https://github.com/zacesshq/games/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/zacesshq/games/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/zacesshq/games/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/zacesshq/games/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/zacesshq/games/releases/tag/v0.1.0
