# How to play

The rules of every game in Zacess Games: the goal, the rules, the scoring, how a game is won or lost, a tip and the keys. Each game shows the same page in the terminal with `?` (`F1` in the three typing games). The keys every game shares, the flags and the settings are on the [controls sheet](controls.md).

- [Stackline](#stackline): join rails across two wells
- [Merge Gates](#merge-gates): push bit tiles through gates
- [Termle](#termle): guess the day's tech term
- [Signal](#signal): a calm rhythm of soft tones
- [Trace](#trace): light every node, no crossings
- [Drift](#drift): a lantern in a quiet landscape
- [Rooms](#rooms): mend a broken server's rooms
- [Relay](#relay): a packet through timed gates
- [Through Ball](#through-ball): a turn-based football puzzle
- [Slipstream](#slipstream): a calm road race against fuel
- [Array Guard](#array-guard): clear debris from a dish array
- [Heatsink](#heatsink): type terms to cool a chip
- [Touch Typing](#touch-typing): learn to type by touch
- [Learn to Code](#learn-to-code): code in Python, Java and C

## Stackline

![Stackline in Deep Contrast Dark, part way through a game](images/stackline-deep-dark.svg)

Start it with `zacess-games stackline`.

### Goal

Keep both wells clear for as long as possible by joining rows of wire from rail to rail.

### Rules

- Pieces of one to three circuit parts fall into one of two wells. Each part carries wire across (━), up and down (┃) or both (╋).
- Turning a piece turns it and its parts: an across part becomes upright and back.
- A full row clears only when every part in it carries wire across, joining the + rail on the left to the - rail on the right. A full row that will not clear is marked ✕ at both ends.
- Each clear pushes a feedback row (───) into the bottom of the other well. It has one break (×) that a part carrying wire across can fill.
- Tab moves the falling piece to the other well, if it fits there.

### Scoring

100 points times the square of the rows cleared at once, times the chain: clears on pieces in a row raise the chain by one each. The pieces fall a little faster as rows clear.

### Winning and losing

There is no end to win. The game ends when a well overflows at the top.

### Keys

- ← → (or H L) move the piece.
- ↑ (or K or X) turns it.
- ↓ (or J) moves it down one row. Enter drops it.
- Tab moves it to the other well.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Keep upright parts together in columns you do not need to clear soon.

## Merge Gates

![Merge Gates in Deep Contrast Dark, part way through a game](images/merge-gates-deep-dark.svg)

Start it with `zacess-games merge-gates`.

### Goal

Make a tile equal the target output, as often as you can.

### Rules

- Every tile holds four bits, such as 1010.
- Choose a tile with the cursor and push it one way. It slides until it meets a wall or another tile.
- When it meets a tile the two combine through the gate at the front of the gate track (AND, OR, XOR, NAND, NOR or XNOR), bit by bit, into one tile.
- Every push moves the track on by one gate and a new tile arrives on the board.
- A tile that equals the target is sent out and a new target appears.

### Scoring

100 points for each tile sent out, plus 50 for every merge that went into it.

### Winning and losing

Each target allows 10 pushes. Missing one is a fault. The game ends at the third fault.

### Keys

- Arrows (or H J K L) move the cursor.
- W A S D push the tile under the cursor up, left, down or right, in upper or lower case.
- Enter grabs the tile; the next arrow pushes it.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Read the next gates on the track before you push.

## Termle

![Termle in Deep Contrast Dark, part way through a game](images/termle-deep-dark.svg)

Start it with `zacess-games termle`.

### Goal

Guess the day's electronics or programming term in six tries.

### Rules

- The category is the clue and the board shows how many letters the term has.
- Type a guess of that length and press Enter.
- Each letter is marked: = when it is in the right place, ◂ or ▸ when the term has it somewhere else (pointing the way to the nearest place it belongs) and · when the term does not use it again.
- Once a round, Tab reveals one letter for the cost of one try.
- The daily term is played once a day. Practice terms follow once it is done and after Enter at the end.

### Scoring

100 points for each try left when you solve it, plus 100. Solving the daily term on days in a row builds a streak.

### Winning and losing

Solve the term to win. Six tries without it shows the answer and resets the streak.

### Keys

- Letters type, in upper or lower case alike.
- Backspace deletes. Enter guesses. Tab reveals a letter.

Letters are typed, so `Esc` leaves instead of `Q`. `Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything.

### Tip

An arrow tells you which way to move a letter, so two guesses can place most of them.

## Signal

![Signal in Deep Contrast Dark, part way through a game](images/signal-deep-dark.svg)

Start it with `zacess-games signal`.

### Goal

Catch every note of a calm song as it reaches the line.

### Rules

- Notes drift left along three lanes: high (▲), middle (●) and low (▼).
- Press a note's lane key as it crosses the line on the left. A caught note plays its tone and leaves a ring.
- A press with no note near costs nothing. A note that passes the line is missed.

### Scoring

300 points for a note in time, 100 for a close one, plus 10 for each note of the current run, up to 200.

### Winning and losing

The song ends when every note has passed. There is no way to lose; the end screen shows how many were in time, close and missed.

### Keys

- ↑ or 1 plays the high lane.
- → or 2 plays the middle lane.
- ↓ or 3 plays the low lane.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Watch the notes rather than listen; the tones are a garnish and the song plays the same muted.

## Trace

![Trace in Deep Contrast Dark, part way through a game](images/trace-deep-dark.svg)

Start it with `zacess-games trace`.

### Goal

Light every node on the board by running wire to it from the source (◆).

### Rules

- The pen starts at the source. Each arrow lays one cell of wire from the pen.
- Entering a node lights it. A lit node is a junction: Tab moves the pen to the next lit point so a new wire can branch from it.
- Wires never cross or share a cell, except on a via (┼), where one wire may run straight across another.
- Arrow nodes (△ ▽ ◁ ▷) only take wire arriving the way they point.
- Each level has a wire budget.

### Scoring

100 points for each circuit lit, plus 10 for each cell of wire left in the budget.

### Winning and losing

Light every circuit to win. There is no way to lose: undo or restart a level as often as you like.

### Keys

- Arrows (or H J K L) lay wire.
- Tab or Enter moves the pen to the next lit point.
- U or Backspace undoes. R restarts the level.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Lay the long straight runs first and branch from the nodes they light.

## Drift

![Drift in Deep Contrast Dark, part way through a game](images/drift-deep-dark.svg)

Start it with `zacess-games drift`.

### Goal

Float a lantern over a quiet landscape and gather motes of light.

### Rules

- The land scrolls slowly past. Arrows give the lantern a gentle push and it glides to a stop.
- Touching a mote (✧) gathers it with a soft note and the lantern's halo grows.
- The hills only lift the lantern; nothing can harm it.

### Scoring

One point for every mote gathered.

### Winning and losing

There is no way to lose. Enter ends the journey whenever you like and keeps the light gathered.

### Keys

- Arrows (or H J K L) steer.
- Enter ends the journey.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Reduced motion halves the pace if the scrolling is too much.

## Rooms

![Rooms in Deep Contrast Dark, part way through a game](images/rooms-deep-dark.svg)

Start it with `zacess-games rooms`.

### Goal

Fix every fault in the six rooms of a broken server before its uptime runs out.

### Rules

- Faults are shown as a capital letter and !: C cable, F fan, U fuse and D disk. The parts that fix them (small letters c, f, u and d) lie in the same room.
- Walk onto a part to pick it up. Hands hold two parts; D drops one.
- Walk into a fault holding the matching part to fix it (✓).
- When every fault in a room is fixed its doors open and you can walk through to the next room. The map shows the rooms fixed.
- Glitches (g) wander the racks. One that walks into you costs 8% uptime. Walking into a glitch clears it.

### Scoring

100 points for each fault, 200 more for each room, 50 for each new room reached and 20 for each glitch cleared. Fixing every room adds 10 points for each percent of uptime left.

### Winning and losing

Fix every room to win. The game ends if uptime reaches 0%.

### Keys

- Arrows (or H J K L) walk, in upper or lower case.
- D drops the first part you hold.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Glitches only move when you do, so stop and plan.

## Relay

![Relay in Deep Contrast Dark, part way through a game](images/relay-deep-dark.svg)

Start it with `zacess-games relay`.

### Goal

Steer a packet to collect every data bit (•) and then reach the exit (⇥), in as few moves as you can.

### Rules

- Each arrow moves the packet one cell. W waits a move in place.
- x gates are open on even moves and y gates on odd moves. The board shows each gate as it will be for your next move: open ┊ or shut █.
- Portals with the same letter, such as (A), are linked: step into one to come out of the other.
- Arrow tiles can only be entered moving the way they point.
- The exit opens once every bit is collected.

### Scoring

100 points for each level, plus 5 for every move under twice its par.

### Winning and losing

Deliver every level to win. There is no way to lose: undo or restart a level as often as you like.

### Keys

- Arrows (or H J K L) move.
- W, . or Enter waits a move.
- U or Backspace undoes. R restarts the level.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Waiting is the only way to change the beat, so count the moves to each gate.

## Through Ball

![Through Ball in Deep Contrast Dark, part way through a game](images/through-ball-deep-dark.svg)

Start it with `zacess-games through-ball`.

### Goal

Score as many goals as you can from five attacks.

### Rules

- Your three attackers are (1), (2) and (3); the one with the ball shows ●. Three defenders (✕) and a keeper ([K]) stand in the way.
- Each turn you do one thing: run one cell with the ball, pass or shoot.
- A pass goes to the teammate marked <2> (Tab picks another). It only goes if no defender stands in the straight lane between you; the lane is dotted when open.
- Shoot with 1, 2 or 3 at the top, middle or low row of goal, from column 9 on. A defender in the way blocks it. The keeper covers its own row. From outside the box it covers the rows either side too.
- After your turn your teammates run forward, the nearest defender presses the ball and the others step into the passing lanes. A defender reaching the ball wins it.
- Each attack has a shot clock of 12 turns.

### Scoring

100 points for each goal plus 10 for each turn left on the clock.

### Winning and losing

The match ends after five attacks. An attack ends with a goal, a save, a blocked shot, a tackle or the clock running out.

### Keys

- Arrows (or H J K L) run with the ball.
- Tab picks a teammate. Enter passes.
- 1, 2 and 3 shoot.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Draw the keeper to one side, then shoot to the far row from inside the box.

## Slipstream

![Slipstream in Deep Contrast Dark, part way through a game](images/slipstream-deep-dark.svg)

Start it with `zacess-games slipstream`.

### Goal

Drive as far as you can on one tank of fuel at a time.

### Rules

- Your car (▗▲▲▖) drives up a four-lane road past slower traffic.
- Fuel burns faster the faster you go. Close behind another car, in its slipstream (≈), it burns at well under half.
- The pit lane on the right (P) refills the tank, but holds the car to pit speed.
- Running into a car is a bump: you slow down and the car moves on.

### Scoring

One point for every metre driven. A tone and a note mark every kilometre.

### Winning and losing

There is no finish line. The run ends after three bumps or when an empty tank coasts to a stop.

### Keys

- ← → (or H L) change lane.
- ↑ (or K) goes faster, ↓ (or J) slower.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Tuck in behind a car before the fuel runs low, then pit when the road is clear.

## Array Guard

![Array Guard in Deep Contrast Dark, part way through a game](images/array-guard-deep-dark.svg)

Start it with `zacess-games array-guard`.

### Goal

Protect a row of five satellite dishes from drifting debris for as many waves as you can.

### Rules

- Your turret (◢▲◣) runs along the rail above the dishes and fires straight up.
- Small debris (◇) breaks with one shot. Large debris (◆) takes two and shows ◇ after the first.
- Debris that reaches the bottom hits the nearest dish. Each dish has a shield of three, drawn as a bar over it. A dish with no shield left is lost to the next hit (╳).
- Capsules drift down now and then. Shoot one to collect it: [S] spread shot for ten seconds, [H] one shield back on every dish and [L] a slow field for eight seconds.
- A wave ends when its debris is cleared. The next one brings a little more.

### Scoring

10 points for small debris, 25 for large. Clearing a wave scores 100 times the wave plus 20 for every shield left.

### Winning and losing

There is no last wave. The game ends when every dish is lost.

### Keys

- ← → (or H L, A D) move the turret.
- ↑, F or Enter fire, in upper or lower case. Up to four shots fly at once.

`Space` pauses and `?` opens the How to play page. The effects volume goes up with `+` and down with `-`. The music volume goes up with `]` and down with `[`. `0` mutes everything. `Q` or `Esc` leaves.

### Tip

Large debris is slow; clear the small pieces first.

## Heatsink

![Heatsink in Deep Contrast Dark, part way through a game](images/heatsink-deep-dark.svg)

Start it with `zacess-games heatsink`.

### Goal

Keep the chip (U1) cool by typing the words and code fragments that drift towards it.

### Rules

- Words drift from the right along five heat pipes. Start typing any word: the one nearest the chip that starts with your letter is picked and marked ▸.
- Type it to the end to clear it. Letters match in upper or lower case; symbols and spaces must match.
- A wrong key is counted as a miss and nothing is typed. Backspace takes back the last letter of the picked word.
- Each word that reaches the chip heats it. Each word cleared cools it a little.
- Tab switches calm mode: half the pace and the chip never overheats.

### Scoring

10 points for each character of a cleared word, doubled while your accuracy is 95% or better. The status bar shows words a minute and accuracy.

### Winning and losing

The game ends when the chip reaches 100°C. In calm mode it ends only when you press Enter.

### Keys

- Any character types, the space included.
- Backspace takes back a letter. Tab switches calm mode. Enter ends a calm round.

Every character is typed, the space and symbols included, so `Esc` pauses instead. While paused the sound keys work as in the other games and `Q` leaves. `F1` opens the How to play page. `Ctrl+C` always leaves.

### Tip

Finish the word nearest the chip before starting another.

## Touch Typing

![Touch Typing in Deep Contrast Dark, part way through a game](images/touch-typing-deep-dark.svg)

Start it with `zacess-games touch-typing`.

### Goal

Learn to type without looking, one group of keys at a time, from the home row to timed tests.

### Rules

- There are five levels: Beginner (the home row, finger by finger), Elementary (the top row, then the bottom row), Intermediate (all letters, capitals with Shift and common words), Advanced (numbers, punctuation, symbols, code and electronics text) and Expert (a timed minute and full sentences).
- Type each line as shown. The next key is marked on the keyboard with » «, with the finger to use written under it.
- Each key shows its finger by shape and colour: ( ) little finger, { } ring finger, [ ] middle finger, < > index finger. The space bar is for either thumb.
- A wrong key is marked ^ under the letter you missed and counted. Backspace lets you fix it, but the miss still counts.
- The keyboard follows your layout: UK Mac (Apple British), UK PC or US. It is read from the system and named above the line. F2 switches it and the choice is remembered. The finger hints, the Shift and Option hints and the symbols lesson all follow it.

### Scoring

Words a minute count five characters as a word. Accuracy is the share of keys typed right. The weakest keys are the three you miss most. The score is 100 for each lesson unlocked plus your best speed.

### Winning and losing

There is no losing. Finish a lesson with 95% accuracy or better to unlock the next one. Below that, Enter tries it again with new text.

### Keys

- Every character types, the space included. Shift types capitals and the upper symbols.
- Backspace takes back the last key. ← and → choose any unlocked lesson. Enter moves on when a lesson is done. F2 switches the keyboard layout.

Every character is typed, the space and symbols included, so `Esc` pauses instead. While paused the sound keys work as in the other games and `Q` leaves. `F1` opens the How to play page. `Ctrl+C` always leaves.

### Tip

Keep your fingers resting on A S D F and J K L ; and feel for the bumps on F and J.

## Learn to Code

![Learn to Code in Deep Contrast Dark, part way through a game](images/learn-to-code-deep-dark.svg)

Start it with `zacess-games learn-to-code`.

### Goal

Learn to program from first steps to job-ready, one idea at a time. There are tracks for Python, Java, C, C++, JavaScript, TypeScript, SQL, Bash and Linux, Git, Rust, Go, C# and Kotlin, a shared Data Structures and Algorithms track and an AI track in Python.

### Rules

- Pick a track with ↑ and ↓ and press Enter. The list is your progress overview: each track shows how many levels are done. Each remembers where you got to.
- Every level has a short lesson and a few lines of code with blanks, shown as [ ]. Type into the blank that is marked ▸; Tab moves to the next blank.
- Enter checks your answers. Spaces outside quotes never matter. When everything is right the program's output is shown and Enter goes on to the next level.
- A wrong answer gets a message that explains the mistake. F2 shows a hint. After three tries F3 shows the answer.
- Code is checked inside the game against the answers each level lists. Nothing you type is ever run on the computer.
- F4 goes back to choose another language.

### Scoring

100 points for each level solved without the answer shown, less 20 for each wrong try, never below 20.

### Winning and losing

There is no losing. A track is complete when its last level is solved. The language list shows how many levels each language has done.

### Keys

- Every character types, the space included. Backspace deletes.
- Tab moves to the next blank. Enter checks, then moves on.
- F2 hint. F3 show the answer after three tries. F4 choose a language.

Every character is typed, the space and symbols included, so `Esc` pauses instead. While paused the sound keys work as in the other games and `Q` leaves. `F1` opens the How to play page. `Ctrl+C` always leaves.

### Tip

Read the expected output under the code first; it tells you what each blank must do.
