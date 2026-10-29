-- Trace levels. Each level is a table with a name, a note, a wire budget and a map.
--   #  wall          S  source        N  node
--   .  open floor    +  via: one wire may cross another straight over it
--   ^ v < >          a node that only takes wire arriving in that direction
-- A file may return one level or a list. Maps may be built with Lua, as the last one is.

local levels = {
  {
    name = "First light",
    note = "Run wire from the source to the node.",
    budget = 8,
    map = {
      "#########",
      "#S.....N#",
      "#########",
    },
  },
  {
    name = "Branch",
    note = "Tab moves the pen to a lit node so a wire can branch from it.",
    budget = 16,
    map = {
      "#########",
      "#S...N..#",
      "#.......#",
      "#...N..N#",
      "#########",
    },
  },
  {
    name = "Round the rack",
    note = "Walls block wire. Plan the long way first.",
    budget = 24,
    map = {
      "###########",
      "#S..#....N#",
      "#...#.##..#",
      "#.N.#..#..#",
      "#......#.N#",
      "###########",
    },
  },
  {
    name = "Via",
    note = "A via lets one wire cross another in a straight line.",
    budget = 19,
    map = {
      "###########",
      "#....N....#",
      "#.###.###.#",
      "#S...+...N#",
      "#####.#####",
      "#####v#####",
      "###########",
    },
  },
  {
    name = "One way",
    note = "Arrow nodes take wire only when it arrives their way.",
    budget = 24,
    map = {
      "###########",
      "#S.......>#",
      "#.#######.#",
      "#.#.....#.#",
      "#.#.^...#.#",
      "#.......<.#",
      "###########",
    },
  },
  {
    name = "Junctions",
    note = "Every lit node is a junction. Branch where it saves wire.",
    budget = 29,
    map = {
      "#############",
      "#S...N....N.#",
      "#....#....#.#",
      "#.N..#.N..#N#",
      "#....#....#.#",
      "#############",
    },
  },
  {
    name = "Two vias",
    note = "Two crossings. Lay the straight runs first.",
    budget = 26,
    map = {
      "#############",
      "#.....N.....#",
      "#.###.#.###.#",
      "#S..+...+..N#",
      "#.###.#.###.#",
      "#..N..#..N..#",
      "#############",
    },
  },
  {
    name = "Long way round",
    note = "Arrows and a via together. Count the wire before you lay it.",
    budget = 31,
    map = {
      "###############",
      "#S...N...#....#",
      "#.#####.##.##.#",
      "#...N..+..N...#",
      "#.#####.#####.#",
      "#......N....<.#",
      "###############",
    },
  },
}

-- a level built by a loop: a ladder of rungs, a node at the end of each
local ladder = { "#############" }
for rung = 1, 4 do
  if rung % 2 == 1 then
    ladder[#ladder + 1] = "#.........N.#"
  else
    ladder[#ladder + 1] = "#.N.........#"
  end
end
ladder[2] = "#S........N.#"
ladder[#ladder + 1] = "#############"
levels[#levels + 1] = {
  name = "Ladder",
  note = "Built by a Lua loop. Four rungs, four nodes, one source.",
  budget = 23,
  map = ladder,
}

return levels
