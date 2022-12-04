use std::{fs::read_to_string, path::Path};

#[derive(Debug)]
pub struct Section(u32, u32);

impl Section {
    pub fn from_str(input: &str) -> Self {
        let nums: Vec<u32> = input
            .split('-')
            .map(|part| part.parse::<u32>().unwrap())
            .collect();

        Self(nums[0], nums[1])
    }

    pub fn contains(&self, other: &Section) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    pub fn overlaps(&self, other: &Section) -> bool {
        let range = self.0..=self.1;

        other.contains(self) || range.contains(&other.0) || range.contains(&other.1)
    }
}

fn parse_group(line: &str) -> (Section, Section) {
    let parts = line.split(',').collect::<Vec<&str>>();
    (Section::from_str(parts[0]), Section::from_str(parts[1]))
}

fn count_groups<F>(lines: &Vec<&str>, compare: F) -> u32
where
    F: Fn(&Section, &Section) -> bool,
{
    let mut count: u32 = 0;

    for line in lines {
        let group = parse_group(line);
        if compare(&group.0, &group.1) {
            count += 1;
        }
    }

    count
}

fn main() {
    let contents = read_to_string(&Path::new("./data/input.txt")).unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();

    let part1 = count_groups(&lines, |section1, section2| {
        section1.contains(section2) || section2.contains(section1)
    });
    println!("{}", part1);

    let part2 = count_groups(&lines, |section1, section2| section1.overlaps(section2));
    println!("{}", part2);
}
