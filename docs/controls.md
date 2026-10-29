# Controls

Every key, command, flag, variable and setting in Zacess Games on one page. Key names follow `Ctrl+C` style. Letters work in upper and lower case alike unless a row says otherwise: in the three typing games every key types exactly what it shows, so there `a` and `A` are different characters. The rules of each game are in [How to play](how-to-play.md).

## Commands

| Command | Does |
| --- | --- |
| `zacess-games` | The menu of all fourteen games |
| `zacess-games <game>` | One game by name, such as `zacess-games trace` or `zacess-games through-ball`. A short start is enough, such as `zacess-games stack` |
| `zacess-games list` | Every game and its best score, as plain text |
| `zacess-games sounds` | Plays every effect with its name and meaning, then eight seconds of each game's music |
| `zacess-games sounds <game>` | Plays eight seconds of one game's music |
| `zacess-games snap [game]` | Writes SVG stills of one game or all of them into the current folder |
| `zacess-games --help` | Every option |
| `zacess-games --version` | The version |

## Keys every game shares

Most games read keys only. Termle types letters. Heatsink, Touch Typing and Learn to Code type every character, the space included, so a few shared keys move there.

| Key | Most games | Termle | Heatsink, Touch Typing and Learn to Code |
| --- | --- | --- | --- |
| `Space` | Pause and carry on | Pause and carry on | Types a space |
| `?` | How to play page | How to play page | Types `?` |
| `F1` | How to play page | How to play page | How to play page |
| `↑` `↓` | Scroll the How to play page while it is open | The same | The same |
| `+` or `=` | Effects volume up by 10 | The same | While paused only |
| `-` or `_` | Effects volume down by 10 | The same | While paused only |
| `]` | Music volume up by 10 | The same | While paused only |
| `[` | Music volume down by 10 | The same | While paused only |
| `0` | Mute everything; again to bring it back | The same | While paused only |
| `Esc` | Back to the menu | Back to the menu | Pause, with the sound keys |
| `Q` | Back to the menu | Types `Q` | Back to the menu while paused |
| `Ctrl+C` | Leave at once | Leave at once | Leave at once |
| `Enter` | Play again after a game ends | The same | The same |

While paused, any key that is not a sound key carries on. The right end of the status bar always shows both levels, such as `FX 35% · Music 20%` or `FX muted · Music off`. Every change is also written above the key lines as it happens.

## The menu

| Key | Does |
| --- | --- |
| `↑` `↓` or `K` `J` | Choose a game |
| `←` `→` or `H` `L` | Move between the two columns |
| `Enter` | Play the chosen game |
| `1` to `9` | Play one of the first nine games at once |
| `?` or `F1` | The chosen game's How to play page |
| `+` `-` `[` `]` `0` | The sound, as in every game |
| `Q` or `Esc` | Leave |

## Each game

| Game | Keys |
| --- | --- |
| Stackline | `←` `→` (or `H` `L`) move. `↑` (or `K` or `X`) turns. `↓` (or `J`) moves one row down. `Enter` drops. `Tab` moves the piece to the other well |
| Merge Gates | Arrows (or `H` `J` `K` `L`) move the cursor. `W` `A` `S` `D` push the tile up, left, down or right. `Enter` grabs; then an arrow pushes |
| Termle | Letters type. `Backspace` deletes. `Enter` guesses. `Tab` reveals a letter for one try |
| Signal | `↑` or `1` plays the high lane. `→` or `2` the middle lane. `↓` or `3` the low lane |
| Trace | Arrows (or `H` `J` `K` `L`) lay wire. `Tab` or `Enter` moves the pen to the next lit point. `U` or `Backspace` undoes. `R` restarts |
| Drift | Arrows (or `H` `J` `K` `L`) steer. `Enter` ends the journey |
| Rooms | Arrows (or `H` `J` `K` `L`) walk. `D` drops a part |
| Relay | Arrows (or `H` `J` `K` `L`) move. `W` or `Enter` waits a move (so does the full stop). `U` or `Backspace` undoes. `R` restarts |
| Through Ball | Arrows (or `H` `J` `K` `L`) run with the ball. `Tab` picks a teammate. `Enter` passes. `1` `2` `3` shoot at the top, middle or low row |
| Slipstream | `←` `→` (or `H` `L`) change lane. `↑` (or `K`) goes faster and `↓` (or `J`) slower |
| Array Guard | `←` `→` (or `H` `L` or `A` `D`) move. `↑` or `F` or `Enter` fires |
| Heatsink | Every character types; letters match in either case. `Backspace` takes back a letter. `Tab` switches calm mode. `Enter` ends a calm round |
| Touch Typing | Every character types exactly. `Backspace` takes back a key. `←` `→` choose an unlocked lesson. `Enter` moves on when a lesson is done. `F2` switches the keyboard layout |
| Learn to Code | Every character types into the marked blank. `Backspace` deletes. `Tab` or `↓` goes to the next blank and `↑` the one before. `Enter` checks, then moves on. `F2` shows a hint. `F3` shows the answer after three tries. `F4` chooses a language |

## Flags

