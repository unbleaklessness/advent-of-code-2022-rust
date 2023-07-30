use std::fs::File;
use std::io::{BufReader, Read};

fn first_half() -> Option<()> {

    let file = File::open("src/bin/day_2/input_2.txt").ok()?;
    let mut reader = BufReader::new(file);
    let mut contents: String = String::new();
    reader.read_to_string(&mut contents).ok()?;

    let lines = contents.split("\n").collect::<Vec<_>>();

    let mut total_score = 0;

    for line in lines {

        if line.is_empty() {
            continue;
        }

        let first = line.chars().nth(0)?;
        let second = line.chars().nth(2)?;

        let win_score = 6;
        let draw_score = 3;
        let lose_score = 0;

        let rock_score = 1;
        let paper_score = 2;
        let scissors_score = 3;

        let mut score = 0;
        
        // A - rock, B - paper, C - scissors.
        // X - rock, Y - paper, Z - scissors.

        // Rock beats scissors.
        // Scissors beats paper.
        // Paper beats rock.
        
        const ROCK_A: char = 'A';
        const PAPER_A: char = 'B';
        const SCISSORS_A: char = 'C';

        const ROCK_B: char = 'X';
        const PAPER_B: char = 'Y';
        const SCISSORS_B: char = 'Z';

        match second {
            ROCK_B => score += rock_score,
            PAPER_B => score += paper_score,
            SCISSORS_B => score += scissors_score,
            _ => panic!("Impossible"),
        }

        match first {
            ROCK_A => {
                match second {
                    ROCK_B => score += draw_score,
                    PAPER_B => score += win_score,
                    SCISSORS_B => score += lose_score,
                    _ => panic!("Impossible"),
                }
            },
            PAPER_A => {
                match second {
                    ROCK_B => score += lose_score,
                    PAPER_B => score += draw_score,
                    SCISSORS_B => score += win_score,
                    _ => panic!("Impossible"),
                }
            },
            SCISSORS_A => {
                match second {
                    ROCK_B => score += win_score,
                    PAPER_B => score += lose_score,
                    SCISSORS_B => score += draw_score,
                    _ => panic!("Impossible"),
                }
            },
            _ => panic!("Impossible"),
        }

        total_score += score;
    }

    println!("Total score: {total_score} (first half)");

    Some(())
}

fn second_half() -> Option<()> {

    let file = File::open("src/bin/day_2/input_2.txt").ok()?;
    let mut reader = BufReader::new(file);
    let mut contents: String = String::new();
    reader.read_to_string(&mut contents).ok()?;

    let lines = contents.split("\n").collect::<Vec<_>>();

    let mut total_score = 0;

    for line in lines {

        if line.is_empty() {
            continue;
        }

        let first = line.chars().nth(0)?;
        let second = line.chars().nth(2)?;

        let win_score = 6;
        let draw_score = 3;
        let lose_score = 0;

        let rock_score = 1;
        let paper_score = 2;
        let scissors_score = 3;

        let mut score = 0;
        
        // A - rock, B - paper, C - scissors.
        // X - rock, Y - paper, Z - scissors.

        // Rock beats scissors.
        // Scissors beats paper.
        // Paper beats rock.
        
        const ROCK_A: char = 'A';
        const PAPER_A: char = 'B';
        const SCISSORS_A: char = 'C';

        const LOSE_B: char = 'X';
        const DRAW_B: char = 'Y';
        const WIN_B: char = 'Z';

        match second {
            LOSE_B => score += lose_score,
            DRAW_B => score += draw_score,
            WIN_B => score += win_score,
            _ => panic!("Impossible"),
        }

        match first {
            ROCK_A => {
                match second {
                    LOSE_B => score += scissors_score,
                    DRAW_B => score += rock_score,
                    WIN_B => score += paper_score,
                    _ => panic!("Impossible"),
                }
            },
            PAPER_A => {
                match second {
                    LOSE_B => score += rock_score,
                    DRAW_B => score += paper_score,
                    WIN_B => score += scissors_score,
                    _ => panic!("Impossible"),
                }
            },
            SCISSORS_A => {
                match second {
                    LOSE_B => score += paper_score,
                    DRAW_B => score += scissors_score,
                    WIN_B => score += rock_score,
                    _ => panic!("Impossible"),
                }
            },
            _ => panic!("Impossible"),
        }

        total_score += score;
    }

    println!("Total score: {total_score} (second half)");

    Some(())
}

fn main() {
    first_half().unwrap();
    second_half().unwrap();
}
