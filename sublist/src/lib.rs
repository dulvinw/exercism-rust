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

    if _first_list == [] {
        return Comparison::Sublist;
    }

    if _second_list == [] {
        return Comparison::Superlist;
    }

    if _first_list.len() > _second_list.len() {
        return sublist_impl(_second_list, _first_list, Comparison::Superlist);
    }

    sublist_impl(_first_list, _second_list, Comparison::Sublist)
}

fn sublist_impl<T: PartialEq>(_first_list: &[T], _second_list: &[T], set_type: Comparison) -> Comparison {
    for sublist in _second_list.windows(_first_list.len()) {
        if sublist == _first_list {
            return set_type;
        }
    }

    Comparison::Unequal
}
