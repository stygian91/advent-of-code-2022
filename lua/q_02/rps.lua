local Hand = {
  rock     = 1,
  paper    = 2,
  scissors = 3,
}

local Outcome = {
  win  = 1,
  loss = 2,
  draw = 3,
}

setmetatable(Hand, {
  asd = function ()
    print("here")
  end
})

local function Play(a, b)
  if (a == Hand.rock and b == Hand.rock) or (a == Hand.scissors and b == Hand.scissors) or (a == Hand.paper and b == Hand.paper) then
    return Outcome.draw
  elseif (a == Hand.rock and b == Hand.scissors) or (a == Hand.paper and b == Hand.rock) or (a == Hand.scissors and b == Hand.paper) then
    return Outcome.win
  elseif (a == Hand.scissors and b == Hand.rock) or (a == Hand.rock and b == Hand.paper) or (a == Hand.paper and b == Hand.scissors) then
    return Outcome.loss
  end
end

local function create_from_enemy(ch)
  if ch == "A" then return Hand.rock
  elseif ch == "B" then return Hand.paper
  else return Hand.scissors
  end
end

local function calc_points(outcome, player)
  local points = 0

  if outcome == Outcome.win then points = 6
  elseif outcome == Outcome.draw then points = 3
  end

  if player == Hand.rock then points = points + 1
  elseif player == Hand.paper then points = points + 2
  else points = points + 3
  end

  return points
end

return {
  Hand              = Hand,
  Outcome           = Outcome,
  create_from_enemy = create_from_enemy,
  Play              = Play,
  calc_points       = calc_points,
}
