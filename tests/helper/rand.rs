use std::io::Read;

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

pub struct Reader(StdRng);

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.fill_bytes(buf);
        Ok(buf.len())
    }
}

impl Reader {
    pub fn new(s: u64) -> Self {
        Self(StdRng::seed_from_u64(s))
    }
}

pub fn read(buf: &mut [u8]) -> Result<(), String> {
    getrandom::getrandom(buf).map_err(|err| err.to_string())
}
