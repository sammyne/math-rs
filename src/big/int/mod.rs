use std::{
    fmt::Display,
    ops::{Neg, Not},
};

use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_traits::{One, Pow, Signed, Zero};

lazy_static::lazy_static! {
  static ref INT_ONE: Int = Int(BigInt::from(1i8));
}

//const W: usize = crate::bits::UINT_SIZE;
//const B: usize = 1 << W;
//const M: usize = usize::MAX;

//pub struct Int {
//    neg: bool,
//    abs: nat,
//}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Int(BigInt);

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Int {
    /// abs sets self to |x| (the absolute value of x) and returns self.
    pub fn abs(&mut self, x: &Self) -> &mut Self {
        self.0 = x.0.abs();
        self
    }

    pub fn add(&mut self, x: &Self, y: &Self) -> &mut Self {
        self.0 = &x.0 + &y.0;
        self
    }

    pub fn and(&mut self, x: &Self, y: &Self) -> &mut Self {
        self.0 = &x.0 & &y.0;
        self
    }

    pub fn and_not(&mut self, x: &Self, y: &Self) -> &mut Self {
        self.0 = &x.0 & (!&y.0);
        self
    }

    pub fn append(&mut self, buf: Vec<u8>, base: u32) -> Vec<u8> {
        let mut out = buf;
        out.extend(self.text(base).as_bytes());
        out
    }

    pub fn binomial(&mut self, n: i64, k: i64) -> &mut Self {
        if k > n {
            self.0.set_zero();
            return self;
        }

        let mut k = k;
        if k > n - k {
            k = n - k;
        }

        let mut i = BigInt::default();
        let n = BigInt::from(n);
        let k = BigInt::from(k);
        let mut z = INT_ONE.0.clone();
        while i < k {
            z *= &n - &i;
            i += 1;
            z /= &i;
        }

        self.0 = z;
        self
    }

    pub fn bit(&self, i: usize) -> u8 {
        if self.0.bit(i as u64) {
            1
        } else {
            0
        }
    }

    pub fn bit_len(&self) -> usize {
        self.0.abs().bits() as usize
    }

    pub fn bits(&self) -> Box<dyn DoubleEndedIterator<Item = u32> + '_> {
        Box::new(self.0.iter_u32_digits())
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = self.0.abs().to_signed_bytes_be();

        if out[0] == 0 {
            out.remove(0);
        }

        out
    }

    pub fn cmp(&self, y: &Self) -> i32 {
        if self.0 < y.0 {
            -1
        } else if self.0 == y.0 {
            0
        } else {
            1
        }
    }

    pub fn cmp_abs(&self, y: &Self) -> i32 {
        let a = self.0.abs();
        let b = y.0.abs();
        if a < b {
            -1
        } else if a == b {
            0
        } else {
            1
        }
    }

    pub fn div(&mut self, x: &Self, y: &Self) -> &mut Self {
        assert!(!y.0.is_zero(), "y mustn't be 0");

        let mut r = Int::default();
        self.quo_rem(x, y, &mut r);

        if r.sign() < 0 {
            if y.sign() < 0 {
                self.0 += &INT_ONE.0;
            } else {
                self.0 -= &INT_ONE.0;
            }
        }

        self
    }

    pub fn div_mod<'a, 'b>(
        &'a mut self,
        x: &Self,
        y: &Self,
        m: &'b mut Self,
    ) -> (&'a mut Self, &'b mut Self) {
        assert!(!y.0.is_zero(), "y mustn't be 0");
        self.0 = &x.0 / &y.0;

        self.quo_rem(x, y, m);
        if m.sign() < 0 {
            if y.sign() < 0 {
                self.0 += &INT_ONE.0;
                m.0 -= &y.0;
            } else {
                self.0 -= &INT_ONE.0;
                m.0 += &y.0;
            }
        }

        (self, m)
    }

    pub fn exp(&mut self, x: &Self, y: &Self, m: Option<&Self>) -> Option<&mut Self> {
        let mut xx = x.0.clone();

        if y.0.is_negative() {
            let mm = match &m {
                None => {
                    self.0 = INT_ONE.0.clone();
                    return Some(self);
                }
                Some(v) if v.0.is_zero() => {
                    self.0 = INT_ONE.0.clone();
                    return Some(self);
                }
                Some(v) => *v,
            };

            match Self::default().mod_inverse(x, mm) {
                None => {
                    println!("hello");
                    return None;
                }
                Some(v) => xx = v.0.abs(),
            }
        }
        let yy = y.0.abs();

        let mm = match m {
            Some(v) if !v.0.is_zero() => v.0.abs(),
            _ => {
                self.0 = xx.pow(yy.to_biguint().expect("as BigUint"));
                return Some(self);
            }
        };

        self.0 = xx.modpow(&yy, &mm);

        Some(self)
    }

    pub fn fill_bytes<'a>(&self, out: &'a mut [u8]) -> &'a mut [u8] {
        let b = self.bytes();

        assert!(out.len() >= b.len(), "buf too small");

        out.fill(0);

        let mut j = out.len();
        for i in (0..b.len()).rev() {
            out[j] = b[i];
            j -= 1;
        }

        out
    }

    pub fn gcd(
        &mut self,
        x: Option<&mut Self>,
        y: Option<&mut Self>,
        a: &Self,
        b: &Self,
    ) -> &mut Self {
        let (xx, yy) = if a.0.is_zero() && b.0.is_zero() {
            self.0.set_zero();
            //x.map(|v| v.0.set_zero());
            //y.map(|v| v.0.set_zero());
            //x.0.set_zero();
            //y.0.set_zero();
            (BigInt::zero(), BigInt::zero())
        } else if a.0.is_zero() && !b.0.is_zero() {
            self.0 = b.0.abs();
            (BigInt::zero(), b.0.signum())
        } else if !a.0.is_zero() && b.0.is_zero() {
            self.0 = a.0.abs();
            (a.0.signum(), BigInt::zero())
        } else {
            let v = a.0.extended_gcd(&b.0);
            self.0 = v.gcd;
            (v.x, v.y)
        };

        x.map(|v| v.0 = xx);
        y.map(|v| v.0 = yy);

        self
    }

    pub fn int64(&self) -> i64 {
        if let Ok(v) = self.0.clone().try_into() {
            v
        } else {
            0
        }
    }

    pub fn is_int64(&self) -> bool {
        <BigInt as TryInto<i64>>::try_into(self.0.clone()).is_ok()
    }

    pub fn is_uint64(&self) -> bool {
        <BigInt as TryInto<u64>>::try_into(self.0.clone()).is_ok()
    }

    pub fn lsh(&mut self, x: &Self, n: usize) -> &mut Self {
        self.0 = &x.0 << n;
        self
    }

    pub fn r#mod(&mut self, x: &Self, y: &Self) -> &mut Self {
        assert!(!y.0.is_zero(), "y mustn't be 0");

        let mut q = Int::default();
        q.quo_rem(x, y, self);

        if self.sign() < 0 {
            if y.sign() < 0 {
                self.0 -= &y.0;
            } else {
                self.0 += &y.0;
            }
        }

        self
    }

    pub fn mod_inverse(&mut self, g: &Self, n: &Self) -> Option<&mut Self> {
        assert!(!n.0.is_zero(), "n mustn't be 0");

        let n = Self(n.0.abs());

        let g = if g.0.is_negative() {
            Self(&g.0 % &n.0)
        } else {
            g.clone()
        };

        let mut d = Self::default();
        let mut x = Self::default();

        d.gcd(Some(&mut x), None, &g, &n);

        if d != *INT_ONE {
            return None;
        }

        if x.0.is_negative() {
            self.add(&x, &n);
        } else {
            self.0 = x.0;
        }

        Some(self)
    }

    pub fn mod_sqrt(&mut self, x: &Self, p: &Self) -> Option<&mut Self> {
        match jacobi(x, p) {
            -1 => return None,
            0 => return Some(self.set_int64(0)),
            _ => {}
        }

        let x = if x.0.is_negative() || (x.0 >= p.0) {
            let mut v = Int::default();
            v.r#mod(x, p);
            v
        } else {
            x.clone()
        };

        let d =
            p.0.iter_u32_digits()
                .next()
                .expect("take least significant digit");

        if d % 4 == 3 {
            self.mod_sqrt_3mod4_prime(&x, &p)
        } else if d % 8 == 5 {
            self.mod_sqrt_5mod8_prime(&x, &p)
        } else {
            self.mod_sqrt_tonelli_shanks(&x, &p)
        }
    }

    pub fn mul(&mut self, x: &Self, y: &Self) -> &mut Self {
        self.0 = &x.0 * &y.0;
        self
    }

    pub fn mul_range(&mut self, a: i64, b: i64) -> &mut Self {
        if a > b {
            return self.set_int64(1);
        } else if (a <= 0) && (b >= 0) {
            return self.set_int64(0);
        }

        let (a, b, sign) = if a < 0 {
            let s = if ((b - a) & 1) == 0 { -1 } else { 1 };
            (-b as u64, -a as u64, s)
        } else {
            (a as u64, b as u64, 1)
        };

        let mut p = BigInt::one();
        for i in a..=b {
            p *= i;
        }

        self.0 = p * sign;

        self
    }

    pub fn neg(&mut self, x: &Self) -> &mut Self {
        self.0 = x.0.clone().neg();
        self
    }

    pub fn not(&mut self, x: &Self) -> &mut Self {
        self.0 = x.0.clone().not();
        self
    }

    pub fn or(&mut self, x: &Self, y: &Self) -> &mut Self {
        self.0 = &x.0 | &y.0;
        self
    }

    /*
    pub fn probably_prime(&mut self, n: usize) -> bool {
        //assert!(n >= 0, "negative n for probably_prime");

        let v = &self.0;

        if v.is_negative() || v.is_zero() {
            return false;
        }

        const PRIME_BIT_MASK: u64 = (1 << 2)
            | (1 << 3)
            | (1 << 5)
            | (1 << 7)
            | (1 << 11)
            | (1 << 13)
            | (1 << 17)
            | (1 << 19)
            | (1 << 23)
            | (1 << 29)
            | (1 << 31)
            | (1 << 37)
            | (1 << 41)
            | (1 << 43)
            | (1 << 47)
            | (1 << 53)
            | (1 << 59)
            | (1 << 61);

        let w = v.iter_u64_digits().next().expect("as u64");
        if (v.bits() < 64) && (w < 64) {
            return (PRIME_BIT_MASK & (1 << w)) != 0;
        }

        if (w & 1) == 0 {
            return false;
        }

        const PRIMES_A: u64 = 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23 * 37;
        const PRIMES_B: u64 = 29 * 31 * 41 * 43 * 47 * 53;

        let (r_a, r_b): (u32, u32) = match W {
            32 => (
                (v % PRIMES_A).to_u32().unwrap(),
                (v % PRIMES_B).to_u32().unwrap(),
            ),
            64 => {
                let r = v % ((PRIMES_A * PRIMES_B) & (M as u64));
                (
                    (&r % PRIMES_A).to_u32().unwrap(),
                    (&r % PRIMES_B).to_u32().unwrap(),
                )
            }
            _ => panic!("math::big: invalid word size"),
        };

        if (r_a % 3 == 0)
            || (r_a % 5 == 0)
            || (r_a % 7 == 0)
            || (r_a % 11 == 0)
            || (r_a % 13 == 0)
            || (r_a % 17 == 0)
            || (r_a % 19 == 0)
            || (r_a % 23 == 0)
            || (r_a % 37 == 0)
            || (r_b % 29 == 0)
            || (r_b % 31 == 0)
            || (r_b % 41 == 0)
            || (r_b % 43 == 0)
            || (r_b % 47 == 0)
            || (r_b % 53 == 0)
        {
            return false;
        }

        probably_prime_miller_rabin(v, n + 1, true) && probably_prime_lucas(v)
    }
    */

    pub fn quo(&mut self, x: &Self, y: &Self) -> &mut Self {
        self.0 = &x.0 / &y.0;
        self
    }

    pub fn quo_rem<'a, 'b>(
        &'a mut self,
        x: &Self,
        y: &Self,
        r: &'b mut Self,
    ) -> (&'a mut Self, &'b mut Self) {
        assert!(!y.0.is_zero(), "divisor mustn't be 0");

        self.0 = &x.0 / &y.0;
        r.0 = &x.0 - &y.0 * &self.0;

        (self, r)
    }

    pub fn rand<R>(&mut self, rnd: &mut R, n: &Self) -> &mut Self
    where
        R: std::io::Read,
    {
        if !n.0.is_positive() {
            self.0.set_zero();
            return self;
        }

        let mut buf = vec![0u8; n.bit_len() / 8];
        self.0 = loop {
            rnd.read_exact(buf.as_mut_slice()).expect("fill buf");
            let v = BigInt::from_bytes_be(Sign::Plus, buf.as_slice());
            if v < n.0 {
                break v;
            }
        };

        self
    }

    pub fn rem(&mut self, x: &Self, y: &Self) -> &mut Self {
        assert!(!y.0.is_zero(), "divisor mustn't be 0");

        let q = &x.0 / &y.0;
        self.0 = &x.0 - &y.0 * &q;

        self
    }

    pub fn rsh(&mut self, x: &Self, n: usize) -> &mut Self {
        self.0 = &x.0 >> n;
        self
    }

    pub fn set(&mut self, x: &Self) -> &mut Self {
        self.0 = x.0.clone();
        self
    }

    pub fn set_bit(&mut self, x: &Self, i: usize, b: bool) -> &mut Self {
        self.0 = x.0.clone();
        self.0.set_bit(i as u64, b);
        self
    }

    pub fn set_bits(&mut self, abs: &[u32]) -> &mut Self {
        self.0 = BigInt::new(Sign::Plus, abs.to_vec());
        self
    }

    pub fn set_bytes(&mut self, buf: &[u8]) -> &mut Self {
        self.0 = BigInt::from_bytes_be(Sign::Plus, buf);
        self
    }

    pub fn set_int64(&mut self, x: i64) -> &mut Self {
        self.0 = BigInt::from(x);
        self
    }

    pub fn set_string(&mut self, s: &str, base: u8) -> Option<&mut Self> {
        let (s, sign) = if let Some(ss) = s.strip_prefix('-') {
            (ss, Sign::Minus)
        } else {
            (s, Sign::Plus)
        };

        if base != 0 {
            let v = BigInt::parse_bytes(s.as_bytes(), base as u32)?;
            self.0 = if sign != Sign::Minus { v } else { v.neg() };
            return Some(self);
        }

        let (s, base) = if let Some(s) = s.strip_prefix("0b") {
            (s, 2)
        } else if let Some(s) = s.strip_prefix("0B") {
            (s, 2)
        } else if let Some(s) = s.strip_prefix("0o") {
            (s, 8)
        } else if let Some(s) = s.strip_prefix("0O") {
            (s, 8)
        } else if let Some(s) = s.strip_prefix("0x") {
            (s, 16)
        } else if let Some(s) = s.strip_prefix("0X") {
            (s, 16)
        } else {
            (s, 10)
        };

        let v = BigInt::parse_bytes(s.as_bytes(), base as u32)?;
        self.0 = if sign != Sign::Minus { v } else { v.neg() };

        Some(self)
    }

    pub fn set_uint64(&mut self, x: u64) -> &mut Self {
        self.0 = x.into();
        self
    }

    pub fn sign(&self) -> i32 {
        if self.0.is_positive() {
            1
        } else if self.0.is_zero() {
            0
        } else {
            -1
        }
    }

    pub fn sqrt(&mut self, x: &Self) -> &mut Self {
        assert!(!x.0.is_zero(), "x mustn't be 0");

        self.0 = x.0.sqrt();

        self
    }

    pub fn string(&self) -> String {
        self.to_string()
    }

    pub fn sub(&mut self, x: &Self, y: &Self) -> &mut Self {
        self.0 = &x.0 - &y.0;
        self
    }

    pub fn text(&self, base: u32) -> String {
        // this is different from golang.
        // assert_eq!((base >= 2) && (base <= 36), "bad base");
        self.0.to_str_radix(base)
    }

    pub fn trailing_zero_bits(&self) -> usize {
        self.0.trailing_zeros().unwrap_or_default() as usize
    }

    pub fn uint64(&self) -> u64 {
        self.0.iter_u64_digits().next().unwrap_or_default()
    }

    pub fn xor(&mut self, x: &Self, y: &Self) -> &mut Self {
        self.0 = &x.0 ^ &y.0;
        self
    }

    pub fn new(x: i64) -> Self {
        Self(BigInt::from(x))
    }
}

