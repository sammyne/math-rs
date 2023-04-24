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
fn bit_len() {
    struct Case {
        input: &'static str,
        out: usize,
    }

    let new_case = |input, out| Case { input, out };

    let test_vector = vec![
        new_case("-1", 1),
        new_case("0", 0),
        new_case("1", 1),
        new_case("2", 2),
        new_case("4", 3),
        new_case("0xabc", 12),
        new_case("0x8000", 16),
        new_case("0x80000000", 32),
        new_case("0x800000000000", 48),
        new_case("0x8000000000000000", 64),
        new_case("0x80000000000000000000", 80),
        new_case("-0x4000000000000000000000", 87),
    ];

    for (i, c) in test_vector.iter().enumerate() {
        let mut x = Int::default();
        assert!(
            x.set_string(c.input, 0).is_some(),
            "#{i} set_string({}, 0)",
            c.input
        );

        assert_eq!(x.bit_len(), c.out, "#{i} bit_len");
    }
}

#[test]
fn bits() {
    let test_vector = vec![
        vec![0u32],
        vec![1],
        vec![0, 1, 2, 3, 4],
        vec![4, 3, 2, 1, 0],
        vec![4, 3, 2, 1, 0, 0, 0, 0],
    ];

    for c in test_vector {
        let mut z = Int::default();

        let got = z.set_bits(c.as_slice());

        assert!(got.sign() >= 0, "set_bits({c:?}): get negative result");

        let want = norm(c.as_slice());
        let got: Vec<u32> = got.bits().collect();
        assert_eq!(got, want, "set_bits({c:?})");

        let bits: Vec<u32> = z.bits().collect();
        assert_eq!(
            bits,
            want,
            "{:?}.bits() = {:?}; want {:?}",
            z.bytes(),
            bits,
            want
        );
    }
}

#[test]
fn bytes() {
    let n = randn(128, 256);
    for _ in 0..n {
        let b = rand_bytes(randn(1, 64));
        check_bytes(b.as_slice());
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
fn quo() {
    let n = randn(128, 256);
    for _ in 0..n {
        let a = rand_bytes(randn(1, 64));
        let b = rand_bytes(randn(1, 64));
        check_quo(a.as_slice(), b.as_slice());
    }

    struct Case {
        x: &'static str,
        y: &'static str,
        q: &'static str,
        r: &'static str,
    }
    let new_case = |x, y, q, r| Case { x, y, q, r };

    let test_vector = vec![
        new_case(
            "476217953993950760840509444250624797097991362735329973741718102894495832294430498335824897858659711275234906400899559094370964723884706254265559534144986498357",
            "9353930466774385905609975137998169297361893554149986716853295022578535724979483772383667534691121982974895531435241089241440253066816724367338287092081996",
            "50911",
            "1",
        ),
       new_case("11510768301994997771168",
            "1328165573307167369775",
            "8",
            "885443715537658812968",
       ),
    ];

    fn int_from_decimal_string(s: &str) -> Int {
        let mut out = Int::default();
        out.set_string(s, 10);
        out
    }

    for (i, c) in test_vector.iter().enumerate() {
        let x = int_from_decimal_string(c.x);
        let y = int_from_decimal_string(c.y);
        let expected_q = int_from_decimal_string(c.q);
        let expected_r = int_from_decimal_string(c.r);

        let mut r = Int::default();
        let mut q = Int::default();
        q.quo_rem(&x, &y, &mut r);

        assert!(
            (q == expected_q) && (r == expected_r),
            "#{i} got ({q},{r}) want ({expected_q}, {expected_r})"
        );
    }
}

#[test]
fn quo_step_d6() {
    let u = {
        let mut v = Int::default();

        let abs = [0, 0, 0, 0, 1, 1 << 31, u32::MAX, u32::MAX ^ (1 << 31)];

        v.set_bits(&abs);
        v
    };

    let v = {
        let mut v = Int::default();

        let abs = [5, 0, 2, 1 << 31, 0, 1 << 31];

        v.set_bits(&abs);
        v
    };

    let mut r = Int::default();
    let mut q = Int::default();
    q.quo_rem(&u, &v, &mut r);

    const EXPECTED_Q64: &'static str = "18446744073709551613";
    const EXPECTED_R64: &'static str = "3138550867693340382088035895064302439801311770021610913807";
    //const EXPECTED_Q32: &'static str = "4294967293";
    //const EXPECTED_R32: &'static str = "39614081266355540837921718287";

    assert_eq!(q.to_string(), EXPECTED_Q64, "bad q64");
    //assert_eq!(q.to_string(), EXPECTED_Q32, "bad q32");
    assert_eq!(r.to_string(), EXPECTED_R64, "bad r64");
    //assert_eq!(r.to_string(), EXPECTED_R32, "bad r32");
}

#[test]
fn set_bytes() {
    let n = randn(128, 256);
    for _ in 0..n {
        let b = rand_bytes(randn(1, 64));
        check_set_bytes(b.as_slice());
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
fn check_bytes(b: &[u8]) {
    let b = {
        let mut v = b;
        while (v.len() > 0) && (v[0] == 0) {
            v = &v[1..];
        }
        v
    };

    let mut v = Int::default();
    let b2 = v.set_bytes(b).bytes();

    assert_eq!(b, b2.as_slice())
}

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

fn check_quo(x: &[u8], y: &[u8]) {
    let mut u = Int::default();
    u.set_bytes(x);

    let mut v = Int::default();
    v.set_bytes(y);

    let zero = Int::default();

    if &v == &zero {
        return;
    }

    let mut r = Int::default();
    let mut q = Int::default();
    q.quo_rem(&u, &v, &mut r);

    assert!(r.cmp(&v) < 0, "remainder not less than divisor");

    let uprime = {
        let mut out = q.clone();
        out.mul(&q, &v);

        let vv = out.clone();
        out.add(&vv, &r);

        out
    };

    assert_eq!(uprime, u);
}

fn check_set_bytes(b: &[u8]) {
    let b1 = {
        let mut v = Int::default();
        v.set_bytes(b).bytes()
    };

    fn normalize(b: &[u8]) -> &[u8] {
        let mut out = b;
        while (out.len() > 1) && (out[0] == 0) {
            out = &out[1..];
        }
        out
    }

    assert_eq!(normalize(b1.as_slice()), normalize(b));
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

fn norm(x: &[u32]) -> Vec<u32> {
    let mut i = x.len();
    while (i > 0) && (x[i - 1] == 0) {
        i -= 1;
    }

    x[..i].to_vec()
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
