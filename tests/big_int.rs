use math::big::int::{self, Extended, Int, Integer};

/*
struct ArgZZ {
    z: Int,
    x: Int,
    y: Int,
}

impl ArgZZ {
    fn new(z: Int, x: Int, y: Int) -> Self {
        Self { z, x, y }
    }
}

fn sum_zz() -> Vec<ArgZZ> {
    vec![
        (ArgZZ::new(big::new_int(0), big::new_int(0), big::new_int(0))),
        (ArgZZ::new(big::new_int(1), big::new_int(1), big::new_int(0))),
        (ArgZZ::new(
            big::new_int(1111111110),
            big::new_int(123456789),
            big::new_int(987654321),
        )),
        (ArgZZ::new(big::new_int(-1), big::new_int(-1), big::new_int(0))),
        (ArgZZ::new(
            big::new_int(864197532),
            big::new_int(-123456789),
            big::new_int(987654321),
        )),
        (ArgZZ::new(
            big::new_int(-1111111110),
            big::new_int(-123456789),
            big::new_int(-987654321),
        )),
    ]
}

#[test]
fn abs() {
    let test_vector = sum_zz();

    let zero = big::new_int(0);
    for v in &test_vector {
        let mut z = big::new_int(0);
        z.abs(&v.z);

        let mut e = big::new_int(0);
        e.abs(&v.z);

        //if e.cmp(&zero) < 0 {}
    }
}
*/

#[test]
fn mod_inverse() {
    // in form of (element, modulus)
    let test_vector = vec![
        ("1234567", "458948883992"),
        ("239487239847", "2410312426921032588552076022197566074856950548502459942654116941958108831682612228890093858261341614673227141477904012196503648957050582631942730706805009223062734745341073406696246014589361659774041027169249453200378729434170325843778659198143763193776859869524088940195577346119843545301547043747207749969763750084308926339295559968882457872412993810129130294592999947926365264059284647209730384947211681434464714438488520940127459844288859336526896320919633919"),
        ("-10", "13"), // issue #16984
        ("10", "-13"),
        ("-17","-13"),
    ];

    for (i, v) in test_vector.iter().enumerate() {
        let element = Int::from_string(v.0, 10).expect("failed to parse Int");
        let modulus = Int::from_string(v.1, 10).expect("failed to parse Int");

        let inverse = {
            let inverse = Int::mod_inverse(&element, &modulus).unwrap();
            println!("i: {:?}", inverse);

            let inverse = {
                let v = inverse * &element;
                Int::euclidean_modulus(&v, &modulus)
            };
            inverse
        };

        assert_eq!(
            inverse,
            int::one(),
            "#{}: mod_inverse({:?},{:?})={:?}!=1",
            i,
            element,
            modulus,
            inverse
        );
    }

    // exhaustive test for small values
    for n in 2..100 {
        let modulus = Int::from(n);
        for x in 1..n {
            let element = Int::from(x);

            let gcd = element.gcd(&modulus);
            if gcd != int::one() {
                continue;
            }

            let inverse = {
                let inverse = Int::mod_inverse(&element, &modulus).unwrap();
                println!("i: {:?}", inverse);

                let inverse = {
                    let v = inverse * &element;
                    Int::euclidean_modulus(&v, &modulus)
                };
                inverse
            };

            assert_eq!(
                inverse,
                int::one(),
                "mod_inverse({:?},{:?})={:?}!=1",
                element,
                modulus,
                inverse
            );
        }
    }
}
