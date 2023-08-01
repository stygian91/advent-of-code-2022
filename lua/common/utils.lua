local function p(tbl, tab)
  if tab == nil then
    tab = 0
  end

  for i, x in pairs(tbl) do
    if type(x) == "table" then
  print(string.rep(" ", tab)..tostring(i))
      p(x, tab + 2)
    else
      print(string.rep(" ", tab)..tostring(i).." "..tostring(x))
    end
  end
end

local function read_lines(path)
  local lines = {}
  local i = 1

  for line in io.lines(path) do
    lines[i] = line
    i = i + 1
  end

  return lines
end

local function getchars(str)
  local chars = {}
  local i = 1

  for ch in str:gmatch"." do
    chars[i] = ch
    i = i + 1
  end

  return chars
end

return {
  p = p,
  read_lines = read_lines,
  getchars = getchars,
}
