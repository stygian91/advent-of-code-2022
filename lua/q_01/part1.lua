local fh = io.lines("./input.txt")

local group = 0
local max = 0

for line in fh do
  if line == "" then
    if max < group then
      max = group
    end
    group = 0
  else
    group = group + tonumber(line)
  end
end

print(max)

