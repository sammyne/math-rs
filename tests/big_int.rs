use math::big::Int;

mod helper;

lazy_static::lazy_static! {
  static ref SUM_ZZ: Vec<ArgZz> = vec![
    ArgZz::new(Int::new(0), Int::new(0), Int::new(0)),
    ArgZz::new(Int::new(1), Int::new(1), Int::new(0)),
    ArgZz::new(Int::new(1111111110), Int::new(123456789), Int::new(987654321)),
    ArgZz::from_i64s(-1, -1, 0),
    ArgZz::from_i64s(864197532, -123456789, 987654321),
    ArgZz::from_i64s(-1111111110, -123456789, -987654321),
  ];

  static ref PROD_ZZ: Vec<ArgZz> = vec![
    ArgZz::from_i64s(0, 0, 0),
    ArgZz::from_i64s(0, 1, 0),
    ArgZz::from_i64s(1, 1, 1),
    ArgZz::from_i64s(-991*991, 991, -991),
  ];
}

#[derive(Clone, Debug)]
struct ArgZz {
    z: Int,
    x: Int,
    y: Int,
}

impl ArgZz {
    fn new(z: Int, x: Int, y: Int) -> Self {
        Self { z, x, y }
    }

    fn from_i64s(z: i64, x: i64, y: i64) -> Self {
        Self {
            z: Int::new(z),
            x: Int::new(x),
            y: Int::new(y),
        }
    }
}

#[test]
fn abs_z() {
    let zero = Int::default();
    for a in SUM_ZZ.iter() {
        let mut z = Int::default();
        z.abs(&a.z);
        let mut e = Int::default();
        e.set(&a.z);
        if e.cmp(&zero) < 0 {
            let v = e.clone();
            e.sub(&zero, &v);
        }

        assert_eq!(z, e);
    }
}

#[test]
fn binomial() {
    struct Case {
        n: i64,
        k: i64,
        want: &'static str,
    }
    let new_case = |n, k, want| -> Case { Case { n, k, want } };

    let test_vector = vec![
        new_case(0, 0, "1"),
        new_case(0, 1, "0"),
        new_case(1, 0, "1"),
        new_case(1, 1, "1"),
        new_case(1, 10, "0"),
        new_case(4, 0, "1"),
        new_case(4, 1, "4"),
        new_case(4, 2, "6"),
        new_case(4, 3, "4"),
        new_case(4, 4, "1"),
        new_case(10, 1, "10"),
        new_case(10, 9, "10"),
        new_case(10, 5, "252"),
        new_case(11, 5, "462"),
        new_case(11, 6, "462"),
        new_case(100, 10, "17310309456440"),
        new_case(100, 90, "17310309456440"),
        new_case(1000, 10, "263409560461970212832400"),
        new_case(1000, 990, "263409560461970212832400"),
    ];

    let mut z = Int::default();
    for c in test_vector {
        let got = z.binomial(c.n, c.k).to_string();
        assert_eq!(c.want, got, "binomial({},{})", c.n, c.k);
    }
}

