use std::fs::read_to_string;
use std::convert::TryFrom;

#[derive(Debug,Clone, Copy)]
struct Instruction{
    run_count:u32,
    arg: i32,
    opp:Opp
}

impl Instruction{
    fn increment_run(&mut self){
        self.run_count+=1;
    }
}

impl TryFrom<&str> for Instruction{
    type Error = &'static str;

    fn try_from(value: &str)-> Result<Self,Self::Error>{
        let mut parts = value.split(" ");
        let opp = parts.next().unwrap();
        let arg: i32= parts.next().unwrap().parse().unwrap();

        let opp = match opp{
            "acc"=>Ok(Opp::Acc),
            "jmp"=>Ok(Opp::Jum),
            "nop"=>Ok(Opp::Nop),
            _=> Err("Instruction is not supported")
        }?;

        Ok(Self{
            run_count:0,
            arg,
            opp
        })
    }
}


#[derive(Debug)]
struct Computer{
    instructions: Vec<Instruction>,
    accumulator: i32,
    current_instruction: isize
}
enum ExecutionHaltReason{
    InfiniteLoop(i32),
    Success(i32)
}
impl Computer{
    fn run(&mut self) -> ExecutionHaltReason{
        loop{
            if self.current_instruction<0 || self.current_instruction as usize >= self.instructions.len()-1{
                return ExecutionHaltReason::Success(self.accumulator)
            }

            let inst  = self.instructions.get_mut(self.current_instruction as usize).unwrap();
            if inst.run_count == 1{
                return ExecutionHaltReason::InfiniteLoop(self.accumulator)
            }

            match inst.opp{
                Opp::Nop => {self.current_instruction +=1},
                Opp::Acc => {self.current_instruction +=1; self.accumulator += inst.arg},
                Opp::Jum => {self.current_instruction = self.current_instruction + inst.arg as isize} 
            }
            inst.increment_run();
        }
    }
}
impl TryFrom<&str> for Computer{
    type Error = &'static str;

    fn try_from(value: &str)-> Result<Self,Self::Error>{
        let instructions: Vec<Instruction> = value.lines().map(|line| Instruction::try_from(line).unwrap()).collect();
        Ok(Computer{
            instructions,
            accumulator:0,
            current_instruction:0,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Opp{
    Nop,
    Acc,
    Jum
}

fn load_computer_from_str(value: &str)->Computer{
    Computer::try_from(value).unwrap()
} 

fn load_computer_from_file (path: &str)-> Computer{
    let instructions = read_to_string(path).unwrap();
    load_computer_from_str(&instructions)
}

fn find_comp_bug(comp: &Computer)-> Option<i32>{
    for (inst_loc, instruction) in comp.instructions.iter().enumerate(){
        let mut new_instructions = comp.instructions.clone();

        new_instructions[inst_loc] = match instruction.opp{
            Opp::Jum => Instruction{opp:Opp::Nop, arg: instruction.arg,run_count:0 },
            Opp::Nop=> Instruction{opp:Opp::Jum, arg: instruction.arg,run_count:0 },
            _ => Instruction{ opp: instruction.opp, arg: instruction.arg, run_count:0}
        };
        let mut test_comp=Computer{
            instructions:new_instructions,
            accumulator:0,
            current_instruction:0,
        };

        if let ExecutionHaltReason::Success(result) =  test_comp.run(){
            return Some(result)
        }

    }
    None
}
fn main() {
    let test="nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let mut test_comp = Computer::try_from(test).unwrap();
    let mut comp = load_computer_from_file("data/day_8/input");

    let result = comp.run();

    
    match result{
        ExecutionHaltReason::InfiniteLoop(result)=> println!("Program excited because of infinite loop detection value was {}",result),
        ExecutionHaltReason::Success(result)=> println!("Program finished {}",result)
    }


    println!("trying to find fix");

    let mut comp = load_computer_from_file("data/day_8/input");
    let res = find_comp_bug(&comp);
    println!("Result after fix {:?}", res);
}
