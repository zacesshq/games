## What changed

<!-- What changed and why. -->

## Closes

<!-- Closes #N -->

## How it was checked

- [ ] `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass (tick this too for a change with no Rust)
- [ ] A changed game was played in a real terminal in a dark theme and a light theme and resized while it ran
- [ ] A changed level loads and can be finished
- [ ] A changed sound or piece of music was listened to at both volumes and with mute on

## Accessibility impact

- [ ] Nothing flashes and nothing that changes in place changes a cell more than three times a second (tick this too when no motion changed)
- [ ] Reduced motion still slows or stills the game
- [ ] Every colour drawn comes from the theme and the contrast tests still pass
- [ ] Every sound has a matching change on screen
- [ ] `Space` still pauses at any moment and a paused game loses no time
- [ ] Any new key is on the game's how-to-play page, in `docs/controls.md` and in ACCESSIBILITY.md where it matters

## Documentation

- [ ] The README, `docs/how-to-play.md`, `docs/controls.md` and the issue forms name any new or renamed game
- [ ] The changelog's `Unreleased` section says what changed

## Before submitting

- [ ] I have read the [Code of Conduct](https://github.com/zacesshq/games/blob/main/CODE_OF_CONDUCT.md) and [CONTRIBUTING.md](https://github.com/zacesshq/games/blob/main/CONTRIBUTING.md)
