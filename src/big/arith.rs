use crate::big::Word;
use crate::bits;

pub fn add_vv(z: &mut [Word], x: &[Word], y: &[Word]) -> Word {
    add_vv_g(z, x, y)
}

pub fn add_vw(z: &mut [Word], x: &[Word], y: Word) -> Word {
    let f = if z.len() > 32 { add_vw_g } else { add_vw_large };

    f(z, x, y)
}

// The resulting carry c is either 0 or 1.
pub fn add_vw_g(z: &mut [Word], x: &[Word], y: Word) -> Word {
    let mut c = y;

    for i in 0..(z.len().min(x.len())) {
        let (zi, cc) = bits::add(x[i], c, 0);
        z[i] = zi;
        c = cc;
    }

    c
}

pub fn add_vw_large(z: &mut [Word], x: &[Word], y: Word) -> Word {
    let mut c = y;

    let n = z.len().min(x.len());
    for i in 0..n {
        if c == 0 {
            z[i..n].copy_from_slice(&x[i..n]);
            return c;
        }

        let (zi, cc) = bits::add(x[i], c, 0);
        z[i] = zi;
        c = cc;
    }

    c
}

pub fn sub_vv(z: &mut [Word], x: &[Word], y: &[Word]) -> Word {
    sub_vv_g(z, x, y)
}

pub fn sub_vw(z: &mut [Word], x: &[Word], y: Word) -> Word {
    let f = if z.len() <= 32 {
        sub_vw_g
    } else {
        sub_vw_large
    };

    f(z, x, y)
}

// The resulting carry c is either 0 or 1.
fn add_vv_g(z: &mut [Word], x: &[Word], y: &[Word]) -> Word {
    let n = z.len().min(x.len().min(y.len()));

    let mut c = 0;
    for i in 0..n {
        let (zi, cc) = bits::add(x[i], y[i], c);
        z[i] = zi;
        c = cc;
    }

    c
}

fn sub_vv_g(z: &mut [Word], x: &[Word], y: &[Word]) -> Word {
    let n = z.len().min(x.len().min(y.len()));

    let mut c = 0;
    for i in 0..n {
        let (zi, cc) = bits::sub(x[i], y[i], c);
        z[i] = zi;
        c = cc;
    }

    c
}

fn sub_vw_g(z: &mut [Word], x: &[Word], y: Word) -> Word {
    let mut c = y;

    // The comment near the top of this file discusses this for loop condition.
    for i in 0..(z.len().min(x.len())) {
        let (zi, cc) = bits::sub(x[i], c, 0);
        z[i] = zi;
        c = cc;
    }

    c
}

// sub_vw_large is to sub_vw as add_vw_large is to add_vw.
fn sub_vw_large(z: &mut [Word], x: &[Word], y: Word) -> Word {
    let mut c = y;

    let n = z.len().min(x.len());
    // The comment near the top of this file discusses this for loop condition.
    for i in 0..n {
        if c == 0 {
            z[i..n].copy_from_slice(&x[i..n]);
            return c;
        }

        let (zi, cc) = bits::sub(x[i], c, 0);
        z[i] = zi;
        c = cc;
    }

    c
}
