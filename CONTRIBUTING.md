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

CI runs the same three on Linux, macOS and Windows and builds the static Linux and Intel macOS
release targets, since Lua is compiled from source for each one. Every markdown file is linted with
the rules in `.markdownlint.json`:

```sh
npx markdownlint-cli2 "**/*.md" "#target"
```

## Adding a game

A game is ready to merge when it has all of the following:

1. **The game itself**, built on [Zacess Scenes](https://github.com/zacesshq/scenes) so it draws
   with theme roles and never with a fixed colour. Its levels live in Lua files beside the code.
2. **The shared keys**, exactly as [docs/controls.md](docs/controls.md) lists them: `Space`
   pauses, `?` opens how to play, the effects volume, the music volume and mute work the same way
   and `Esc` or `Ctrl+C` quits and gives the terminal back.
3. **A how-to-play page** behind `?`, written in plain words: the goal, the keys and how a round
   ends. The same text goes into its section of [docs/how-to-play.md](docs/how-to-play.md).
4. **Tests** that hold what [ACCESSIBILITY.md](ACCESSIBILITY.md) requires: every colour keeps the
   theme's floor in all eight themes, nothing flashes, reduced motion slows or stills the game and
   the game can be paused at any moment. A game that runs on a timer also needs a test that a
   paused game never loses time.
5. **Documentation**: its row in the README's planned games table moves to its new status, its
   section in each guide replaces "coming", its name goes into the game lists in the three issue
   forms and the changelog's `Unreleased` section says what arrived.

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
