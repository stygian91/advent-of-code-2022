local utils = require("common.utils")
local rps   = require("q_02.rps")

local lines = utils.read_lines("./q_02/input.txt")

local function create_player(ch)
  if ch == "X" then return rps.Hand.rock
  elseif ch == "Y" then return rps.Hand.paper
  end

  return rps.Hand.scissors
end

local total = 0

for _, line in pairs(lines) do
  local enemy = rps.create_from_enemy(string.sub(line, 1, 1))
  local player = create_player(string.sub(line, 3, 3))
  local outcome = rps.Play(player, enemy)
  local points = rps.calc_points(outcome, player)
  total = total + points
end

print(total)

