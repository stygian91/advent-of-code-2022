local fh = io.lines("./input.txt")

local top3 = {}
local group = 0

local function update_top3(num)
  if #top3 == 0 then
    top3[1] = num
    return
  end

  for i, x in pairs(top3) do
    if x < num then
      table.insert(top3, i, num)
      if #top3 > 3 then
        table.remove(top3, #top3)
      end
      return
    end
  end
end

for line in fh do
  if line == "" then
    update_top3(group)
    group = 0
  else
    group = group + tonumber(line)
  end
end

local sum = 0
for _, x in pairs(top3) do
  sum = sum + x
end

print(sum)
