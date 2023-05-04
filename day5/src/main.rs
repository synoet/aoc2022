use std::collections::HashMap;

#[derive(Debug)]
struct ShippingCrate {
    value: String,
}

#[derive(Debug)]
struct CrateStack {
    index: usize,
    crates: Vec<ShippingCrate>,
}

impl CrateStack {
    fn push(&mut self, shipping_crate: ShippingCrate) {
        self.crates.push(shipping_crate);
    }
    fn insert(&mut self, shipping_crate: ShippingCrate) {
        self.crates.insert(0, shipping_crate);
    }

    fn pop(&mut self) -> Option<ShippingCrate> {
        self.crates.pop()
    }
}

#[derive(Debug)]
struct Instruction {
    start: usize,
    target: usize,
    number_of_crates: usize,
}

enum CurrentMode {
    Build,
    ParseInstructions,
}

fn main() {
    let input: String = std::fs::read_to_string("./src/input.txt").expect("to read input");
    let lines = input.lines();

    let mut stack_map: HashMap<usize, CrateStack> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut current_mode = CurrentMode::Build;

    for line in lines {
         if line.trim().is_empty() {
            current_mode = CurrentMode::ParseInstructions;
            continue;
        }
        match current_mode {
            CurrentMode::Build => {
                let groups = line
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|c| c.iter().collect::<String>())
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>();


                for (index, group) in groups.iter().enumerate() {
                    match group.parse::<usize>() {
                        Ok(_) => {
                            continue
                        }
                        Err(_) => {
                            if group.trim().is_empty() {
                                continue
                            }
                            let crate_stack = stack_map.entry(index + 1).or_insert(CrateStack {
                                index: index + 1,
                                crates: Vec::new(),
                            });
                            crate_stack.insert(ShippingCrate {
                                value: group.to_string().chars().nth(1).unwrap().to_string(),
                            });
                        }
                    }
                }
            }
            CurrentMode::ParseInstructions => {
                let parsed_instructions = line.split(" ").map(|s| s.to_owned()).collect::<Vec<String>>();
                instructions.push(
                    Instruction {
                        start: parsed_instructions[3].parse::<usize>().unwrap(),
                        target: parsed_instructions[5].parse::<usize>().unwrap(),
                        number_of_crates: parsed_instructions[1].parse::<usize>().unwrap(),
                    }
                );
            }
        }
    }

    for instruction in instructions {
        let mut start_stack: Option<&mut CrateStack> = None;
        let mut end_stack: Option<&mut CrateStack> = None;
        for (index, stack) in stack_map.iter_mut() {
            if *index == instruction.start {
                start_stack = Some(stack);
            }else if *index == instruction.target {
                end_stack = Some(stack);
            }
        }

        match (start_stack, end_stack) {
            (Some(start_stack), Some(end_stack)) => {
                let mut crates_to_move: Vec<ShippingCrate> = Vec::new();
                for _ in 0..instruction.number_of_crates {
                    let crate_to_move = start_stack.pop();
                    if let Some(crate_to_move) = crate_to_move {
                        crates_to_move.insert(0, crate_to_move);
                    }
                }

                for crate_to_move in crates_to_move {
                    end_stack.push(crate_to_move);
                }
            }
            _ => {
                println!("Could not find stack");
            }
        }
    }

    let mut result: String = "".to_string(); 
    for i in 0..stack_map.iter().len() {
        let stack = stack_map.get_mut(&(i + 1)).unwrap();
        result.push_str(&stack.pop().unwrap().value);
    }

    println!("{}", result);
}
