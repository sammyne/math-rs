use crate::big::nat::nat;

lazy_static::lazy_static! {
  static ref INT_ONE: Int= Int{
    neg:false,
    abs:nat::one(),
  };
}

pub struct Int {
    neg: bool,
    abs: nat,
}

impl Int {
    /// abs sets self to |x| (the absolute value of x) and returns self.
    pub fn abs(&mut self, x: &Self) -> &mut Self {
        self.set(x);
        self.neg = false;

        self
    }

    pub fn add(&mut self, x: &Self, y: &Self) -> &mut Self {
        let mut neg = x.neg;
        if x.neg == y.neg {
            // x + y == x + y
            // (-x) + (-y) == -(x + y)
            self.abs.add(&x.abs, &y.abs);
        } else {
            // x + (-y) == x - y == -(y - x)
            // (-x) + y == y - x == -(x - y)
            if x.abs.cmp(&y.abs) >= 0 {
                self.abs.sub(&x.abs, &y.abs);
            } else {
                neg = !neg;
                self.abs.sub(&y.abs, &x.abs);
            }
        }

        self.neg = (self.abs.0.len() > 0) && neg;

        self
    }

    /*
    pub fn and_not(&mut self, x: &Self, y: &Self) -> Self {
        todo!()
    }

    pub fn append<'a>(&'a mut self, buf: &[u8], base: i32) -> &'a [u8] {
        todo!()
    }

    pub fn binomial(&mut self, n: i64, k: i64) -> &mut Self {
        todo!()
    }

    pub fn bit(&self, i: usize) -> usize {
        todo!()
    }

    pub fn bit_len(&self) -> usize {
        todo!()
    }

    pub fn bits(&self) -> Vec<Word> {
        todo!()
    }

    pub fn bytes(&self) -> &[u8] {
        todo!()
    }
    */

    pub fn set(&mut self, x: &Self) -> &mut Self {
        if !std::ptr::eq(self, x) {
            self.abs.set(&x.abs);
            self.neg = x.neg;
        }

        self
    }

    pub fn new(_x: i64) -> Self {
        todo!()
    }
}
