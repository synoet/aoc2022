enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_string(input: &str) -> Move {
        match input {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => Move::Paper,
        }
    }

    fn value(&self) -> u64 {
        match &self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn loses_to(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn wins_against(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn value(&self) -> u64 {
        match &self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn from_string(input: &str) -> Outcome {
        match input {
            "Z" => Outcome::Win,
            "Y" => Outcome::Draw,
            "X" => Outcome::Loss,
            _ => Outcome::Draw,
        }
    }
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("to read input");

    let mut points: u64 = 0;

    for line in input.lines() {
        let mut line = line.split_whitespace();
        let opponent_move = Move::from_string(line.next().unwrap());
        let desired_outcome = Outcome::from_string(line.next().unwrap());
        let your_move: Move = match desired_outcome {
            Outcome::Win => opponent_move.loses_to(),
            Outcome::Draw => opponent_move,
            Outcome::Loss => opponent_move.wins_against(),
        };

        points += your_move.value() + desired_outcome.value();
    }

    println!("{}", points);
}
