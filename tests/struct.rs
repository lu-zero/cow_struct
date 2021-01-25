#[cfg(test)]
mod tests {
    use cow_struct::CowStruct;
    #[derive(Debug, Default, Clone)]
    struct Nested {
        f: [u8; 16],
    }

    #[derive(Debug, Default, CowStruct)]
    struct A {
        field: [[u8; 10]; 20],
        nested: Nested,
    }

    #[test]
    fn derive() {
        let a = A::default();

        let mut b = a.to_cow();

        b.field.to_mut()[3][3] = 42;

        let mut c = b.to_cow();

        c.field.to_mut()[3][3] = 12;
        c.field.to_mut()[2][2] = 42;

        println!("{:?} {:?} {:?}", a, b, c);

        println!(
            "{} vs {}",
            std::mem::size_of_val(&a),
            std::mem::size_of_val(&c)
        );
    }
}
