local utils = require("common.utils")
local rps   = require("q_02.rps")

local lines = utils.read_lines("./q_02/input.txt")

local function create_outcome(ch)
  if ch == "X" then
    return rps.Outcome.loss
  elseif ch == "Y" then
    return rps.Outcome.draw
  end

  return rps.Outcome.win
end

local function player_from_outcome(enemy, outcome)
  if outcome == rps.Outcome.win then
    if enemy == rps.Hand.rock then return rps.Hand.paper
    elseif enemy == rps.Hand.paper then return rps.Hand.scissors
    end

    return rps.Hand.rock
  elseif outcome == rps.Outcome.loss then
    if enemy == rps.Hand.rock then return rps.Hand.scissors
    elseif enemy == rps.Hand.paper then return rps.Hand.rock
    end

    return rps.Hand.paper
  end

  if enemy == rps.Hand.rock then return rps.Hand.rock
  elseif enemy == rps.Hand.paper then return rps.Hand.paper
  end

  return rps.Hand.scissors
end

local total = 0

for _, line in pairs(lines) do
  local enemy = rps.create_from_enemy(string.sub(line, 1, 1))
  local outcome = create_outcome(string.sub(line, 3, 3))
  local player = player_from_outcome(enemy, outcome)
  local points = rps.calc_points(outcome, player)
  total = total + points
end

print(total)

