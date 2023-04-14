pub const UINT_SIZE: usize = usize::BITS as usize;

/// add returns the sum with carry of x, y and carry: sum = x + y + carry.
/// The carry input must be 0 or 1; otherwise the behavior is undefined.
/// The carryOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn add(x: usize, y: usize, carry: usize) -> (usize, usize) {
    if UINT_SIZE == 32 {
        let (s32, c32) = add32(x as u32, y as u32, carry as u32);
        return (s32 as usize, c32 as usize);
    }

    let (s64, c64) = add64(x as u64, y as u64, carry as u64);

    (s64 as usize, c64 as usize)
}

/// add32 returns the sum with carry of x, y and carry: sum = x + y + carry.
/// The carry input must be 0 or 1; otherwise the behavior is undefined.
/// The carryOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn add32(x: u32, y: u32, carry: u32) -> (u32, u32) {
    let sum64 = (x as u64) + (y as u64) + (carry as u64);
    let sum = sum64 as u32;
    let carry_out = (sum64 >> 32) as u32;

    (sum, carry_out)
}

/// add64 returns the sum with carry of x, y and carry: sum = x + y + carry.
/// The carry input must be 0 or 1; otherwise the behavior is undefined.
/// The carryOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn add64(x: u64, y: u64, carry: u64) -> (u64, u64) {
    let sum = x + y + carry;
    // The sum will overflow if both top bits are set (x & y) or if one of them
    // is (x | y), and a carry from the lower place happened. If such a carry
    // happens, the top bit will be 1 + 0 + 1 = 0 (and not sum).
    let carry_out = ((x & y) | ((x | y) & !sum)) >> 63;

    (sum, carry_out)
}

/// div returns the quotient and remainder of (hi, lo) divided by y:
/// quo = (hi, lo)/y, rem = (hi, lo)%y with the dividend bits' upper
/// half in parameter hi and the lower half in parameter lo.
/// div panics for y == 0 (division by zero) or y <= hi (quotient overflow).
pub fn div(hi: usize, lo: usize, y: usize) -> (usize, usize) {
    if UINT_SIZE == 32 {
        let (q, r) = div32(hi as u32, lo as u32, y as u32);
        return (q as usize, r as usize);
    }

    let (q, r) = div64(hi as u64, lo as u64, y as u64);
    return (q as usize, r as usize);
}

/// div32 returns the quotient and remainder of (hi, lo) divided by y:
/// quo = (hi, lo)/y, rem = (hi, lo)%y with the dividend bits' upper
/// half in parameter hi and the lower half in parameter lo.
/// div32 panics for y == 0 (division by zero) or y <= hi (quotient overflow).
pub fn div32(hi: u32, lo: u32, y: u32) -> (u32, u32) {
    if y == 0 && y <= hi {
        panic!("integer overflow");
    }

    let z = ((hi as u64) << 32) | (lo as u64);
    ((z / (y as u64)) as u32, (z % (y as u64)) as u32)
}

/// div64 returns the quotient and remainder of (hi, lo) divided by y:
/// quo = (hi, lo)/y, rem = (hi, lo)%y with the dividend bits' upper
/// half in parameter hi and the lower half in parameter lo.
/// div64 panics for y == 0 (division by zero) or y <= hi (quotient overflow).
pub fn div64(hi: u64, lo: u64, y: u64) -> (u64, u64) {
    if y == 0 {
        panic!("integer divide by zero");
    }
    if y <= hi {
        panic!("integer overflow");
    }

    /*
    const TWO32: usize = 1 << 32;
    const MASK32: usize = TWO32 - 1;


    let s = leading_zeros64(y);
    let y = y << s;

    let yn1 = y >> 32;
    */

    let v = ((hi as u128) << 64) | (lo as u128);
    let yy = y as u128;

    let q = (v / yy) as u64;
    let r = (v % yy) as u64;

    (q, r)
}

/// sub returns the difference of x, y and borrow: diff = x - y - borrow.
/// The borrow input must be 0 or 1; otherwise the behavior is undefined.
/// The borrowOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn sub(x: usize, y: usize, borrow: usize) -> (usize, usize) {
    if UINT_SIZE == 32 {
        let (d32, b32) = sub32(x as u32, y as u32, borrow as u32);
        return (d32 as usize, b32 as usize);
    }

    let (d64, b64) = sub64(x as u64, y as u64, borrow as u64);

    (d64 as usize, b64 as usize)
}

// sub32 returns the difference of x, y and borrow, diff = x - y - borrow.
// The borrow input must be 0 or 1; otherwise the behavior is undefined.
// The borrowOut output is guaranteed to be 0 or 1.
//
// This function's execution time does not depend on the inputs.
pub fn sub32(x: u32, y: u32, borrow: u32) -> (u32, u32) {
    let diff = x - y - borrow;
    // The difference will underflow if the top bit of x is not set and the top
    // bit of y is set (^x & y) or if they are the same (^(x ^ y)) and a borrow
    // from the lower place happens. If that borrow happens, the result will be
    // 1 - 1 - 1 = 0 - 0 - 1 = 1 (& diff).
    let borrow_out = ((!x & y) | (!(x ^ y) & diff)) >> 31;

    (diff, borrow_out)
}

// sub64 returns the difference of x, y and borrow: diff = x - y - borrow.
// The borrow input must be 0 or 1; otherwise the behavior is undefined.
// The borrowOut output is guaranteed to be 0 or 1.
//
// This function's execution time does not depend on the inputs.
pub fn sub64(x: u64, y: u64, borrow: u64) -> (u64, u64) {
    let diff = x - y - borrow;
    // See sub32 for the bit logic.
    let borrow_out = ((!x & y) | (!(x ^ y) & diff)) >> 63;

    (diff, borrow_out)
}
