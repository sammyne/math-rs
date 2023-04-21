use math::big::Int;

lazy_static::lazy_static! {
  static ref SUM_ZZ: Vec<ArgZz> = vec![
    ArgZz::new(Int::new(0), Int::new(0), Int::new(0)),
    ArgZz::new(Int::new(1), Int::new(1), Int::new(0)),
    ArgZz::new(Int::new(1111111110), Int::new(123456789), Int::new(987654321)),
    ArgZz::from_i64s(-1, -1, 0),
    ArgZz::from_i64s(864197532, -123456789, 987654321),
    ArgZz::from_i64s(-1111111110, -123456789, -987654321),
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
fn is_normalized(x: &Int) -> bool {
    match x.bits().next_back() {
        None => x.sign() == 0,
        Some(v) => v != 0,
    }
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
