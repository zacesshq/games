#!/usr/bin/env bash
# regenerates the stills in docs/images and the showcase frames in docs/frames from the program
# itself, so they always match what the games draw. CI runs this and fails when the committed
# files differ.
set -euo pipefail

cd "$(dirname "$0")/.."
cargo build --release --quiet
bin=target/release/zacess-games
[ -x "$bin" ] || bin="$bin.exe"

# a fixed state folder and keyboard keep a developer's records, settings, level files and
# system layout out of the stills
unset ZACESS_THEME ZACESS_REDUCED_MOTION ZACESS_SOUND ZACESS_VOLUME ZACESS_MUSIC_VOLUME
export ZACESS_GAMES_STATE="$PWD/target/snapshots-no-state"
export ZACESS_GAMES_NO_USER_LEVELS=1
export ZACESS_KEYBOARD=uk-mac
export XDG_STATE_HOME="$PWD/target/snapshots-no-state"
rm -rf "$ZACESS_GAMES_STATE"

stills=docs/images
frames=docs/frames
work=target/snapshots
rm -rf "$stills" "$frames" "$work"
mkdir -p "$stills" "$frames" "$work"

# every game and the menu in Deep Contrast Dark and in Light at the standard size
for theme in deep-dark light; do
  "$bin" snap all --theme "$theme" --size 100x30 --out "$stills" >/dev/null
done

# a few games in other themes
snap() { "$bin" snap "$1" --theme "$2" --size 100x30 --out "$stills" >/dev/null; }
snap array-guard hc-dark
snap relay hc-dark
snap heatsink yellow
snap rooms yellow
snap slipstream yellow
snap stackline cream
snap touch-typing cream
snap learn-to-code hc-light
snap through-ball hc-light
snap trace hc-light

# the compact and large layouts, renamed after their size
for game in merge-gates stackline touch-typing; do
  "$bin" snap "$game" --theme light --size 60x20 --out "$work" >/dev/null
  mv "$work/$game-light.svg" "$stills/$game-compact-light.svg"
done
for game in merge-gates stackline touch-typing; do
  "$bin" snap "$game" --theme deep-dark --size 207x48 --out "$work" >/dev/null
  mv "$work/$game-deep-dark.svg" "$stills/$game-large-deep-dark.svg"
done
"$bin" snap all --theme deep-dark --size 207x48 --out "$work" >/dev/null
mv "$work/menu-deep-dark.svg" "$stills/menu-large-deep-dark.svg"
rm -rf "$work"

# each game's recording from scripts/recordings.txt, seven seconds at ten frames a second in the
# same JSON format as the Scenes frames, so one web player shows both
"$bin" snap all --frames 70 --fps 10 --theme deep-dark --size 80x24 --out "$frames" >/dev/null
