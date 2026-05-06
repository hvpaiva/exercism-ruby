pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut out = Vec::with_capacity(garden.len());

    for (row_idx, row) in garden.iter().enumerate() {
        let bytes = row.as_bytes();
        let mut line = String::with_capacity(bytes.len());

        for (col_idx, _) in bytes.iter().enumerate() {
            if bytes[col_idx] == FLOWER {
                line.push('*');
            } else {
                let count = count_adjacent_flowers(garden, row_idx, col_idx);
                line.push(match count {
                    0 => ' ',
                    n => char::from_digit(n as u32, 10).unwrap(),
                });
            }
        }

        out.push(line);
    }

    out
}

const FLOWER: u8 = b'*';
const OFFSETS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_adjacent_flowers(garden: &[&str], row: usize, col: usize) -> u8 {
    let mut n = 0;

    for &(dr, dc) in OFFSETS {
        let r = row as isize + dr;
        let c = col as isize + dc;

        if r < 0 || c < 0 {
            continue;
        }

        if let Some(b) = spot_in_garden(garden, r as usize, c as usize) {
            if b == FLOWER {
                n += 1;
            }
        }
    }

    n
}

fn spot_in_garden(garden: &[&str], row: usize, col: usize) -> Option<u8> {
    garden.get(row)?.as_bytes().get(col).copied()
}
