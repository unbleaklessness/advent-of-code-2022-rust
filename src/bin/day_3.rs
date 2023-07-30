use std::fs::File;
use std::io::{BufReader, Read};

fn character_to_number(c: u8) -> u8 {

    let alphabet_size = 26;

    if c >= 97 && c <= 122 { // From `a` to `z`.
        return c - 97 + 1;
    } 

    if c >= 64 && c <= 90 { // From `A` to `Z`.
        return c - 64 + alphabet_size;
    }

    panic!("Impossible");
}

fn characters_to_numbers(cs: Vec<u8>) -> Vec<u8> {
    cs.into_iter().map(|x| character_to_number(x)).collect::<Vec<_>>()
}

fn first_part() -> Option<()> {
    let file = File::open("src/bin/day_3/input_2.txt").ok()?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).ok()?;

    let lines = contents.split("\n").collect::<Vec<_>>();

    let mut total: i64 = 0;

    for line in lines {

        if line.is_empty() {
            continue;
        }

        if line.len() % 2 != 0 {
            panic!("Impossible");
        }

        let half_index = line.len() / 2;

        let first = Vec::from(&line[0 .. half_index]);
        let second = Vec::from(&line[half_index ..]);

        let first_n = characters_to_numbers(first);
        let second_n = characters_to_numbers(second);

        let mut index = None;
        
        'outer: for (i, x) in first_n.iter().enumerate() {
            for (_, y) in second_n.iter().enumerate() {
                if x == y {
                    index = Some(i);
                    break 'outer;
                }
            }
        }

        total += first_n[index?] as i64;
    }

    println!("Total: {total}");

    Some(())
}

fn main() {
    first_part().unwrap();
}
