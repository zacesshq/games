# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0] - 2026-11-08

### Added

- A background sound under each game, beside the music, from the Zacess Scenes background sounds: a machine room hum under Stackline, Merge Gates, Trace, Rooms, Relay and Heatsink, a slow pad under Termle and Signal, birdsong under Drift, wind under Through Ball, distant traffic under Slipstream, a night bed under Array Guard and rain under Touch Typing and Learn to Code. It is off until turned on with `}`, `--background` or `ZACESS_AMBIENT=1`, starts at 15 out of 100 and `{` turns it down and off at 0. The level and whether it is on are remembered. It stops with the music while a game is paused or its page is open.
- A warning tone: two even notes that sound once when something needs attention soon. Slipstream sounds it as the fuel drops below a quarter, Heatsink as the chip passes 70 percent of the way to its limit and Array Guard as a dish loses its last shield. Each comes with a written line and the warning colour on screen at the same moment. Calm mode in Heatsink never sounds it.
- `zacess-games sounds` plays the warning tone with the other effects and six seconds of each game's background sound after its music.
- The flags `--no-background` and `--background-volume`. The settings keys `setting-background-on` and `setting-background-volume` keep the choice.

### Changed

- Builds on Zacess Scenes 1.1.0, which brings the five new background sounds the games use.
- The status bar names the background sound's level while it is on, such as `FX 35% · Music 20% · Background 25%`.
- The how-to-play pages and the pause screen list the `{` and `}` keys.

## [1.0.0] - 2026-11-08

### Added

- Zacess Warm Dark and Zacess Warm Light, so the games are drawn in all ten Zacess themes. Both keep the 4.5 to 1 text floor the palette sets for them, pick up the `warm-dark` and `warm-light` short names for `--theme` and appear in the theme lists of the bug and accessibility issue forms. They reach the games through Zacess Scenes 1.0.0, which carries palette 1.0.0.
- Tests that draw every game in every theme in the palette rather than a fixed list, that no theme sets a floor below 4.5 to 1 and that the six piece hues, the text and muted status colours, the fault, warning and success colours and each game's own colour stay apart in every theme.
- The Zacess Games mark in `assets/` in light, dark, automatic and single colour versions, with a light and a dark lockup at the head of the README. A dark and a light banner and a 1280 by 640 social preview image sit beside them.
- Pixel art boards in all fourteen games, drawn on the Zacess Scenes pixel canvas as half blocks, two pixels to a character cell, in the theme's own colours. Stackline's wells are circuit boards with copper chips and solder pads, Merge Gates has rounded tiles in a tray of sockets, Termle sets each letter on a tile with a lip and Signal's notes are triangles and beads that aim at outlines on the line. Trace lays copper traces between a diamond source and ring nodes, Drift glides a framed lantern over hills with pines, reeds, a moon and fireflies (a sun, clouds and butterflies on a light theme), Rooms is a raised floor of server cabinets and Relay a checkered floor with brick walls, gems, ring portals and barred gates. Through Ball is a striped pitch with a netted goal and small players who wear their numbers, Slipstream a road seen from above with tree-lined verges and cars with tyres and windscreens, Array Guard a night sky of tumbling rocks over radio dishes on masts and a domed cannon, Heatsink a chip fed by copper heat pipes, Touch Typing a keyboard of keycaps with solid lips and Learn to Code an editor page with each blank in a slot of its own and the output on a small screen. Scores, labels, typed words and code stay as text on top.
- Boards that grow with the window. Each board draws itself larger as the window grows, at standard as well as large sizes, keeping its own grid and rules, so a large window gives bigger and finer shapes instead of a doubled small drawing. Merge Gates is still drawn at half size and doubled from 160 by 45.
- Tests that text on pixel art keeps the theme's floor against the colour it sits on in every game and theme, that every pixel in Signal, Slipstream, Stackline, Termle, Through Ball, Touch Typing and Trace is either ground under 3 to 1 or a solid shape at 3.2 to 1 or more at four window sizes. Others check that the ground, surface and shade colours keep those limits in every theme.

### Changed

- The terminal paints each cell's own background, so the pixel art and the tiles under text reach it. With `--colour 256` pixel colours take the nearest table entry without the text floor, as they do in Zacess Scenes, while text keeps the floor.
- Text set on pixel art keeps the colour under it as its background instead of cutting a hole in the board. The large-window zoom and the crop for small windows carry pixel cells whole.
- The how-to-play pages in the games and in `docs/how-to-play.md` describe the pieces by their new shapes.
- The stills in `docs/images` and the recordings in `docs/frames` are made again. The stills draw the boards in one crisp-edged group with runs of one colour merged and the recordings write pixels as full and lower half blocks, both from Scenes, so neither shows thin lines between rows.
- Builds on Zacess Scenes 1.0.0 from crates.io, which brings the pixel canvas and the ten themes.
- `ACCESSIBILITY.md` and the README describe the pixel art, the growing boards and the contrast rules for both. They also state the real floors: 7 to 1 on every theme but Warm Dark and Warm Light, which keep 4.5 to 1.
- `docs/controls.md` describes the standard and large window sizes with the growing pixel boards. CONTRIBUTING and the winget description count ten themes.
- The version placeholders in the bug and accessibility issue forms read 1.0.0.
- SUPPORT.md points to the install steps in the README in place of the old note that there was nothing to install yet. It gives support@zacess.com for help with a game or a level and for private accessibility reports, as does the accessibility problem form, with contact@zacess.com kept for anything else.

### Fixed

- Drift no longer overflows its sky and hill arithmetic on a large random seed, which stopped a debug build in the random key test. Release builds draw exactly as before.

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

[Unreleased]: https://github.com/zacesshq/games/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/zacesshq/games/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/zacesshq/games/compare/v0.4.0...v1.0.0
[0.4.0]: https://github.com/zacesshq/games/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/zacesshq/games/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/zacesshq/games/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/zacesshq/games/releases/tag/v0.1.0
