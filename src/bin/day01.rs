fn to_digit_str(value: &str) -> Option<&str> {
    match value {
        value if value.ends_with("zero") => Some("0"),
        value if value.ends_with("one") => Some("1"),
        value if value.ends_with("two") => Some("2"),
        value if value.ends_with("three") => Some("3"),
        value if value.ends_with("four") => Some("4"),
        value if value.ends_with("five") => Some("5"),
        value if value.ends_with("six") => Some("6"),
        value if value.ends_with("seven") => Some("7"),
        value if value.ends_with("eight") => Some("8"),
        value if value.ends_with("nine") => Some("9"),
        _ => None,
    }
}

fn normalize_line(line: &str) -> String {
    let mut result = String::new();
    let mut buffer = String::new();

    // Should go ahead and learn a parser like chumsky or something
    line.chars().for_each(|c| {
        if char::is_digit(c, 10) {
            result.push(c);
            buffer.clear();
            return;
        }

        buffer.push(c);
        if let Some(num) = to_digit_str(buffer.as_str()) {
            result.push_str(num);
            buffer.clear();
        }
    });

    result
}

fn get_num(line: &str) -> u32 {
    // O(n) pass, O(1) mem -- may be composed better with things like
    // filter digits w/ first/last
    let mut have_first = false;
    let mut first = 0;
    let mut second = 0;

    normalize_line(line).chars().for_each(|c| {
        if let Some(dig) = c.to_digit(10) {
            if !have_first {
                first = dig;
                have_first = true;
            }

            second = dig;
        }
    });

    (first * 10) + second
}

fn main() {
    let input = aoc2023::read_input(1);

    let sum = input.lines().map(get_num).sum::<u32>();
    println!("{}\n-=======-", sum);

    let numbers = input.lines().map(get_num).collect::<Vec<u32>>();
    println!("{:?}", numbers);
}
