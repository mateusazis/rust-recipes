use std::fmt::Debug;

trait Iterator<'a, T> {

    fn next(&mut self) -> Option<T>;
}

struct Range<'a, T>(&'a[&'a T]);

// With generics, traits can be implemented multiple times for the smae struct
impl <'a> Iterator<'a, i32> for Range<'a, i32> {

    fn next(&mut self) -> Option<i32> {
        if self.0.is_empty() {
            None
        } else {
            let ret = self.0[0];
            self.0 = &self.0[1..self.0.len()];
            Some(*ret)
        }
    }
}

// With generics, traits can be implemented multiple times for the smae struct
impl <'a> Iterator<'a, String> for Range<'a, String> {

    fn next(&mut self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            let ret = self.0[0];
            self.0 = &self.0[1..self.0.len()];
            
            let mut inverted = String::new();
            for c in ret.chars().rev() {
                inverted.push(c);
            }
            Some(inverted)
        }
    }
}

fn iterate<'a, T>(mut it : impl Iterator<'a, T>) where T: Debug {
    while let Some(v) = it.next() {
        println!("Value: {:?}", v);
    }
}

pub fn main() {
    let range = Range(&[&15, &19, &22]);
    iterate(range);

    let b = [&String::from("abacaxi"), &String::from("morango"), &String::from("uva")];
    let words = Range(&b[0..b.len()]);
    iterate( words);
}