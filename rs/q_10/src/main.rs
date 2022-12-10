use std::{fs::read_to_string, path::Path};

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

struct CPU {
    tick: usize,
    x: i32,
    commands: Vec<Command>,
    pc: usize,
    last_started: usize,
}

impl CPU {
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

    pub fn exec(&mut self) {
        let cmd = &self.commands[self.pc];

        match cmd {
            Command::Noop => {
                self.pc += 1;
                self.last_started += 1;
            },
            Command::Add(value) => {
                if self.tick - self.last_started == 2 {
                    self.last_started = self.tick;
                    self.pc += 1;
                    self.x += value;
                }
            }
        };
    }

    pub fn signal_str(&self) -> i32 {
        self.tick as i32 * self.x
    }
}

fn parse_file(path: &str) -> Vec<Command> {
    read_to_string(&Path::new(path))
        .unwrap()
        .lines()
        .map(|line| Command::from_str(line))
        .collect()
}

fn part1(commands: &[Command]) -> i32 {
    let mut cpu = CPU::new(commands);
    let mut next_signal_check = 20;
    let mut total_str = 0;

    while cpu.tick < 220 {
        cpu.next_tick();

        if cpu.tick == next_signal_check {
            next_signal_check += 40;
            total_str += cpu.signal_str();
        }

        cpu.exec();
    }

    total_str
}

fn main() {
    let commands = parse_file("./data/input.txt");
    let part1_res = part1(&commands);
    println!("{:#?}", part1_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_tick_works() {
        let commands = vec![
            Command::Noop,
            Command::Add(3),
            Command::Add(-5),
        ];
        let mut cpu = CPU::new(&commands);

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
}
