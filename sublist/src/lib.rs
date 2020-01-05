#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    if _first_list.is_empty() {
        return Comparison::Sublist;
    }
    if _second_list.is_empty() {
        return Comparison::Superlist;
    }
    if _first_list.len() > _second_list.len() {
        return match sublist(_second_list, _first_list) {
            Comparison::Sublist => Comparison::Superlist,
            Comparison::Superlist => Comparison::Sublist,
            other => other,
        };
    }
    if _second_list
        .windows(_first_list.len())
        .any(|w| w == _first_list)
    {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}
