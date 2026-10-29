# Zacess Games

Original games for the terminal, drawn in the eight [Zacess](https://zacess.com) high contrast
themes. Each game is written in Rust with its levels in Lua and is built on
[Zacess Scenes](https://github.com/zacesshq/scenes), the library behind the Zacess terminal scenes.

> [!IMPORTANT]
> The games are in development. This repository holds the documentation, the checks and the
> release workflow so far. The code arrives game by game and nothing is ready to install yet.

## What every game has

- **Zacess themes with contrast floors.** Every colour keeps at least 4.5 to 1 contrast on the
  theme's background. The High Contrast and Deep Contrast themes keep at least 7 to 1.
- **No flashing.** Reduced motion is honoured from a flag, an environment variable and the system
  setting.
- **Sound effects and music**, synthesised by the game itself. Each has its own volume and both can
  be muted.
- **A how-to-play page** behind `?` in every game, so the rules are never more than one key away.
- **Levels in Lua**, so a level can be read, changed and shared without building anything.

## Planned games

The set below is planned. Which of them are released is still being decided.

| Game | What it is | Status |
| --- | --- | --- |
| Stackline | Details to come | Planned |
| Merge Gates | Details to come | Planned |
| Termle | Details to come | Planned |
| Signal | Details to come | Planned |
| Trace | Details to come | Planned |
| Drift | Details to come | Planned |
| Rooms | Details to come | Planned |
| Relay | Details to come | Planned |
| Football | A football game | Planned |
| Racing | A racing game | Planned |
| Typing | A typing game | Planned |
| Arcade shooter | An arcade shooter that clears drifting debris | Planned |
| Learn to code | A learn-to-code game with Python, Java and C tracks | Planned |
| Touch typing | A touch typing trainer with levels | Planned |

## Install

> [!NOTE]
> Coming with the first release. Each release will carry built binaries for macOS, Linux and
> Windows with a SHA-256 file beside each, in the same way as
> [Zacess Scenes releases](https://github.com/zacesshq/scenes/releases/latest).

## Documentation

| Page | Covers |
| --- | --- |
| [How to play](docs/how-to-play.md) | What every game shares and each game's rules as it arrives |
| [Controls](docs/controls.md) | The keys every game shares and each game's own keys as it arrives |
| [Accessibility](ACCESSIBILITY.md) | The commitments every game has to meet |
| [Contributing](CONTRIBUTING.md) | How a game is added, the checks and the release steps |

## Accessibility

Zacess products are built for people with low vision, light sensitivity and motion sensitivity
first. Every game keeps the theme's contrast floor, never flashes and slows down or holds still
under reduced motion. Neither colour nor sound is ever the only signal. Every game can be paused at
any moment. The full set of requirements is in [ACCESSIBILITY.md](ACCESSIBILITY.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Ideas for a game are welcome through the
[game idea form](https://github.com/zacesshq/games/issues/new?template=game_idea.yml).

## Licence

[MIT](LICENSE). The licence does not cover the Zacess name or mark; see [NOTICE.md](NOTICE.md).
