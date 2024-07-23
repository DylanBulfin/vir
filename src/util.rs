use std::env;

pub fn find_word_end(line: &str, index: usize) -> usize {
    let word_chars = match env::var("WORDCHARS") {
        Ok(wc) => wc,
        Err(_) => String::from("*?_-.[]~=&;!#$%^(){}<>"),
    };

    match line[index..].split_once(|c| word_chars.contains(c)) {
        Some((w, _)) => index + w.len(),
        None => line.len() - 1,
    }
}

