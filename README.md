# derive CowStruct

This crate consists in a procedural macro derive that provides a struct that is Cow and the impl to create one from the target struct.

## Usage

``` rust
    use cow_struct::CowStruct;

    #[derive(Debug, Default, CowStruct)]
    struct A {
        a: LargeStruct,
        b: OtherLargeStruct,
        c: AnotherOne,
        ...
        z: YetAnother,
    }

    fn evaluation(cfg: &Config, state: &mut CowA) -> usize { ... }

    fn foo() {
        // original struct
        let a = A::new();

        let mut val = 0;
        // similar struct, with all the fields set as Cow::Borrowed from a;
        let mut cow = a.to_cow();
        let mut cow_out = cow.to_cow();
        let mut v_max = usize::min();
        for cfg in inputs() {
            // it is going to change cow, a remains untouched
            let val = evaluation(cfg, &mut cow);

            // let's pick the best candidate
            for candidate in candidates_b(val) {
                let mut cow2 = cow.to_cow();
                let v = evaluation_b(candidate, &mut cow2);
                if v > v_max {
                    v_max = v;
                    cow_out = cow2;
                }
            }
        }
    }
```
