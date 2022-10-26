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
}
