use crypto::digest::Digest;
use crypto::sha3::Sha3;

use rand;
use rand::{Rng, SeedableRng};

fn main() {
  let s: &str = "hello";
  let u: [u8; 2] = [66, 66];
  for i in s.as_bytes() {

  }
  for i in u.as_slice() {

  }
  let v = s.as_bytes().iter().chain(u.as_slice().iter()).collect::<Vec<_>>();
  println!("{:?}", v);
}
