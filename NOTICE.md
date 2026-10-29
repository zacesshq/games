# Licensing notice

Zacess Games is licensed under the [MIT licence](LICENSE).

## The Zacess name and mark

The MIT licence covers the games and their code. It does not grant any right to use the Zacess
name or mark to suggest that another product is Zacess or is endorsed by it. A fork or a copy of a
game must carry a name of its own.

## Zacess Scenes

The games are built on [Zacess Scenes](https://github.com/zacesshq/scenes), which is our own and
also under the MIT licence. Its own [licensing notice](https://github.com/zacesshq/scenes/blob/main/NOTICE.md)
lists the crates and the Lua runtime it brings with it.

## Lua

The levels run on [Lua 5.4](https://www.lua.org/), copyright Lua.org, PUC-Rio, under the
[MIT licence](https://www.lua.org/license.html). It is compiled into each binary, so nothing needs
installing. The levels in this repository are our own and under this project's MIT licence.

## Sound and music

Every sound effect and every piece of music is synthesised by the games' own code. No recorded
audio is bundled.

## Games and levels

Every game, its rules, its levels and its text are original and our own. None is a copy of another
game's art, levels or text.

## Dependencies

The released binary is built from these crates, each linked in rather than bundled as a file:

| Crate | Used for | Licence |
| --- | --- | --- |
| zacess-scenes | The themes, the palette roles with their contrast floors, the flash meter, the motion settings, frames and SVG stills | MIT |
| crossterm | Raw mode, the alternate screen, colours and keys | MIT |
| signal-hook (Unix only) | Restoring the terminal on a termination signal | MIT or Apache-2.0 |
| mlua | Running the Lua levels in a sandbox | MIT |
| lua-src | The Lua 5.4 source, compiled into the binary so nothing needs installing | MIT |

mlua also links a few small crates of its own (bstr, either, memchr, num-traits, parking_lot,
rustc-hash and smallvec, with their own dependencies), each under MIT or Apache-2.0 at the
user's choice; memchr is also offered under the Unlicense.

## Stills

The SVG stills in `docs/images` are drawn by the program itself with `zacess-games snap` through
`scripts/snapshots.sh` and are under this project's MIT licence.

## Fonts

No font is bundled. The games draw in the terminal's own font.
