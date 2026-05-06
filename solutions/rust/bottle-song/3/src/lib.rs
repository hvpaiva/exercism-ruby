pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down.min(start_bottles))
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(n: u32) -> String {
    let word = capitalize(num_to_word(n));
    let bottle = pluralize(n);
    let left = num_to_word(n.saturating_sub(1));
    let left_bottle = pluralize(n.saturating_sub(1));

    format!(
        "{word} green {bottle} hanging on the wall,\n\
         {word} green {bottle} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {left} green {left_bottle} hanging on the wall."
    )
}

#[inline]
fn pluralize(amount: u32) -> &'static str {
    if amount == 1 { "bottle" } else { "bottles" }
}

fn capitalize(s: String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn num_to_word(n: u32) -> String {
    match n {
        0 => "no".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20..=99 => {
            let tens = [
                "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty",
                "ninety",
            ];
            let t = tens[(n / 10) as usize];
            if n % 10 == 0 {
                t.to_string()
            } else {
                format!("{}-{}", t, num_to_word(n % 10))
            }
        }
        _ => n.to_string(),
    }
}
