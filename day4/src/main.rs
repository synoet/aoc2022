
#[derive(Debug)]
struct Elf {
    bottom_range: i32,
    top_range: i32,
}

impl Elf {
    fn fully_contains(&self, other: &Elf) -> bool {
        self.bottom_range >= other.bottom_range && self.top_range <= other.top_range
    }
    fn overlaps(&self, other: &Elf) -> bool {
        self.top_range >= other.bottom_range && self.bottom_range <= other.top_range
    }
}

trait FromInput {
    fn from_input(&self) -> i32;
}

impl FromInput for &str {
    fn from_input(&self) -> i32 {
        self.parse::<i32>().unwrap_or(0)
    }
}

fn main(){
    let input: String = std::fs::read_to_string("./src/input.txt").expect("to read input");
    let pairs = input.lines();

    let mut elf_pairs: Vec<(Elf, Elf)> = Vec::new();
    for pair in pairs {
        let split_pair = pair.split(",").into_iter().collect::<Vec<&str>>();

        elf_pairs.push((
            Elf {
            bottom_range: split_pair[0].split("-").into_iter().collect::<Vec<&str>>()[0].from_input(),
            top_range: split_pair[0].split("-").into_iter().collect::<Vec<&str>>()[1].from_input(),
            },
            Elf {
                bottom_range: split_pair[1].split("-").into_iter().collect::<Vec<&str>>()[0].from_input(),
                top_range: split_pair[1].split("-").into_iter().collect::<Vec<&str>>()[1].from_input(),
            }
        ))
    }

    let pairs_that_contain = elf_pairs.iter().filter(|(elf1, elf2)| {
        elf1.fully_contains(elf2) || elf2.fully_contains(elf1)
    });

    let pairs_that_overlap = elf_pairs.iter().filter(|(elf1, elf2)| {
        elf1.overlaps(elf2) || elf2.overlaps(elf1)
    });

    println!("{} pairs contain each other", pairs_that_contain.count());
    println!("{} pairs overlap", pairs_that_overlap.count());
}
