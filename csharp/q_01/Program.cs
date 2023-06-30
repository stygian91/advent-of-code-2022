
Solution.run("./data/input.txt");

class Solution
{
  public static void run(string path)
  {
    string[] lines = File.ReadAllLines(path);
    List<List<string>> groups = new List<List<string>>();
    groups.Add(new List<string>());

    for (int i = 0; i < lines.Length; i++)
    {
      if (lines[i].Length == 0)
      {
        groups.Add(new List<string>());
      }
      else
      {
        groups.Last().Add(lines[i]);
      }
    }

    Int32[] calories = groups
      .Select(group => group.Select(line => Int32.Parse(line)).Sum())
      .OrderDescending()
      .ToArray();

    Console.WriteLine($"Part 1: {calories.First()}");
    Console.WriteLine($"Part 2: {calories.Take(3).Sum()}");
  }
}