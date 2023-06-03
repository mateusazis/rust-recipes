pub fn main() {
    struct S1 {
        a: i32,
        b: i8,
    }
    // let s = S1 { a: 0, b: 0 };
    assert_eq!(8, std::mem::size_of::<S1>());
    assert_eq!(8 * 100, std::mem::size_of::<[S1; 100]>());

    #[repr(packed)]
    struct S2 {
        a: i32,
        b: i8,
    }
    assert_eq!(5, std::mem::size_of::<S2>());
    assert_eq!(5 * 100, std::mem::size_of::<[S2; 100]>());

    #[repr(packed)]
    struct S3 {
        a: i32,
        b: i8,
        c: i16,
    }
    assert_eq!(7, std::mem::size_of::<S3>());
    assert_eq!(7 * 100, std::mem::size_of::<[S3; 100]>());
    println!("done");
}