#[test]
fn division_signs() {
    struct Case {
        x: i64,
        y: i64,
        q: i64,
        r: i64,
        d: i64,
        m: i64,
    }

    let new_case = |x, y, q, r, d, m| Case { x, y, q, r, d, m };

    let test_vector = vec![
        new_case(5, 3, 1, 2, 1, 2),
        new_case(-5, 3, -1, -2, -2, 1),
        new_case(5, -3, -1, 2, -1, 2),
        new_case(-5, -3, 1, -2, 2, 1),
        new_case(1, 2, 0, 1, 0, 1),
        new_case(8, 4, 2, 0, 2, 0),
    ];

    for (i, c) in test_vector.iter().enumerate() {
        let x = Int::new(c.x);
        let y = Int::new(c.y);
        let q = Int::new(c.q);
        let r = Int::new(c.r);
        let d = Int::new(c.d);
        let m = Int::new(c.m);

        let mut q1 = Int::default();
        q1.quo(&x, &y);

        let mut r1 = Int::default();
        r1.rem(&x, &y);

        assert!(is_normalized(&q1), "#{i} quo: {q1} is not normalized");
        assert!(is_normalized(&r1), "#{i} rem: {r1} is not normalized");
        assert!(
            (q1 == q) && (r1 == r),
            "#{i} quo/rem: got ({q1}, {r1}), want ({q}, {r})"
        );

        let mut q2 = Int::default();
        let mut r2 = Int::default();
        q2.quo_rem(&x, &y, &mut r2);

        assert!(is_normalized(&q2), "#{i} quo: {q2} is not normalized");
        assert!(is_normalized(&r2), "#{i} rem: {r2} is not normalized");
        assert!(
            (q2 == q) && (r2 == r),
            "#{i} quo_rem: got ({q2}, {r2}), want ({q}, {r})"
        );

        let mut d1 = Int::default();
        let mut m1 = Int::default();
        d1.div(&x, &y);
        m1.r#mod(&x, &y);

        assert!(is_normalized(&d1), "#{i} div: {d1} is not normalized");
        assert!(is_normalized(&m1), "#{i} mod: {m1} is not normalized");
        assert!(
            (d1 == d) && (m1 == m),
            "#{i} div/mod: got ({d1}, {m1}), want ({d}, {m})"
        );

        let mut d2 = Int::default();
        let mut m2 = Int::default();
        d2.div_mod(&x, &y, &mut m2);

        assert!(is_normalized(&d2), "#{i} div: {d2} is not normalized");
        assert!(is_normalized(&m2), "#{i} mod: {m2} is not normalized");
        assert!(
            (d2 == d) && (m2 == m),
            "#{i} div_mod: got ({d2}, {m2}), want ({d}, {m})"
        );
    }
}

#[test]
fn mul() {
    let n = randn(128, 256);
    for _ in 0..n {
        let a = rand_bytes(randn(1, 64));
        let b = rand_bytes(randn(1, 64));
        assert!(
            check_mul(a.as_slice(), b.as_slice()),
            "a={:?}, b={:?}",
            a,
            b
        );
    }
}

#[test]
fn mul_range_z() {
    struct S<T> {
        a: T,
        b: T,
        prod: &'static str,
    }

    impl<T> S<T> {
        fn new(a: T, b: T, prod: &'static str) -> Self {
            Self { a, b, prod }
        }
    }

    let e9 = 10i64.pow(9);
    let e9u = 10u64.pow(9);

    let mul_ranges_n = vec![
        S::new(0u64, 0, "0"),
        S::new(1, 1, "1"),
        S::new(1, 2, "2"),
        S::new(1, 3, "6"),
        S::new(10, 10, "10"),
        S::new(0, 100, "0"),
        S::new(0, e9u, "0"),
        S::new(1, 0, "1"),                    // empty range
        S::new(100, 1, "1"),                  // empty range
        S::new(1, 10, "3628800"),             // 10!
        S::new(1, 20, "2432902008176640000"), // 20!
        S::new(
            1,
            100,
            concat!(
                "933262154439441526816992388562667004907159682643816214685929",
                "638952175999932299156089414639761565182862536979208272237582",
                "51185210916864000000000000000000000000"
            ),
        ), // 100!
    ];

    let mul_ranges_z = vec![
        // entirely positive ranges are covered by mulRangesN
        S::new(-1, 1, "0"),
        S::new(-2, -1, "2"),
        S::new(-3, -2, "6"),
        S::new(-3, -1, "-6"),
        S::new(1, 3, "6"),
        S::new(-10, -10, "-10"),
        S::new(0, -1, "1"),                      // empty range
        S::new(-1, -100, "1"),                   // empty range
        S::new(-1, 1, "0"),                      // range includes 0
        S::new(-1 * e9, 0, "0"),                 // range includes 0
        S::new(-1 * e9, e9, "0"),                // range includes 0
        S::new(-10, -1, "3628800"),              // 10!
        S::new(-20, -2, "-2432902008176640000"), // -20!
        S::new(
            -99,
            -1,
            concat!(
                "-933262154439441526816992388562667004907159682643816214685929",
                "638952175999932299156089414639761565182862536979208272237582",
                "511852109168640000000000000000000000"
            ), // -99!
        ),
    ];

    let mut tmp = Int::default();
    for (i, r) in mul_ranges_n.iter().enumerate() {
        let prod = tmp.mul_range(r.a as i64, r.b as i64).string();
        assert_eq!(&prod, r.prod, "{i}");
    }

    for (i, r) in mul_ranges_z.iter().enumerate() {
        let prod = tmp.mul_range(r.a as i64, r.b as i64).string();
        assert_eq!(&prod, r.prod, "{i}");
    }
}