| Flag | Does |
| --- | --- |
| `--theme <name>` | A Zacess theme by id, short name or full name, such as `deep-dark` |
| `--reduced-motion` | Slower falling, scrolling and rhythm, shorter pauses for clears |
| `--no-sound` | Start with effects and music muted |
| `--volume <0 to 100>` | The effects volume to start with |
| `--no-music` | Start with the music off |
| `--music-volume <0 to 100>` | The music volume to start with |
| `--seed <number>` | A fixed seed, for the same game twice |
| `--colour <truecolour or 256>` | How colours reach the terminal |
| `--basic` | Plain characters for fonts without geometric shapes |
| `--layout <uk-mac or uk-pc or us>` | The keyboard layout Touch Typing teaches on, for this run |
| `--headless <game> --keys <keys> --size <W>x<H>` | Plays keys without a terminal and prints the screen as text |
| `snap --theme all --out <dir>` | Stills for every theme into a folder |

## Environment variables

| Variable | Does |
| --- | --- |
| `ZACESS_THEME` | The theme, under `--theme` and over the theme state file |
| `ZACESS_REDUCED_MOTION=1` | Reduced motion, as the flag |
| `ZACESS_SOUND=0` | Start muted, as `--no-sound` |
| `ZACESS_VOLUME` | The effects volume, 0 to 100 |
| `ZACESS_MUSIC_VOLUME` | The music volume, 0 to 100 |
| `ZACESS_KEYBOARD` | The keyboard layout: `uk-mac`, `uk-pc` or `us` |
| `ZACESS_GAMES_STATE` | Another folder for the records and settings |
| `ZACESS_GAMES_NO_USER_LEVELS=1` | Ignore level files in `~/.config/zacess/games` |

The system's reduced motion setting (Reduce Motion on macOS, animations off in GNOME or in Windows) turns reduced motion on as well.

## Window sizes

Every game and the menu lay themselves out for the window they are in, chosen again on every resize without losing the game:

| Size | Layout |
| --- | --- |
| Below 60 by 20 | A centred message giving the size needed and the size now |
| From 60 by 20 (a quarter tmux window) | Compact: one status line, one key line, side panels left out where they would not fit (their facts move to the status line) and smaller boards |
| From 100 by 30 | Standard: a status bar, two key lines and the full side panels |
| From 160 by 45 | Large: boards drawn at twice the size, a larger Touch Typing keyboard of boxed keys with a progress panel and wider text in the typing and coding games |

## Keyboard layouts

Touch Typing draws the keyboard and teaches the fingers for one of three layouts:

| Layout | Where | Differences |
| --- | --- | --- |
| `uk-mac` | Apple British on a Mac | `@` is `Shift+2` and `"` is `Shift+'`. `£` is `Shift+3` and `#` is `Option+3`. `§` and `±` sit left of `1`; the backtick and `~` sit left of `Z` |
| `uk-pc` | UK keyboards on Windows and Linux | `"` is `Shift+2` and `@` is `Shift+'`. `£` is `Shift+3`. `#` and `~` sit left of `Enter`; the backslash and the pipe sit left of `Z`; the backtick and `¬` sit left of `1` |
| `us` | US ANSI keyboards | `@` is `Shift+2`, `#` is `Shift+3` and `"` is `Shift+'` |

The layout comes from the `--layout` flag, then `ZACESS_KEYBOARD`, then the one chosen last time with `F2`, then the system: on macOS the current input source (`com.apple.keylayout.British` gives `uk-mac`), on Windows the first layout in `HKCU\Keyboard Layout\Preload` (`00000809` gives `uk-pc`) and on Linux `XKB_DEFAULT_LAYOUT`, `localectl status` or `setxkbmap -query` (`gb` gives `uk-pc`). When none of these answers, it is `uk-mac` on a Mac and `uk-pc` elsewhere, never US. The lesson screen names the layout in use.

## The theme

The games are drawn in a Zacess theme picked in this order: `--theme`, then `ZACESS_THEME`, then the theme state file `$XDG_STATE_HOME/zacess/theme` or `~/.local/state/zacess/theme`, then Zacess Deep Contrast Dark. The state file is the one [Zacess Scenes](https://github.com/zacesshq/scenes/blob/main/docs/themes.md) reads, so any tool that writes it switches the games and the scenes together. It holds a theme id such as `deep-contrast-dark`. It can instead hold `auto` followed by a dark and a light id, to follow the system's dark mode. On Windows the file is under the user folder, `%USERPROFILE%\.local\state\zacess\theme`.

## Settings and records

The effects and music levels and whether each is muted are remembered between runs. So are the best scores, the Termle streak, the Touch Typing lessons unlocked and keyboard layout and the Learn to Code progress for each language. They live in `~/.local/state/zacess-games/scores.txt` (`$XDG_STATE_HOME/zacess-games` when that is set and `%LOCALAPPDATA%\zacess-games` on Windows), one `key value` pair a line. A flag or variable wins over a remembered setting for that run.

Sound and music files are made once and kept in `~/Library/Caches/zacess-games` on macOS, `~/.cache/zacess-games` on Linux and `%LOCALAPPDATA%\zacess-games` on Windows. They are safe to delete; they are made again when needed.

## Level files

Trace, Relay and Learn to Code read their levels from Lua files. Extra files go in `~/.config/zacess/games/trace`, `~/.config/zacess/games/relay` or `~/.config/zacess/games/learn-to-code` (`%APPDATA%\zacess\games` on Windows) and play after the built-in ones. The format is described at the top of each built-in file in [`levels/`](../levels/).
