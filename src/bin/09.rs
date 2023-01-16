#![allow(dead_code)]
pub fn part_one(program: &str) -> Option<i64> {
    let mut c = Computer::new(program);
    c.input = 1; // Test mode
    c.run();
    c.outputs.pop()
}

pub fn part_two(program: &str) -> Option<i64> {
    let mut c = Computer::new(program);
    c.input = 2; // BOOST mode
    c.run();
    c.outputs.pop()
}

struct Computer {
    memory: Vec<i64>,
    instruction_pointer: usize,
    input: i64,
    outputs: Vec<i64>,
    base: i64,
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
    AdjustBase(Location),                   // Opcode 9
    Halt,                                   // Opcode 99
}

#[derive(Debug, Clone, Copy)]
enum Location {
    Absolute(usize),
    Relative(i64),
    Immediate(i64),
}

impl Computer {
    fn new(input: &str) -> Computer {
        let memory: Vec<i64> = input
            .split(',')
            .map(|e| e.trim().parse::<i64>().unwrap())
            .collect();
        // let memory = Box::new(memory);
        let ip = 0;
        Computer {
            memory,
            instruction_pointer: ip,
            input: 0,
            outputs: vec![],
            base: 0,
        }
    }

    // Run until completion
    fn run(&mut self) -> Option<i64> {
        while let Some(_) = self.next() {
            // println!("{:?}", self.memory)
        }
        Some(self.memory[0])
    }

    // Parse and run the next instruction
    fn next(&mut self) -> Option<()> {
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
            Opcode::AdjustBase(p1) => {
                self.base += self.get(p1);
                self.instruction_pointer += 2;
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
            9 => Opcode::AdjustBase(self.parse_loc(1)),
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
        let mode = (opcode / 10i64.pow((offset as u32 )+1)) /* remove RHS digits */ % 10; /* remove LHS digits */
        match mode {
            0 => Location::Absolute(value as usize),
            1 => Location::Immediate(value),
            2 => Location::Relative(value),
            _ => panic!("can't parse parameter mode"),
        }
    }

    fn get(&self, location: Location) -> i64 {
        match location {
            Location::Absolute(index) => self.memory.get(index).get_or_insert(&0).clone(),
            Location::Relative(index) => self
                .memory
                .get((self.base + index) as usize)
                .get_or_insert(&0)
                .clone(),
            Location::Immediate(value) => value,
        }
    }

    fn set(&mut self, location: Location, val: i64) {
        let index = match location {
            Location::Absolute(index) => index,
            Location::Relative(offset) => (self.base + offset) as usize,
            Location::Immediate(_) => panic!("destination is not a pointer"),
        };
        match self.memory.get_mut(index) {
            Some(mem) => *mem = val,
            None => {
                // Vec is too small, upgrade size
                self.memory.resize(index + 1, 0);
                self.set(location, val)
            }
        };
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let program = String::from("104,1125899906842624,99");
        let mut c = Computer::new(&program);
        c.run();
        assert_eq!(c.outputs.pop(), Some(1125899906842624));

        let program = String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut c = Computer::new(&program);
        c.run();
        assert_eq!(
            c.outputs,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }
}
