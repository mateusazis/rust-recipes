trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Range {
    start : i32,
    end: i32,
    current_pos : i32
}

impl Range {
    fn new(start: i32, end : i32) -> Range {
        Range{start, end, current_pos: start}
    }
}

// Iterator is implemented at most once for each struct
impl Iterator for Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos < self.end {
            let v = self.current_pos;
            self.current_pos += 1;
            Some(v)
        } else {
            None
        }
    }
}

struct Words<'a>(&'a [&'a str]);

impl <'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            let ret = self.0[0];
            self.0 = &self.0[1..self.0.len()];
            Some(ret)
        }
    }
}

pub fn main() {
    let mut range = Range::new(15, 23);
    while let Some(v) = range.next() {
        println!("Value: {}", v);
    }

    let mut words = Words(&["abacaxi", "morango", "uva"]);
    while let Some(v) = words.next() {
        println!("Word: {}", v);
    }
}