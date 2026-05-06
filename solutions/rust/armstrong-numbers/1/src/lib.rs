pub fn is_armstrong_number(num: u32) -> bool {
    let numbers = num.to_string().chars().map(to_digit).collect::<Vec<u32>>();
    let num_digits = numbers.len() as u32;

    let res: u32 = numbers.iter().map(|n| n.pow(num_digits)).sum();

    res == num
}

fn to_digit(ch: char) -> u32 {
    ch.to_digit(10).unwrap()
}
