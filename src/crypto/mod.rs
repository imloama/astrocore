#![allow(dead_code)]

mod error;
mod keypair;
mod strkey;

// pub use self::ecdh::{Curve25519Public, Curve25519Secret};
pub use self::keypair::from_secret_seed;
