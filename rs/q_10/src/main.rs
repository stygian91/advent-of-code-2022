use std::{fs::read_to_string, path::Path, fmt::Write};

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;
const CRT_LIT: char = '#';
const CRT_UNLIT: char = '.';

#[derive(Debug, PartialEq, Clone)]
enum Command {
    Noop,
    Add(i32),
}

impl Command {
    pub fn from_str(input: &str) -> Self {
        if input == "noop" {
            Command::Noop
        } else {
            let value = input.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
            Command::Add(value)
        }
    }
}

struct Cpu {
    tick: usize,
    x: i32,
    commands: Vec<Command>,
    pc: usize,
    last_started: usize,
}

impl Cpu {
    pub fn new(commands: &[Command]) -> Self {
        Self {
            tick: 0,
            x: 1,
            commands: commands.to_vec(),
            pc: 0,
            last_started: 0,
        }
    }

    pub fn next_tick(&mut self) {
        self.tick += 1;
    }

    pub fn exec(&mut self) -> Option<()> {
        let cmd = self.commands.get(self.pc);
        cmd?;

        let cmd = cmd.unwrap();

        match cmd {
            Command::Noop => {
                self.pc += 1;
                self.last_started += 1;
            }
            Command::Add(value) => {
                if self.tick - self.last_started == 2 {
                    self.last_started = self.tick;
                    self.pc += 1;
                    self.x += value;
                }
            }
        };

        Some(())
    }

    pub fn signal_str(&self) -> i32 {
        self.tick as i32 * self.x
    }
}

struct Crt {
    buffer: Vec<String>,
}

impl Crt {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(CRT_HEIGHT),
        }
    }

    pub fn add_pixel(&mut self, is_lit: bool) {
        if self.buffer.is_empty() {
            self.new_row();
        }

        let mut last = self.buffer.last_mut().unwrap();
        if last.len() == CRT_WIDTH {
            self.new_row();
            last = self.buffer.last_mut().unwrap();
        }

        match is_lit {
            true => last.push(CRT_LIT),
            false => last.push(CRT_UNLIT),
        };
    }

    fn new_row(&mut self) {
        self.buffer.push(String::with_capacity(CRT_WIDTH));
    }

    pub fn get_current_idx(&self) -> usize {
        if self.buffer.is_empty() {
            return 0;
        }

        let row = self.buffer.last().unwrap();
        if row.len() == CRT_WIDTH {
            0
        } else {
            row.len()
        }
    }

    pub fn print(&self) -> String {
        let mut res = String::new();

        for line in self.buffer.iter() {
            writeln!(res, "{}", line).unwrap();
        }

        res
    }
}

fn parse_file(path: &str) -> Vec<Command> {
    read_to_string(Path::new(path))
        .unwrap()
        .lines()
        .map(Command::from_str)
        .collect()
}

fn part1(commands: &[Command]) -> i32 {
    let mut cpu = Cpu::new(commands);
    let mut next_signal_check = 20;
    let mut total_str = 0;

    while cpu.tick < 220 {
        cpu.next_tick();

        if cpu.tick == next_signal_check {
            next_signal_check += 40;
            total_str += cpu.signal_str();
        }

        cpu.exec().unwrap();
    }

    total_str
}

fn part2(commands: &[Command]) -> String {
    let mut cpu = Cpu::new(commands);
    let mut crt = Crt::new();

    loop {
        cpu.next_tick();

        let draw_idx = crt.get_current_idx() as i32;
        let is_lit = draw_idx >= cpu.x - 1 && draw_idx <= cpu.x + 1;

        if cpu.exec().is_none() {
            break;
        }

        crt.add_pixel(is_lit);
    }

    crt.print()
}

fn main() {
    let commands = parse_file("./data/input.txt");
    let part1_res = part1(&commands);
    let part2_res = part2(&commands);
    println!("part 1: {:#?}", part1_res);
    println!("{}", "-".repeat(20));
    println!("part 2:");
    print!("{}", part2_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_tick_works() {
        let commands = vec![Command::Noop, Command::Add(3), Command::Add(-5)];
        let mut cpu = Cpu::new(&commands);

        cpu.next_tick();
        cpu.exec();
        assert_eq!(cpu.tick, 1);
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.pc, 1);

        cpu.next_tick();
        cpu.exec();
        assert_eq!(cpu.tick, 2);
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.pc, 1);

        cpu.next_tick();
        cpu.exec();
        assert_eq!(cpu.tick, 3);
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.pc, 2);

        cpu.next_tick();
        cpu.exec();
        assert_eq!(cpu.tick, 4);
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.pc, 2);

        cpu.next_tick();
        cpu.exec();
        assert_eq!(cpu.tick, 5);
        assert_eq!(cpu.x, -1);
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn command_from_str_works() {
        assert_eq!(Command::from_str("noop"), Command::Noop);
        assert_eq!(Command::from_str("addx 15"), Command::Add(15));
        assert_eq!(Command::from_str("addx -11"), Command::Add(-11));
    }

    #[test]
    fn part1_works() {
        let commands = parse_file("./data/demo.txt");
        let res = part1(&commands);
        assert_eq!(res, 13140);
    }

    #[test]
    fn part2_works() {
        let commands = parse_file("./data/demo.txt");
        let res = part2(&commands);
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(res, expected);
    }
}
