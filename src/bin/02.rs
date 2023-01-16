#![allow(dead_code)]
pub fn part_one(input: &str) -> Option<i32> {
    let mut c = Computer::new(input);
    c.memory[1] = 12;
    c.memory[2] = 2;
    c.run()
}

pub fn part_two(input: &str) -> Option<i32> {
    let goal = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut c = Computer::new(input);
            c.memory[1] = noun;
            c.memory[2] = verb;
            if c.run().unwrap() == goal {
                return Some(noun * 100 + verb);
            }
        }
    }
    None
}

struct Computer {
    memory: Vec<i32>,
    instruction_pointer: usize,
}

enum Opcode {
    Add(Location, Location, Location),      // Opcode 1
    Multiply(Location, Location, Location), // Opcode 2
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
        }
    }

    // Run until completion
    fn run(&mut self) -> Option<i32> {
        while let Some(_) = self.next() {}
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
                self.instruction_pointer += 4
            }
            Opcode::Halt => return None,
        };
        Some(())
    }

    // Parse the current instruction
    fn parse(&self) -> Opcode {
        let opcode = self.get(Location::Absolute(self.instruction_pointer));
        match opcode {
            1 => Opcode::Add(self.parse_loc(1), self.parse_loc(2), self.parse_loc(3)),
            2 => Opcode::Multiply(self.parse_loc(1), self.parse_loc(2), self.parse_loc(3)),
            99 => Opcode::Halt,
            _ => panic!("Could not parse opcode"),
        }
    }

    fn parse_loc(&self, offset: usize) -> Location {
        // TODO: logic here for abs, rel, dir (access at ins pointer)
        let index = Location::Absolute(self.instruction_pointer + offset);
        let value = self.get(index).clone() as usize;
        Location::Absolute(value)
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
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input); // 4090701
    advent_of_code::solve!(2, part_two, input); // 6421
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(3500));
    }

    #[test]
    fn test_other_inputs_part_one() {
        let input = String::from("1,0,0,0,99");
        assert_eq!(part_one(&input), Some(2));
        let input = String::from("2,3,0,3,99");
        assert_eq!(part_one(&input), Some(2));
        let input = String::from("2,4,4,5,99,0");
        assert_eq!(part_one(&input), Some(2));
        let input = String::from("1,1,1,4,99,5,6,0,99");
        assert_eq!(part_one(&input), Some(30));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
