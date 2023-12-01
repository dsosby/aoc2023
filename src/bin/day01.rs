fn get_num(line: &str) -> u32 {
    // O(n) pass, O(1) mem -- may be composed better with things like
    // filter digits w/ first/last
    let mut have_first = false;
    let mut first = 0;
    let mut second = 0;

    line.chars().for_each(|c| {
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
    println!("{}", sum);
}
