pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down.min(start_bottles))
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(n: u32) -> String {
    let number = say_number(n);
    let bottle = say_bottle(n);
    let left = say_number(n - 1).to_lowercase();
    let left_bottle = say_bottle(n - 1);

    format!(
        "{number} green {bottle} hanging on the wall,\n\
         {number} green {bottle} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {left} green {left_bottle} hanging on the wall."
    )
}

#[inline]
fn say_bottle(amount: u32) -> &'static str {
    if amount == 1 { "bottle" } else { "bottles" }
}

fn say_number(n: u32) -> String {
    match n {
        0 => "No",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => unreachable!(),
    }
    .to_owned()
}
