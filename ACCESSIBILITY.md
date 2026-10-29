# Accessibility

Zacess Games is a Zacess product, so it is built for people with low vision, light sensitivity
and motion sensitivity first. No game has been released yet, so this statement sets out the
requirements every game has to meet before it merges. Once a game arrives, its measured limits and
the tests that hold them are added here, as [Zacess Scenes](https://github.com/zacesshq/scenes/blob/main/ACCESSIBILITY.md)
does for its scenes.

## Keys

- Every game shares the same keys for the same jobs: `Space` pauses and resumes, `?` opens how to
  play, the effects volume, the music volume and mute have keys of their own and `Esc` or `Ctrl+C`
  quits. [docs/controls.md](docs/controls.md) lists them.
- A game is played with the keyboard alone. The mouse is never needed.
- Nothing needs a modifier key apart from `Ctrl+C`. Where a key has an uppercase form that does
  something different, the guides say so.
- A held key never makes anything flash: key repeats that would toggle a state are ignored.
- The keys a game uses are shown when it starts and on its how-to-play page.

## Pausing and time

- Every game can be paused at any moment with `Space`. A paused game says so and says how to
  resume.
- A game that runs on a timer never loses time while paused or while its how-to-play page is open.
- Opening how to play with `?` pauses the game first.
- Where a game has a speed or a time limit, a slower setting or one with no time limit is offered
  wherever the game still makes sense without it.

## Motion and flashing

- Nothing flashes. No game changes the whole screen or a large area at once more than three times
  a second, inside the three a second that WCAG 2.3.1 allows. Nothing that changes in place changes
  a cell more than three times a second.
- Reduced motion is on when the game is started with a flag, when `ZACESS_REDUCED_MOTION` is set or
  when the system asks for it on macOS, GNOME or Windows. It slows or stills anything that moves for
  effect and removes shaking, sparkles and flicker. Movement that is the game itself, such as a
  falling piece, keeps working at a pace the player can choose.
- Each periodic limit lives in a named constant with a test that holds it.

## Colour and contrast

- Every colour comes from the Zacess palette through the theme roles of Zacess Scenes. No game sets
  a fixed colour.
- Every colour a game draws keeps at least 4.5 to 1 contrast on the theme's background and at least
  7 to 1 on the High Contrast and Deep Contrast themes. Tests check every colour in all eight
  themes.
- Colour is never the only signal. Pieces, teams, states and right or wrong answers also differ in
  shape, character or words. With `NO_COLOR` set every game stays playable.

## Sound and music

- Every sound effect and every piece of music is synthesised by the game. No recorded audio is
  bundled.
- Effects and music have separate volumes and both can be muted. Mute is one key and is shown on
  screen.
- Sound is never the only signal. Every sound event has a matching change on screen at the same
  moment.
- Sounds are levelled to a shared loudness with soft starts and ends, so they never click or jump
  in volume.

## How to play

- `?` opens a how-to-play page in every game: the goal, the keys and how a round ends, in plain
  words. The same text is in [docs/how-to-play.md](docs/how-to-play.md), so it can be read with a
  screen reader before a game starts.

## Before and after a game

- A plain line is printed before a game takes over the screen, naming the game, the theme, whether
  reduced motion is on and the keys that quit, so a screen reader hears what started.
- When a game ends by a quit key, by `Ctrl+C`, by a panic or by a termination signal, the terminal
  is given back as it was.

## Size

Every game lays itself out for the terminal it is in and again on every resize. The smallest size
each game supports is stated in its guide. A terminal smaller than that gets a plain message rather
than a squeezed game.

## Known limits

> [!NOTE]
> The terminal's alternate screen is not read by screen readers, so a game played on it cannot be
> followed by ear alone. The plain lines before and after a game and the guides in `docs` are what a
> screen reader user gets. Each game's own limits are added here when it arrives.

## Reporting a barrier

An accessibility problem is a bug. Please open an issue with the
[accessibility problem
form](https://github.com/zacesshq/games/issues/new?template=accessibility_problem.yml),
describing what you were doing and what got in the way. You never need to share a diagnosis or
anything personal. To report privately, email [contact@zacess.com](mailto:contact@zacess.com).

## The shared statement

The organisation's [accessibility
statement](https://github.com/zacesshq/.github/blob/main/ACCESSIBILITY.md)
covers every Zacess repository. This file takes precedence where the two differ.
