mod data;

use self::data::TABLE;

pub fn get_char(digraph: &str) -> Option<char> {
    let mut chars_it = digraph.chars();
    let chars: [char; 2] = [chars_it.next().unwrap(), chars_it.next().unwrap()];
    let res = TABLE.into_iter().find(|&x| x.sequence == chars);
    match res {
        Some(digr) => Some(digr.character),
        None => None,
    }
}

pub fn get_digraph(c: &char) -> Option<String> {
    let res = TABLE.into_iter().find(|&x| x.character == *c);
    match res {
        Some(digr) => Some(digr.sequence.iter().cloned().collect::<String>()),
        None => None,
    }
}