#[test]
fn prod_zz() {
    fn mul_zz<'a>(z: &'a mut Int, x: &Int, y: &Int) -> &'a mut Int {
        z.mul(x, y)
    }

    for a in PROD_ZZ.iter() {
        test_fun_zz("mul_zz", mul_zz, a);

        let arg = ArgZz::new(a.z.clone(), a.y.clone(), a.x.clone());
        test_fun_zz("mul_zz symmetric", mul_zz, &arg);
    }
}

#[test]
fn set_z() {
    for a in SUM_ZZ.iter() {
        let mut z = Int::default();
        z.set(&a.z);
        assert!(is_normalized(&z), "{z} is not normalized");
        assert!(z.cmp(&a.z) == 0, "got z={}; want {}", z, a.z);
    }
}

#[test]
fn sign_z() {
    let zero = Int::default();
    for a in SUM_ZZ.iter() {
        let s = a.z.sign();
        let e = a.z.cmp(&zero);
        assert_eq!(s, e, "z = {}", a.z);
    }
}

#[test]
fn sum_zz() {
    fn add_zz<'a>(z: &'a mut Int, x: &Int, y: &Int) -> &'a mut Int {
        z.add(x, y)
    }

    fn sub_zz<'a>(z: &'a mut Int, x: &Int, y: &Int) -> &'a mut Int {
        z.sub(x, y)
    }

    for a in SUM_ZZ.iter() {
        let arg = a;
        test_fun_zz("add_zz", add_zz, arg);

        let arg = ArgZz::new(a.z.clone(), a.y.clone(), a.x.clone());
        test_fun_zz("add_zz symmetric", add_zz, &arg);

        let arg = ArgZz::new(a.x.clone(), a.z.clone(), a.y.clone());
        test_fun_zz("sub_zz", sub_zz, &arg);

        let arg = ArgZz::new(a.y.clone(), a.z.clone(), a.x.clone());
        test_fun_zz("sub_zz symmetric", sub_zz, &arg);
    }
}

// private functions
fn check_mul(a: &[u8], b: &[u8]) -> bool {
    let mut x = Int::default();
    let mut y = Int::default();
    let mut z1 = Int::default();

    x.set_bytes(a);
    y.set_bytes(b);

    z1.mul(&x, &y);

    let mut z2 = Int::default();
    z2.set_bytes(&mul_bytes(a, b));

    z1 == z2
}

fn is_normalized(x: &Int) -> bool {
    match x.bits().next_back() {
        None => x.sign() == 0,
        Some(v) => v != 0,
    }
}

fn mul_bytes(x: &[u8], y: &[u8]) -> Vec<u8> {
    let mut z = vec![0u8; x.len() + y.len()];

    let mut k0 = z.len() - 1;
    for j in (0..y.len()).rev() {
        let d = y[j] as i32;
        if d != 0 {
            let mut k = k0;
            let mut carry = 0;
            for i in (0..x.len()).rev() {
                let t = (z[k] as i32) + (x[i] as i32) * d + carry;
                z[k] = t as u8;
                carry = t >> 8;
                k -= 1;
            }
            z[k] = carry as u8;
        }
        println!("j={j}, k0={k0}, x.len()={}", x.len());
        k0 -= 1;
    }

    let mut i = 0;
    while (i < z.len()) && (z[i] == 0) {
        i += 1;
    }

    let (_, b) = z.split_at(i);

    b.to_vec()
}

fn rand_bytes(n: usize) -> Vec<u8> {
    let mut out = vec![0u8; n];
    helper::rand::read(&mut out).unwrap();
    out
}

fn randn(lo: u64, hi: u64) -> usize {
    let mut b = [0u8; 8];
    helper::rand::read(&mut b).unwrap();

    (u64::from_be_bytes(b) % (hi - lo) + lo) as usize
}

fn test_fun_zz<F>(msg: &str, f: F, a: &ArgZz)
where
    F: for<'a> Fn(&'a mut Int, &Int, &Int) -> &'a mut Int,
{
    let mut z = Int::default();
    f(&mut z, &a.x, &a.y);
    assert!(is_normalized(&z), "{msg}{z} is not normalized");
    assert_eq!(z, a.z, "{msg}{a:?}");
}
