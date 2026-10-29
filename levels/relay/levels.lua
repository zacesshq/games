-- Relay levels. Each level is a table with a name, a note, a par (the move count to beat) and a
-- map.
--   #  wall          .  floor         @  start        E  exit
--   o  data bit      x  gate open on even moves      y  gate open on odd moves
--   A B C D          linked portals, two of each letter
--   < > ^ v          a tile entered only moving that way
-- A file may return one level or a list. Maps may be built with Lua, as the last one is.

local levels = {
  {
    name = "Hello packet",
    note = "Collect the bit, then the exit opens.",
    par = 10,
    map = {
      "###########",
      "#@...o...E#",
      "###########",
    },
  },
  {
    name = "Even and odd",
    note = "x is open on even moves. W waits a move when the beat is wrong.",
    par = 13,
    map = {
      "###########",
      "#@..x..o..#",
      "#.#######.#",
      "#....y...E#",
      "###########",
    },
  },
  {
    name = "Portal pair",
    note = "Step into a letter to come out of its twin.",
    par = 28,
    map = {
      "#############",
      "#@..A#..o..B#",
      "#######.#####",
      "#o..B#A....E#",
      "#############",
    },
  },
  {
    name = "One-way street",
    note = "Arrow tiles only let you through their way.",
    par = 18,
    map = {
      "###########",
      "#@.>...o..#",
      "#.###v###.#",
      "#.o.....<.#",
      "#.#######.#",
      "#E........#",
      "###########",
    },
  },
  {
    name = "Switchyard",
    note = "Two gate kinds in a row. Count before you move.",
    par = 29,
    map = {
      "#############",
      "#@...x.y...o#",
      "#.#########.#",
      "#.o..y.x....#",
      "#.#########.#",
      "#E..........#",
      "#############",
    },
  },
  {
    name = "Crossed wires",
    note = "Portals and gates together.",
    par = 15,
    map = {
      "#############",
      "#@..A...x..o#",
      "#.####.####.#",
      "#.o..#B#..C.#",
      "#.####.####.#",
      "#C..y...A..E#",
      "#############",
    },
  },
}

-- a level built by a loop: a zigzag of gates that alternate x and y
local zig = { "#############" }
for row = 1, 5 do
  local gate = (row % 2 == 1) and "x" or "y"
  if row % 2 == 1 then
    zig[#zig + 1] = "#.." .. gate .. "....o...#"
  else
    zig[#zig + 1] = "#...o...." .. gate .. "..#"
  end
end
zig[2] = "#@.x....o...#"
zig[#zig + 1] = "#..........E#"
zig[#zig + 1] = "#############"
levels[#levels + 1] = {
  name = "Zigzag",
  note = "Built by a Lua loop. Rows of gates in step and out of step.",
  par = 23,
  map = zig,
}

return levels
