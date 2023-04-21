pub fn read(buf: &mut [u8]) -> Result<(), String> {
  getrandom::getrandom(buf).map_err(|err| err.to_string())
}
