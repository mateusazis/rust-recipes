trait Stringer {
    fn make_str(&self) -> String;
}

struct IntStringer {
    value: i32,
}
impl Stringer for IntStringer {
    fn make_str(&self) -> String {
        format!("{}", self.value)
    }
}
struct StrStringer<'a> {
    value: &'a str,
}
impl<'a> Stringer for StrStringer<'a> {
    fn make_str(&self) -> String {
        String::from(self.value)
    }
}

pub fn main() {
    let mut stringers_by_reference = Vec::<&dyn Stringer>::new();
    stringers_by_reference.push(&IntStringer { value: 2 });
    stringers_by_reference.push(&StrStringer { value: "foo" });
    for stringer in stringers_by_reference {
        println!("Str: {}", stringer.make_str());
    }

    let mut stringers_by_pointer = Vec::<Box<dyn Stringer>>::new();
    stringers_by_pointer.push(Box::new(IntStringer { value: 4 }));
    stringers_by_pointer.push(Box::new(StrStringer { value: "bar" }));
    for stringer in stringers_by_pointer {
        println!("Str: {}", stringer.make_str());
    }
}
