use num_bigint::{BigInt, Sign};
pub use num_integer::{ExtendedGcd, Integer};

pub type Int = BigInt;

/// Extended specifies more API to extend BigInt.
pub trait Extended {
    /// euclidean_modulus calculates the modulus x%y for y != 0. If y == 0, a division-by-zero
    /// run-time panic occurs.
    /// Alias to Int.Mod in Go.
    fn euclidean_modulus(x: &Int, y: &Int) -> Int;
    /// mod_inverse calculates the multiplicative inverse of g in the ring ℤ/nℤ. If g and n are not
    /// relatively prime, g has no multiplicative inverse in the ring ℤ/nℤ. In this case, the
    /// return value is None.
    fn mod_inverse(g: &Int, n: &Int) -> Option<Int>;
    /// from_string makes a Int out of the value of s, interpreted in the given base, and returns
    /// the resultant Int. The entire string (not just a prefix) must be valid for success. If
    /// from_string fails, the returned value is None.
    /// Alias to Int.SetString in Go.
    fn from_string(s: &str, base: isize) -> Option<Int>;
}

impl Extended for Int {
    fn euclidean_modulus(x: &Int, y: &Int) -> Int {
        let z = x % y;
        match z.sign() {
            Sign::Minus => match y.sign() {
                Sign::Minus => z - y,
                _ => z + y,
            },
            _ => z,
        }
    }
    fn mod_inverse(g: &Int, n: &Int) -> Option<Int> {
        let n = match n.sign() {
            Sign::Minus => -n.clone(),
            _ => n.clone(),
        };
        let g = match g.sign() {
            Sign::Minus => Int::euclidean_modulus(&g, &n),
            _ => g.clone(),
        };

        let ExtendedGcd { gcd, x, .. } = g.extended_gcd(&n);

        if gcd != one() {
            return None;
        }

        match x.sign() {
            Sign::Minus => Some(x + n),
            _ => Some(x),
        }
    }

    fn from_string(s: &str, base: isize) -> Option<Int> {
        Int::parse_bytes(s.as_bytes(), base as u32)
    }
}

pub fn one() -> Int {
    Int::from(1u8)
}

pub fn zero() -> Int {
    Int::from(0u8)
}
