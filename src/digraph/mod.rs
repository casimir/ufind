mod data;

use self::data::{Digraph, TABLE};

fn get_char(digraph: &str) -> Option<char> {
    let mut chars_it = digraph.chars();
    let chars: [char; 2] = [chars_it.next().unwrap(), chars_it.next().unwrap()];
    let res = TABLE.into_iter().find(|&x| x.sequence == chars);
    match res {
        Some(digr) => Some(digr.character),
        None => None,
    }
}

fn get_digraph(c: &char) -> Option<String> {
    let res = TABLE.into_iter().find(|&x| x.character == *c);
    match res {
        Some(digr) => Some(digr.sequence.iter().cloned().collect::<String>()),
        None => None,
    }
}

pub fn convert(pattern: &str) -> Option<String> {
    match pattern.chars().count() {
        1usize => get_digraph(&pattern.chars().next().unwrap()),
        2usize => {
            match get_char(pattern) {
                Some(c) => Some(c.to_string()),
                None => None,
            }
        }
        _ => None,
    }
}

pub fn filter(pattern: &str) -> Vec<&Digraph> {
    match pattern.chars().count() {
        0usize => TABLE.iter().collect(),
        1usize => {
            let c = pattern.chars().next().unwrap();
            TABLE.into_iter().filter(|&x| x.sequence[0] == c).collect()
        }
        2usize => {
            let mut chars_it = pattern.chars();
            let chars: [char; 2] = [chars_it.next().unwrap(), chars_it.next().unwrap()];
            if chars[0] == '_' {
                TABLE.into_iter().filter(|&x| x.sequence[1] == chars[1]).collect()
            } else {
                TABLE.into_iter().filter(|&x| x.sequence == chars).collect()
            }
        }
        _ => TABLE.iter().collect(),
    }
}