impl Int {
    fn mod_sqrt_3mod4_prime(&mut self, x: &Self, p: &Self) -> Option<&mut Self> {
        let mut e = Self(BigInt::one() + &p.0);
        e.0 >>= 2;
        self.exp(x, &e, Some(p))
    }

    fn mod_sqrt_5mod8_prime(&mut self, x: &Self, p: &Self) -> Option<&mut Self> {
        let mut e = Int::default();
        e.rsh(p, 3);

        let mut tx = Int::default();
        tx.lsh(x, 1);

        let mut alpha = Int::default();
        alpha.exp(&tx, &e, Some(p));

        let mut beta = Int::default();

        beta.mul(&alpha, &alpha);
        beta.r#mod(&beta.clone(), p);
        beta.mul(&beta.clone(), &tx);
        beta.r#mod(&beta.clone(), p);
        beta.sub(&beta.clone(), &INT_ONE);
        beta.mul(&beta.clone(), x);
        beta.r#mod(&beta.clone(), p);
        beta.mul(&beta.clone(), &alpha);

        self.r#mod(&beta, p);

        Some(self)
    }

    fn mod_sqrt_tonelli_shanks(&mut self, x: &Self, p: &Self) -> Option<&mut Self> {
        let x = &x.0;
        let pp = p;
        let p = &p.0;

        let mut s: BigInt = p - 1;
        let e = s.trailing_zeros().expect("s is 0");
        s >>= e;

        let n = {
            let mut n = Self(BigInt::from(2i64));
            while jacobi(&n, pp) != -1 {
                n.0 += 1;
            }
            n.0
        };

        let mut y = &s + 1;
        y >>= 1;
        y = x.modpow(&y, p);
        let mut b = x.modpow(&s, p);
        let mut g = n.modpow(&s, p);

        let mut r = e;
        loop {
            let mut m = 0u64;
            let mut t = b.clone();
            while t != INT_ONE.0 {
                t = &t * &t % p;
                m += 1;
            }

            if m == 0 {
                self.0 = y;
                return Some(self);
            }

            t = BigInt::zero();
            t.set_bit(r - m - 1, true);
            t = g.modpow(&t, p);

            g = &t * &t % p;
            y = y * &t % p;
            b = b * &g % p;
            r = m;
        }
    }
}

