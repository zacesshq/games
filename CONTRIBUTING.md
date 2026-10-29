# Contributing to Zacess Games

Thank you for helping make games that everyone can play. This guide covers how a change is made,
how a game is added and how a release goes out.

## How a change happens

1. Every change starts from an issue that says what "done" means. A game idea uses the
   [game idea form](https://github.com/zacesshq/games/issues/new?template=game_idea.yml) and an
   accessibility problem the
   [accessibility problem
   form](https://github.com/zacesshq/games/issues/new?template=accessibility_problem.yml).
2. Work on a branch named after its type and topic, such as `feat/termle-daily-word` or
   `fix/stackline-pause-on-resize`.
3. Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/): `feat:`,
   `fix:`, `docs:`, `test:`, `refactor:` and `chore:`.
4. Open a pull request that closes its issue. It is squash-merged once the checks pass and it is
   reviewed.

## Checks

With a stable Rust toolchain:

```sh
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
```

CI runs the same three on Linux, macOS and Windows, plays every game without a terminal at three
sizes and builds the static Linux and Intel macOS release targets, since Lua is compiled from
source for each one. It also checks the crate on Rust 1.88, the `rust-version` in `Cargo.toml`.

The stills in `docs/images` are drawn by the program. After a change to how a game looks, make them
again and commit the result, since CI fails when they differ:

```sh
./scripts/snapshots.sh
```

A single game can be played without a terminal to see a frame as text:

```sh
cargo run -- --headless relay --keys right,right --size 100x30
```

Every markdown file is linted with the rules in `.markdownlint.json`:

```sh
npx markdownlint-cli2 "**/*.md" "#target"
```

## Adding a game

A game is ready to merge when it has all of the following:

1. **The game itself**, a file in `src/games` that implements the `Game` trait in `src/game.rs`
   and joins the list in `src/games/mod.rs`. It draws with the theme roles of
   [Zacess Scenes](https://github.com/zacesshq/scenes) and never with a fixed colour. Any levels
   live in Lua files under `levels/`.
2. **The shared keys**, exactly as [docs/controls.md](docs/controls.md) lists them: `Space`
   pauses, `?` opens how to play, the sound keys work the same way, `Esc` goes back to the menu
   and `Ctrl+C` leaves and gives the terminal back. A game where every key types moves pause to
   `Esc` and how to play to `F1`, as the typing games do.
3. **A how-to-play page** behind `?`, written in plain words: the goal, the rules, the scoring, how
   the game is won or lost, a tip and the keys. The same text goes into its section of [docs/how-to-play.md](docs/how-to-play.md).
4. **Tests** that hold what [ACCESSIBILITY.md](ACCESSIBILITY.md) requires: every colour keeps the
   theme's floor in all eight themes, nothing flashes, reduced motion slows or stills the game and
   the game can be paused at any moment. A game that runs on a timer also needs a test that a
   paused game never loses time.
5. **Documentation**: a row and a still in the README's games tables, its section in
   [docs/how-to-play.md](docs/how-to-play.md), its keys in [docs/controls.md](docs/controls.md),
   its signals in [ACCESSIBILITY.md](ACCESSIBILITY.md), its name in the game lists of the three
   issue forms, its stills from `scripts/snapshots.sh` and an entry in the changelog's
   `Unreleased` section.

> [!IMPORTANT]
> Nothing may flash. Nothing that changes in place may change a cell more than three times a
> second. Anything periodic keeps its period in a named constant with a test that holds it. A
> change that loosens one of those limits cannot merge.

## Sound and music

Every effect and every piece of music is synthesised by the game's own code. No recorded audio is
added to the repository. Effects and music have separate volumes and both obey mute. Every sound
event has a matching change on screen, so nothing is heard without also being seen.

## Project conventions

- **Accessibility is a requirement, not a feature.** See [ACCESSIBILITY.md](ACCESSIBILITY.md) for
  what every game has to keep to.
- **Original work only.** Rules, levels, text and art are our own. A game may share a genre with
  another game but never its art, levels or text.
- **Few dependencies.** A game uses Zacess Scenes and the crates it already brings before reaching
  for a new one.

## Releasing

1. Bump the version in `Cargo.toml` (and `Cargo.lock` by running `cargo build`), move the
   changelog's `Unreleased` entries into a section for the version and merge that through a pull
   request.
2. A maintainer tags the merged commit with a signed annotated tag, pushes it and creates the
   GitHub release from the tag with that version's changelog section as the notes.
3. The tag runs [`release.yml`](.github/workflows/release.yml), which builds the binaries for every
   platform and attaches them to that release. It waits for the release to exist and never creates
   one.
