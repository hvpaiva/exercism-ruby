#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;

    match (first_list.len(), second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        _ if first_list == second_list => Equal,
        _ if is_slice(second_list, first_list) => Sublist,
        _ if is_slice(first_list, second_list) => Superlist,
        _ => Unequal,
    }
}

fn is_slice<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
