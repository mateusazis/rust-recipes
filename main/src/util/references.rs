use std::vec::Vec;

fn break_words(s: &str) -> Vec<&str> {
    let mut ret = Vec::<&str>::new();
    let mut start = 0usize;
    let mut end = 1usize;
    let mut chars = s.chars();
    while start < s.len() && end < s.len() {
        let c = chars.next().unwrap();
        if c.is_alphanumeric() {
            end += 1;
            continue;
        }
        if start == end - 1 {
            start += 1;
            end += 1;
            continue;
        }
        ret.push(&s[start..end - 1]);
        start = end;
        end = start + 1;
    }
    if start < end {
        ret.push(&s[start..end]);
    }
    ret
}

pub fn main() {
    let sentence = "Hello world, this is rust!";
    let words = break_words(sentence);
    println!("Words: {:?}", words);

    assert_eq!(words, vec!["Hello", "world", "this", "is", "rust"]);
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
