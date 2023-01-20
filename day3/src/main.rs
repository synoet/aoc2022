use std::collections::HashSet;

struct Item {
    value: char,
}
impl Item {
    fn from_str(s: char) -> Self {
        Item { value: s }
    }

    fn priority(&self) -> usize {
        let alphabet: Vec<char> = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        let priority: usize = alphabet
            .iter()
            .position(|c| match self.value.is_uppercase() {
                true => c == &self.value.to_lowercase().next().unwrap(),
                false => c == &self.value,
            })
            .expect("to find in alphabet");

        match self.value.is_uppercase() {
            true => priority + 27,
            false => priority + 1,
        }
    }
}

fn main() {
    let input: String = std::fs::read_to_string("./src/input.txt").expect("to read input");
    let lines = input.lines();

    let mut sum: usize = 0;
    for line in lines {
        let container_one: HashSet<char> = line[0..line.len() / 2].chars().collect();
        let container_two: HashSet<char> = line[line.len() / 2..line.len()].chars().collect();

        let intersection: HashSet<char> = container_one
            .intersection(&container_two)
            .cloned()
            .collect();

        sum += intersection
            .iter()
            .map(|c| Item::from_str(*c).priority())
            .sum::<usize>();
    }

    println!("sum: {}", sum);
}
