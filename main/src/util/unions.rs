struct Holder<'a> {
    inner: BasicHolder<'a>,
    holder_type: HolderType,
}

union BasicHolder<'a> {
    integer: i32,
    floating_point: f32,
    message: &'a str,
}

enum HolderType {
    INT,
    FLOAT,
    STR,
}

impl<'a> std::fmt::Display for Holder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self.inner {
                // This is dangerous; this value could have been matched by mistake when
                // setting the other fields.
                BasicHolder { integer: 42 } => {
                    return write!(f, "Hit the secret!");
                }
                _ => {}
            }

            match self.holder_type {
                HolderType::INT => {
                    println!("Holder-of-int:{}", self.inner.integer)
                }
                HolderType::FLOAT => println!("Holder-of-float:{}", self.inner.floating_point),
                _ => println!("Holder-of-str:{}", self.inner.message),
            }
        };
        // Err(std::fmt::Error)
        write!(f, "")
    }
}

union Holder2 {
    integer: u32,
    long: u64,
    data: [u8; 8],
}

fn handle(holder: &Holder2) {
    unsafe {
        match holder {
            &Holder2 {
                data:
                    [
                        0x41u8,
                        0x41u8,
                        0x41u8,
                        0x41u8,
                        0x41u8,
                        0x41u8,
                        0x41u8,
                        0x41u8,
                    ],
            } => {
                println!("Data case");
            }
            &Holder2 {
                integer: 0x41414141u32,
            } => {
                println!("Integer case: {:x}", holder.integer);
            }
            _ => {
                println!("No case; bytes: {:?}", holder.data);
            }
        };
    }
}

pub fn main() {
    let mut h = Holder {
        inner: BasicHolder { integer: 10 },
        holder_type: HolderType::INT,
    };
    println!("Holder: {}", h);
    h.inner.integer = 42;
    println!("Holder: {}", h);
    h.inner.floating_point = 42.3;
    h.holder_type = HolderType::FLOAT;
    println!("Holder: {}", h);
    h.inner.message = "hello world";
    h.holder_type = HolderType::STR;
    println!("Holder: {}", h);

    let mut h2 = Holder2 {
        integer: 0x41414141u32,
    };
    // h2.integer = 256*256;
    h2.long = 0x4141414141414141u64;
    handle(&h2);
    unsafe {
        h2.data.copy_from_slice("AAAAAAAA".as_bytes());
    }
    handle(&h2);
}
