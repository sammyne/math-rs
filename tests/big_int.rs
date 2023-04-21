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
