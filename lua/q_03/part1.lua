local utils = require("common.utils")
local lines = utils.read_lines("./q_03/input.txt")

local function split(str)
  return string.sub(str, 1, #str/2), string.sub(str, #str/2 + 1)
end

local function contains(search, list)
  for _, el in pairs(list) do
    if el == search then
      return true
    end
  end

  return false
end

local function find_common(a, b)
  for _, ch in pairs(a) do
    if contains(ch, b) then
      return ch
    end
  end

  return nil
end

local a_code = string.byte("a")
local z_code = string.byte("z")
local A_code = string.byte("A")

local function calc_points(char)
  local char_code = string.byte(char)

  if char_code >= a_code and char_code <= z_code then
    return char_code - a_code + 1
  end

  return char_code - A_code + 27
end

local function process_line(line)
  local part1, part2 = split(line)
  local chars1, chars2 = utils.getchars(part1), utils.getchars(part2)
  local char = find_common(chars1, chars2)
  return calc_points(char)
end

local total = 0
for _, line in pairs(lines) do
  local points = process_line(line)
  total = total + points
end

print(total)

