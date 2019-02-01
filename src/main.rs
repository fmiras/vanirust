extern crate ethkey;
extern crate hex;
extern crate rand;

use ethkey::{Error, KeyPair, Secret};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
  let random32char: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
  let secret = Secret::from_slice(random32char.as_bytes());
  println!("New secret: {:? }", secret);
  let kp = KeyPair::from_secret(secret.unwrap());
  println!("New public: {:? }", kp.unwrap().address());
}
