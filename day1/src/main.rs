fn part_one() -> u64{
    let input: String = std::fs::read_to_string("./src/input.txt").expect("to read in input");
    let mut max: u64 = 0;

    for group in input.split("\n\n") {
        let mut sum: u64 = 0;
        for line in group.lines() {
            sum = sum +  line.parse::<u64>().expect("to parse string to number")
        }

        if max < sum {
            max = sum
        }
    }

    max
}

fn part_two() -> u64{
    let input: String = std::fs::read_to_string("./src/input.txt").expect("to read in input");

    let mut elves: Vec<u64> = vec![];

    for group in input.split("\n\n") {
        let mut sum: u64 = 0;
        for line in group.lines() {
            sum = sum +  line.parse::<u64>().expect("to parse string to number")
        }

        elves.push(sum)

    }


    elves.sort();

    let mut sum: u64=0;

    for elf in &elves[elves.len() - 3..elves.len()] {
        sum += elf;
    }

    sum

}


fn main() {
    let part_two_response = part_two();
    //let part_one_response = part_one();
    println!("{:?}", part_two_response)
}
