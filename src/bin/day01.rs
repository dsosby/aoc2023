fn find_first_digit(line: &str) -> Option<u32> {
    let mut buffer = String::new();

    for c in line.chars() {
        if let Some(num) = char::to_digit(c, 10) {
            return Some(num);
        }

        buffer.push(c);
        match buffer {
            b if b.ends_with("zero") => return Some(0),
            b if b.ends_with("one") => return Some(1),
            b if b.ends_with("two") => return Some(2),
            b if b.ends_with("three") => return Some(3),
            b if b.ends_with("four") => return Some(4),
            b if b.ends_with("five") => return Some(5),
            b if b.ends_with("six") => return Some(6),
            b if b.ends_with("seven") => return Some(7),
            b if b.ends_with("eight") => return Some(8),
            b if b.ends_with("nine") => return Some(9),
            _ => continue,
        }
    };

    None
}

fn find_last_digit(line: &str) -> Option<u32> {
    let mut buffer = String::new();

    for c in line.chars().rev() {
        if let Some(num) = char::to_digit(c, 10) {
            return Some(num);
        }

        buffer.push(c);
        match buffer {
            b if b.ends_with("orez") => return Some(0),
            b if b.ends_with("eno") => return Some(1),
            b if b.ends_with("owt") => return Some(2),
            b if b.ends_with("eerht") => return Some(3),
            b if b.ends_with("ruof") => return Some(4),
            b if b.ends_with("evif") => return Some(5),
            b if b.ends_with("xis") => return Some(6),
            b if b.ends_with("neves") => return Some(7),
            b if b.ends_with("thgie") => return Some(8),
            b if b.ends_with("enin") => return Some(9),
            _ => continue,
        }
    };

    None
}

fn get_num(line: &str) -> u32 {
    let first_digit = find_first_digit(line);
    let second_digit = find_last_digit(line);

    match (first_digit, second_digit) {
        (Some(f), Some(l)) => (f * 10) + l,
        _ => unreachable!("Didnt find digit"),
    }
}

fn main() {
    let input = aoc2023::read_input(1);

    let values = input.lines().map(|line| (line, get_num(line))).collect::<Vec::<(&str, u32)>>();
    let sum = values.iter().map(|(_, val)| val).sum::<u32>();

    println!("{}\n ======= ", sum);
    // for value in values {
    //     println!("{:50} - {}", value.0, value.1);
    // }
}
