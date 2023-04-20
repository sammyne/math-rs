use math::big::Int;

lazy_static::lazy_static! {
  static ref SUM_ZZ: Vec<ArgZz> = vec![
    ArgZz::new(Int::new(0), Int::new(0), Int::new(0))
  ];
}

struct ArgZz {
    z: Int,
    x: Int,
    y: Int,
}

impl ArgZz {
    fn new(z: Int, x: Int, y: Int) -> Self {
        Self { z, x, y }
    }
}

#[test]
fn sign_z() {}
