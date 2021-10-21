use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    for item in possible_anagrams {
        if item.len() != word.len() {
            continue;
        }

        if item.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let mut letters_map: HashMap<char, i32> = HashMap::new();
        hash_word(&mut letters_map, item);   
        
        let mut found: bool = true;
        for letter in word.to_lowercase().chars() {
            let value = letters_map.get_mut(&letter);
            if value.is_none()  {
                found = false;
                break;
            }

            let val = value.unwrap();
            
            if *val == 0 {
                found = false;
                break;
            }

            *val -= 1;
        }

        if !found {
            continue;
        }

        anagrams.insert(*item);
    }
    anagrams
}

fn hash_word(letters_map: &mut HashMap<char, i32>, item: &str) {
    for letter in item.to_lowercase().chars() {
        let value = letters_map.get_mut(&letter);
        if value.is_some() {
            *value.unwrap() += 1;
        } else {
            letters_map.insert(letter, 1);
        }
    }
}
