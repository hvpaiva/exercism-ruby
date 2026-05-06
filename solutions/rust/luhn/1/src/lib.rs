/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0u32;
    let mut double_next = false;
    let mut digits_seen = 0;

    for byte in code.as_bytes().iter().rev() {
        match byte {
            b' ' => continue,
            b'0'..=b'9' => {
                let mut digit = (byte - b'0') as u32;

                if double_next {
                    digit *= 2;
                    if digit > 9 {
                        digit -= 9;
                    }
                }

                sum += digit;
                double_next = !double_next;
                digits_seen += 1;
            }
            _ => return false,
        }
    }

    digits_seen > 1 && sum % 10 == 0
}
