use std::vec::Vec;

fn str_as_ptr_u64(s: &str) -> u64 {
    s.as_bytes().as_ptr() as u64
}

fn break_words(s: &str) -> Vec<&str> {
    let mut ret = Vec::<&str>::new();
    let mut start = 0usize;

    for (i, c) in s.char_indices() {
        if c.is_alphanumeric() {
            continue;
        }
        if start != i {
            ret.push(&s[start..i]);
        }
        start = i + 1;
    }
    if start < s.len() {
        ret.push(&s[start..s.len()]);
    }
    ret
}

pub fn main() {
    let sentence = "Hello world, this is rust!";
    let words = break_words(sentence);
    println!("Words: {:?}", words);

    assert_eq!(words, vec!["Hello", "world", "this", "is", "rust"]);

    let sentence_addr = str_as_ptr_u64(sentence);
    let all_addresses = words
        .into_iter()
        .map(|word| format!("0x{:x} ({})", str_as_ptr_u64(word), word))
        .reduce(|mut a, b| {
            a.push_str("\n");
            a.push_str(b.as_str());
            a
        })
        .unwrap();

    println!(
        "Sentence starts at:\n0x{:x}\nand the others at:\n{}",
        sentence_addr, all_addresses
    );
}

#[cfg(test)]
mod test {
    use crate::util::references::break_words;

    #[test]
    fn test_break_words() {
        assert_eq!(
            break_words("Hello world, this is rust!"),
            vec!["Hello", "world", "this", "is", "rust"]
        );
        assert_eq!(break_words("one"), vec!["one"]);
        assert_eq!(break_words(" with padding  "), vec!["with", "padding"]);
        assert_eq!(break_words(""), Vec::<&str>::new());
    }
}
