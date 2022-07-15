#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if inside(_first_list, _second_list) && inside(_second_list, _first_list) {
        Comparison::Equal
    } else if inside(_first_list, _second_list) {
        Comparison::Sublist
    } else if inside(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

pub fn inside<T: PartialEq>(_first: &[T], _second: &[T]) -> bool {
    (0..=_first.len()).into_iter().any(|idx| {
        _first
            .into_iter()
            .eq(_second.into_iter().skip(idx).take(_first.len()))
    })
}
