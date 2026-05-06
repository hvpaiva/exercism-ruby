#![allow(dead_code)]

use capitalize::Capitalize;
use num2words::Num2Words;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down.min(start_bottles))
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(n: u32) -> String {
    let word = num_to_word(n).capitalize();
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

#[inline]
fn num_to_word(n: u32) -> String {
    if n == 0 {
        "no".into()
    } else {
        Num2Words::new(n).to_words().unwrap()
    }
}
