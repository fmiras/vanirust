extern crate ethereum_types;
extern crate memzero;

extern crate rustc_hex;
extern crate secp256k1;
extern crate tiny_keccak;

#[macro_use]
extern crate lazy_static;

mod error;
mod keccak;
mod keypair;
mod secret;

pub use self::error::Error;
pub use self::keypair::{public_to_address, KeyPair};
pub use self::secret::Secret;

use ethereum_types::H256;

pub use ethereum_types::{Address, Public};
pub type Message = H256;

lazy_static! {
  pub static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

/// Uninstantiatable error type for infallible generators.
#[derive(Debug)]
pub enum Void {}

/// Generates new keypair.
pub trait Generator {
  type Error;

  /// Should be called to generate new keypair.
  fn generate(&mut self) -> Result<KeyPair, Self::Error>;
}