pub fn jacobi(x: &Int, y: &Int) -> i32 {
    assert!(
        !(y.0.is_zero() || y.0.is_even()),
        "big: invalid 2nd argument to Int.Jacobi: need odd integer but got {}",
        y.0
    );

    let (mut a, mut b) = (x.clone(), y.clone());
    #[allow(unused_assignments)]
    let mut c = Int::default();
    let mut j = 1;

    if b.0.is_negative() {
        if a.0.is_negative() {
            j = -1;
        }
        b.abs(&b.clone());
    }

    loop {
        //println!("a={a}");
        //println!("b={b}");
        if b.0 == INT_ONE.0 {
            return j;
        }
        if a.0.is_zero() {
            return 0;
        }
        a.r#mod(&a.clone(), &b);
        //println!("a1={a}");
        if a.0.is_zero() {
            return 0;
        }

        let s = a.trailing_zero_bits();
        if (s & 1) != 0 {
            match b.0.iter_u32_digits().next().expect("get 1st digit") & 7 {
                3 | 5 => j = -j,
                _ => {}
            }
        }
        //println!("s={s}");
        c.rsh(&mut a, s);

        let b0 = b.0.iter_u32_digits().next().expect("get 1st word of b");
        let c0 = c.0.iter_u32_digits().next().expect("get 1st word of c");
        if (b0 & 3 == 3) && (c0 & 3 == 3) {
            j = -j;
        }

        a = b;
        b = c.clone();
    }
}

// fn probably_prime_lucas(v: &BigInt) -> bool {}

//fn probably_prime_miller_rabin(v: &BigInt, reps: usize, force2: bool) -> bool {
//    let nm1 = v - 1;
//    let k = v.trailing_zeros().unwrap_or_default();
//    let q = nm1 << k;
//
//    let nm3 = nm1 - 2;
//}
