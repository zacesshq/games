# The scores file

Everything Zacess Games remembers between runs is in one plain text file, `scores.txt`. It holds
the best score of each game, the Termle streak, the Touch Typing lessons unlocked, the Learn to
Code progress of each track and the sound and keyboard settings. The file can be read, edited or
deleted by hand; a missing or damaged file just means no records.

## Where it lives

| System | Path |
| --- | --- |
| macOS and Linux | `~/.local/state/zacess-games/scores.txt` |
| With `XDG_STATE_HOME` set | `$XDG_STATE_HOME/zacess-games/scores.txt` |
| Windows | `%LOCALAPPDATA%\zacess-games\scores.txt` |
| With `ZACESS_GAMES_STATE` set | `$ZACESS_GAMES_STATE/scores.txt`, on any system |

`ZACESS_GAMES_STATE` wins over everything else, which is how the stills, the showcase frames and
the tests keep a developer's own records out of what they make. The folder is created the first
time something is saved.

## Format

One `key value` pair a line, the key and the value split by a single space, the value a whole
number of zero or more. Keys are kept in alphabetical order and the file is written again after
every change. The first line is a comment that starts with `#`; any line whose value is not a whole
number is ignored when the file is read, so a note can be left in it.

```text
# Zacess Games records: best scores and the daily Termle
code-language 0
code-python 3
learn-to-code 280
setting-background-on 0
setting-background-volume 15
setting-effects-on 1
setting-effects-volume 35
setting-music-on 1
setting-music-volume 20
termle-best-streak 4
termle-day 20760
termle-streak 4
trace 1200
typing-unlocked 2
typing-wpm-0 31
```

A best score is only ever replaced by a higher one and a score of zero is never written. The
`zacess-games list` command prints every game with its best score from this file.

## Keys

### Best scores

One key per game, named after the game's id: `stackline`, `merge-gates`, `termle`, `signal`,
`trace`, `drift`, `rooms`, `relay`, `through-ball`, `slipstream`, `array-guard`, `heatsink`,
`touch-typing` and `learn-to-code`. The value is the best score in that game's own unit (points,
rows, light, moves and so on, as the How to play page of each game says).

### Termle

| Key | Value |
| --- | --- |
| `termle-day` | The day the daily term was last finished, as days since 1 January 1970 |
| `termle-streak` | Days in a row with the daily term finished. It starts again at one after a missed day |
| `termle-best-streak` | The longest streak so far |

### Touch Typing

| Key | Value |
| --- | --- |
| `typing-unlocked` | The highest lesson unlocked, counted from zero |
| `typing-wpm-<n>` | The best words a minute in lesson `n`, one key per lesson played |
| `setting-keyboard` | The layout chosen with `F2`: `0` for UK Mac, `1` for UK PC and `2` for US. Without this key the layout comes from the flag, the variable or the system |

### Learn to Code

| Key | Value |
| --- | --- |
| `code-<track id>` | Levels done in that track, such as `code-python` or `code-rust`. The ids are the file names in [`levels/code`](../levels/code/) without `.lua`. The game opens the track at this level and its level list ticks every level below it |
| `code-language` | The track opened last, as its place in the track list counted from zero |

### Sound

| Key | Value |
| --- | --- |
| `setting-effects-volume` | The effects volume, 0 to 100 |
| `setting-effects-on` | `1` when the effects play and `0` when they are muted |
| `setting-music-volume` | The music volume, 0 to 100 |
| `setting-music-on` | `1` when the music plays and `0` when it is muted |
| `setting-background-volume` | The background sound's volume, 0 to 100 |
| `setting-background-on` | `1` when the background sound plays and `0` when it is off |

A flag or an environment variable wins over a saved setting for that run and does not change the
file; see [the controls sheet](controls.md#settings-and-records).

## Clearing records

Delete the file or just the lines to forget while the games are not running. To start one track of
Learn to Code again, remove its `code-` line. To try the daily Termle again on a day it was already
finished, remove `termle-day`; the streak lines can stay.
