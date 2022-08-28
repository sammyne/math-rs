use crate::big::arith;
use crate::big::Word;

#[allow(non_camel_case_types)]
pub struct nat(pub(crate) Vec<Word>);

impl nat {
    pub fn add(&mut self, x: &Self, y: &Self) -> &mut Self {
        let (m, n) = (x.0.len(), y.0.len());

        if m < n {
            return self.add(y, x);
        } else if m == 0 {
            self.0.clear();
            return self; // n==0 because m>=n; result is 0
        } else if n == 0 {
            return self.set(x); // result is x
        }
        // m>0

        self.make(m + 1);

        let mut c = arith::add_vv(&mut self.0[0..n], &x.0, &y.0);
        if m > n {
            c = arith::add_vw(&mut self.0[0..n], &x.0[n..], 0)
        }
        self.0[m] = c;

        self.norm()
    }

    pub fn cmp(&self, y: &Self) -> i32 {
        let (m, n) = (self.0.len(), y.0.len());
        if (m != n) || (m == 0) {
            let r = if m < n {
                -1
            } else if m > n {
                1
            } else {
                0
            };

            return r;
        }

        let mut i = m - 1;
        while (i > 0) && (self.0[i] == y.0[i]) {
            i -= 1;
        }

        if self.0[i] < y.0[i] {
            -1
        } else if self.0[i] > y.0[i] {
            1
        } else {
            0
        }
    }

    pub fn make(&mut self, n: usize) -> Self {
        if n == 1 {
            return Self(vec![0; 1]);
        }

        const E: usize = 4;
        let mut values = Vec::with_capacity(n + E);
        values.resize(n, 0);
        Self(values)
    }

    pub fn norm(&mut self) -> &mut Self {
        let mut i = self.0.len();
        while (i > 0) && (self.0[i - 1] == 0) {
            i -= 1;
        }

        self.0.resize(i, 0);
        self
    }

    pub fn set(&mut self, x: &Self) -> &mut Self {
        *self = self.make(x.0.len());
        self.0.copy_from_slice(&x.0);
        self
    }

    pub fn sub(&mut self, x: &Self, y: &Self) -> &mut Self {
        let (m, n) = (x.0.len(), y.0.len());

        if m < n {
            panic!("underflow");
        } else if m == 0 {
            self.0.clear();
            return self; // n == 0 because m >= n; result is 0
        } else if n == 0 {
            return self.set(&x); // result is x
        }
        // m>0

        self.make(m);
        let mut c = arith::sub_vv(&mut self.0[..n], &x.0, &y.0);
        if m > n {
            c = arith::sub_vw(&mut self.0[n..], &x.0[n..], c);
        }
        if c != 0 {
            panic!("underflow");
        }

        self.norm()
    }

    pub fn one() -> Self {
        Self(vec![1])
    }
}
