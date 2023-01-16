#![allow(dead_code)]
pub fn part_one(program: &str) -> Option<i32> {
    let mut c = Computer::new(program);
    c.input = 1; // HVAC
    c.run();
    c.outputs.pop()
}

pub fn part_two(program: &str) -> Option<i32> {
    let mut c = Computer::new(program);
    c.input = 5; // Thermals
    c.run();
    c.outputs.pop()
}

struct Computer {
    memory: Vec<i32>,
    instruction_pointer: usize,
    input: i32,
    outputs: Vec<i32>,
}

#[derive(Debug)]
enum Opcode {
    Add(Location, Location, Location),      // Opcode 1
    Multiply(Location, Location, Location), // Opcode 2
    Input(Location),                        // Opcode 3
    Output(Location),                       // Opcode 4
    JumpIfTrue(Location, Location),         // Opcode 5
    JumpIfFalse(Location, Location),        // Opcode 6
    LessThan(Location, Location, Location), // Opcode 7
    Equals(Location, Location, Location),   // Opcode 8
    Halt,                                   // Opcode 99
}

#[derive(Debug, Clone, Copy)]
enum Location {
    Absolute(usize),
    Relative(usize),
    Immediate(i32),
}

impl Computer {
    fn new(input: &str) -> Computer {
        let memory: Vec<i32> = input
            .split(',')
            .map(|e| e.trim().parse::<i32>().unwrap())
            .collect();
        let ip = 0;
        Computer {
            memory,
            instruction_pointer: ip,
            input: 0,
            outputs: vec![],
        }
    }

    // Run until completion
    fn run(&mut self) -> Option<i32> {
        while let Some(_) = self.next() {
            // println!("{:?}", self.memory)
        }
        Some(self.memory[0])
    }

    // Parse and run the next instruction
    fn next(&mut self) -> Option<()> {
        // let op = self.parse();
        match self.parse() {
            Opcode::Add(p1, p2, p3) => {
                self.set(p3, self.get(p1) + self.get(p2));
                self.instruction_pointer += 4;
            }
            Opcode::Multiply(p1, p2, p3) => {
                self.set(p3, self.get(p1) * self.get(p2));
                self.instruction_pointer += 4;
            }
            Opcode::Input(p1) => {
                self.set(p1, self.input);
                self.instruction_pointer += 2;
            }
            Opcode::Output(p1) => {
                self.outputs.push(self.get(p1));
                self.instruction_pointer += 2;
            }
            Opcode::JumpIfTrue(p1, p2) => {
                if self.get(p1) != 0 {
                    self.instruction_pointer = self.get(p2) as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            Opcode::JumpIfFalse(p1, p2) => {
                if self.get(p1) == 0 {
                    self.instruction_pointer = self.get(p2) as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            Opcode::LessThan(p1, p2, p3) => {
                if self.get(p1) < self.get(p2) {
                    self.set(p3, 1);
                } else {
                    self.set(p3, 0);
                }
                self.instruction_pointer += 4;
            }
            Opcode::Equals(p1, p2, p3) => {
                if self.get(p1) == self.get(p2) {
                    self.set(p3, 1);
                } else {
                    self.set(p3, 0);
                }
                self.instruction_pointer += 4;
            }
            Opcode::Halt => return None,
        };
        Some(())
    }

    // Parse the current instruction
    fn parse(&self) -> Opcode {
        let opcode = self.get(Location::Absolute(self.instruction_pointer));
        match opcode % 100 {
            1 => Opcode::Add(self.parse_loc(1), self.parse_loc(2), self.parse_loc(3)),
            2 => Opcode::Multiply(self.parse_loc(1), self.parse_loc(2), self.parse_loc(3)),
            3 => Opcode::Input(self.parse_loc(1)),
            4 => Opcode::Output(self.parse_loc(1)),
            5 => Opcode::JumpIfTrue(self.parse_loc(1), self.parse_loc(2)),
            6 => Opcode::JumpIfFalse(self.parse_loc(1), self.parse_loc(2)),
            7 => Opcode::LessThan(self.parse_loc(1), self.parse_loc(2), self.parse_loc(3)),
            8 => Opcode::Equals(self.parse_loc(1), self.parse_loc(2), self.parse_loc(3)),
            99 => Opcode::Halt,
            i => panic!("Could not parse opcode {i}"),
        }
    }

    fn parse_loc(&self, offset: usize) -> Location {
        if !(1..10).contains(&offset) {
            panic!("Offset not in range 1..10")
        }
        let index = Location::Absolute(self.instruction_pointer + offset);
        let value = self.get(index).clone();

        let opcode = self.get(Location::Absolute(self.instruction_pointer));
        let mode = (opcode / 10i32.pow((offset as u32 )+1)) /* remove RHS digits */ % 10; /* remove LHS digits */
        match mode {
            0 => Location::Absolute(value as usize),
            1 => Location::Immediate(value),
            _ => panic!("can't parse parameter mode"),
        }
    }

    fn get(&self, location: Location) -> i32 {
        match location {
            Location::Absolute(index) => self.memory.get(index).unwrap().clone(),
            Location::Relative(index) => self
                .memory
                .get(self.instruction_pointer + index)
                .unwrap()
                .clone(),
            Location::Immediate(value) => value,
        }
    }

    fn set(&mut self, location: Location, val: i32) {
        let index = match location {
            Location::Absolute(index) => index,
            Location::Relative(offset) => self.instruction_pointer + offset,
            Location::Immediate(_) => panic!("destination is not a pointer"),
        };
        match self.memory.get_mut(index) {
            Some(mem) => *mem = val,
            None => {
                // Vec is too small, upgrade size
                self.memory.resize(index, 0);
                self.set(location, val)
            }
        };
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input); // 16225258
    advent_of_code::solve!(2, part_two, input); // 2808771
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(1));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
