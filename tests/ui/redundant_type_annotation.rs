// run-rustfix
#![allow(unused)]
#![warn(clippy::redundant_type_annotation)]

struct A;

enum B {
    Variant1,
    Variant2,
}

fn main() {
    // Testing with a standard library type
    let std_string: String = String::new();

    // Testing with a custom struct
    let a_struct: A = A;

    // Testing with an enum
    let b_enum: B = B::Variant1;

    // This one should not raise a warning
    let number: u32 = 5208;
}
