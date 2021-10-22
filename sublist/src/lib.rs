#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == 0 && _second_list.len() == 0 {
        return Comparison::Equal;
    }

    if _first_list.len() > _second_list.len() {
        return sublist_impl(_second_list, _first_list, Comparison::Superlist);
    }

    sublist_impl(_first_list, _second_list, Comparison::Sublist)
}

fn sublist_impl<T: PartialEq>(_first_list: &[T], _second_list: &[T], set_type: Comparison) -> Comparison {
    let len = _second_list.len() - _first_list.len() + 1;
    for i in 0..len {
        let mut found: bool = true;
        let mut first_list_index: usize = 0;

        while first_list_index < _first_list.len() {
            if _first_list[first_list_index] != _second_list[first_list_index+i] {
                found = false;
                break;
            }
            first_list_index += 1;
        }
        if found && _first_list.len() != _second_list.len() {
            return set_type;
        }

        if found {
            return Comparison::Equal;
        }
    }

    Comparison::Unequal
}
