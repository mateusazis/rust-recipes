trait Consumable<'a> {
    fn consume(self) -> String;
}

impl<'a, const N: usize> Consumable<'a> for [u8; N] {
    fn consume(self) -> String {
        format!("Array 'consumed', last: {}", self[N - 1])
    }
}

impl<'a> Consumable<'a> for &[u8] {
    fn consume(self) -> String {
        format!("Slice 'consumed', last: {}", self[self.len() - 1])
    }
}

fn consume_array<const T: usize>(arr: [u8; T]) {
    println!("{}", arr.consume());
}

fn consume_slice(s: &[u8]) {
    println!("{}", s.consume());
}

pub fn main() {
    let vec = [1u8, 2u8, 4u8, 8u8];
    let middle = &vec[1..3];
    let times_three = middle.into_iter().map(|v| v * 3).collect::<Vec<u8>>();

    assert_eq!(middle, &[2u8, 4u8]);
    assert_eq!(times_three.as_slice(), &[6u8, 12u8]);

    consume_array(vec);
    consume_slice(middle);
    consume_slice(&times_three);
    let vec_5 = vec.map(|v| v * 5);
    consume_array(vec_5);
    // Doesn't make sense to consume an array or a slice; they are in the stack. No data on the heap whose ownership to transfer.
    assert_eq!(vec, [1u8, 2u8, 4u8, 8u8]);
    assert_eq!(vec_5, [5u8, 10u8, 20u8, 40u8]);
    assert_eq!(middle, [2u8, 4u8]);
    assert_eq!(times_three, [6u8, 12u8]);
}
