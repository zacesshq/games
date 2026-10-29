# Accessibility

Zacess Games is a Zacess product, so it is built for people with low vision, light sensitivity
and motion sensitivity first. This statement describes what the games do today on screen, in sound
and at the keyboard. The contrast floor, the flash limit, the pause and help keys and the fit of
the key lines are checked by the tests in `src/`.

## Colour and contrast

- **The theme is the one in use.** The games read `--theme`, then `ZACESS_THEME`, then the Zacess
  theme state file that [Zacess Scenes](https://github.com/zacesshq/scenes/blob/main/docs/themes.md)
  reads, so the games and the scenes always match. [docs/controls.md](docs/controls.md#the-theme)
  has the order.
- **Every colour keeps the theme's floor.** Each colour comes from a Zacess palette role that is
  checked against the theme background: at least 4.5 to 1 on most themes and at least 7 to 1 on the
  High Contrast and Deep Contrast themes. A role that would miss the floor falls back to the text
  colour. A test draws every game in all eight themes and checks every colour on screen.
- **The background is painted.** Every cell is drawn on the theme's own background, so the contrast
  holds even when the terminal is set to another theme. With `--colour 256` each colour maps to the
  nearest table entry that still keeps the floor.
- **Colour is never the only signal.** Every piece, tile and state has a shape or letter of its own
  as well:
  - Stackline parts are `━`, `┃`, `╋` and `───` for feedback. A full row that will not clear is
    marked `✕` at both ends and the rails are marked `+` and `-`.
  - Merge Gates tiles show their bits as text. The tile under the cursor has a double outline, a
    grabbed tile a `✥` and the target is drawn as filled and empty squares beside its bits.
  - Termle marks letters `[A]=` in place, `◂` or `▸` elsewhere and `·` not used. The letter strip
    uses `=`, `~` and `·` the same way.
  - Signal lanes are `▲`, `●` and `▼` with their keys written beside them. Results are written: In
    time, Close or Missed.
  - Trace nodes are hollow when unlit and filled with brackets when lit. Arrow nodes point their way
    and the pen is bracketed with `❰ ❱`.
  - Rooms faults are a capital letter and `!`, parts a small letter, fixed faults `✓`, glitches `g≈`
    and locked doors a solid bar.
  - Relay gates are `┊` when open and `██` when shut for the next move, with the gate letter beside
    them. Portals are `(A)` and the exit is bracketed while it is shut.
  - Through Ball attackers are numbered `(1)`, the ball carrier carries `●`, the chosen pass target
    is `<2>`, defenders are `✕` and the keeper is `[K]`. An open lane is dotted with `•` and the
    panel says in words whether the pass is open or blocked and which goal rows the keeper covers.
  - Slipstream's car has a pointed front (`▗▲▲▖`) where traffic is square. The slipstream shows as
    `≈` behind the car ahead and in words on the dash. The pit lane is walled by a heavy line and
    marked `P` and `+`. After a bump the car turns the warning colour and the bump count shows as
    `✕` marks.
  - Array Guard's debris is `◇` (one hit) or `◆` (two hits), capsules carry their letter (`[S]`,
    `[H]` or `[L]`) and each dish's shield is a bar over it as well as a count in the panel. A lost
    dish turns to `╳`.
  - Heatsink marks the word being typed with `▸` and shows the typed part in another colour. The
    chip's heat is a gauge with the degrees written under it.
  - Touch Typing shows each key's finger by bracket shape as well as colour: `( )` little, `{ }`
    ring, `[ ]` middle and `< >` index. The next key is marked `» «` and its finger is written out
    in words, with Shift or Option when needed. The keyboard follows the layout in use (UK Mac, UK
    PC or US), found from the system, so the hints match the keys under the player's fingers. A
    missed key is marked `^` underneath, never by blinking.
  - Learn to Code marks the blank being typed into with `▸`, marks right answers `✓` and wrong ones
    `✗` and explains every mistake in words.
- **Plain glyphs.** `--basic` swaps the geometric shapes for ASCII stand-ins. The same happens on its
  own in a console known to lack them (the Linux console and the classic Windows console).

## Motion and flashing

- **Nothing flashes.** Every frame passes through the Scenes flash meter, which holds back any frame
  that would change a large area more than three times in a second, inside the limit WCAG 2.3.1
  sets. A test plays every game in every theme and checks the limit. Clears, merges and wins are
  marked by a calm change of shape rather than blinking: a joined row turns to `═══` for a third of
  a second, a merged tile gets a heavy outline and a caught note leaves a ring that shrinks in three
  steps.
- **Reduced motion.** `--reduced-motion`, `ZACESS_REDUCED_MOTION=1` or the system setting (Reduce
  Motion on macOS, animations off in GNOME or in Windows) slows things down. Stackline falls at 1.6
  seconds a row instead of 0.9. Drift, Slipstream, Array Guard and Heatsink move at half pace.
  Signal plays at 54 beats a minute instead of 72 and the clear and solve pauses are shortened.
- **Turn based where it can be.** Termle, Trace, Rooms, Relay, Through Ball, Touch Typing and Learn
  to Code never move on their own (Touch Typing's one timed lesson only counts down). Only
  Stackline, Signal, Drift, Slipstream, Array Guard and Heatsink run in time. Heatsink has a calm
  mode with no way to lose. Drift has no way to lose. Slipstream starts at a gentle speed, only goes
  faster when asked and never shakes or blinks on a bump.
- **A pause key everywhere.** `Space` pauses every game (`Esc` in the three typing games) and any
  key carries on. Opening the help pauses too.

## Sound and music

- **On by default and easy to change.** Effects start at volume 35 and music at 20, out of 100. The
  right end of every status bar always shows both (`FX 35% · Music 20%` or `FX muted · Music off`).
  The second key line under every game and the menu lists the keys. Effects go up with `+` and down
  with `-`. Music goes up with `]` and down with `[`. `0` mutes everything. Changes are remembered
  for the next run.
- **Loud enough to hear.** Effects are levelled to the same loudness Zacess Scenes uses, so a game
  at 35 is as loud as a scene at 35. No tone is shorter than 0.12 seconds. Frequent actions get a
  quieter tick with a short gap between ticks, so fast keys never stack into noise.
- **Calm music.** Each game has a soft loop in its own key and tempo, levelled well below the
  effects. It stops while a game is paused or its page is open and stops on every way out, so no
  player is left running.
- **Synthesised, never recorded.** Every effect and every loop is made by the program itself and
  played by the system's own player. No recorded audio is bundled.
- **Sound is never the only signal.** Every effect has a written twin above the key lines, such as
  "Node lit, 2 to go". Music never carries meaning. Every game plays the same muted.
- **Test it outside a game.** `zacess-games sounds` plays every effect with its meaning and then
  each game's music.

## Keyboard and screen readers

- **A plain line first.** Before the screen is taken over, a plain line says which game started and
  how to pause, get help and quit, so a screen reader announces it.
- **How to play on every game.** `?` (`F1` in the typing games) opens a page with the goal, the
  rules, the scoring, how the game is won or lost and the keys, wrapped to the window and scrolled
  with `↑` and `↓`. It names the full controls sheet, `docs/controls.md`. The menu opens the same
  page for the chosen game. The same text is in [docs/how-to-play.md](docs/how-to-play.md), so it
  can be read with a screen reader before a game starts.
- **Typing games keep their keys for typing.** In Heatsink, Touch Typing and Learn to Code every
  character types, so `Esc` pauses and opens the sound keys, `F1` opens the page and `Ctrl+C`
  leaves.
- **A clean way out.** `Ctrl+C` always leaves. `Esc` goes back to the menu in most games and
  pauses in the typing games. `Q` goes back too, except in Termle where it is a letter. The terminal is restored and `Zacess Games stopped.` is printed on every way out:
  quitting, a panic, `Ctrl+C` and on macOS and Linux `SIGTERM`, `SIGHUP`, `SIGQUIT` and `SIGINT`.
- **Keys in both cases.** Letters work in upper and lower case in every game apart from the typing
  games, where every key types exactly what it shows. Arrow keys work in every game that moves
  anything, with `H`, `J`, `K` and `L` as well. The mouse is never needed.

## Size

- **Fits any window from 60 by 20.** A compact layout keeps every game playable in a quarter tmux
  window. From 160 by 45 boards are drawn at twice the size, which helps at a distance. Below the
  minimum a centred message gives the size needed and the size now.
- **The window has one layout.** Every game uses the same frame: the title and theme in the top
  border, a status bar with the sound setting at its right end, the play area, a line of the game's
  own keys, a line of the keys every game shares and a help box with the full keys.

## Known limits

> [!NOTE]
> The terminal's alternate screen is not read by screen readers, so a game played on it cannot be
> followed by ear alone. The plain lines before and after a game, the written twin of every sound
> and the guides in `docs` are what a screen reader user gets.

Many of these are preferences: the theme, the volumes, the keyboard layout and reduced motion are
all flags or variables. If a different default would help others, open an issue or a pull request.

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
