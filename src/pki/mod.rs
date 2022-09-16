mod ssh;

use crate::Settings;
use ed25519_dalek::Keypair;
use rand_07::rngs::OsRng;
use ssh_key::private::Ed25519Keypair;
use ssh_key::LineEnding;
use std::error::Error;

pub fn generate_keypair(x: &Settings) -> Result<(), Box<dyn Error>> {
    let mut csprng = OsRng {};
    let keypair: Ed25519Keypair = Keypair::generate(&mut csprng).into();
    let ssh_public_key = ssh_key::PublicKey::from(keypair.public).to_openssh()?;
    let ssh_private_key = ssh_key::PrivateKey::from(keypair).to_openssh(LineEnding::LF)?;
    println!("public_key: {}", ssh_public_key);
    println!("private_key: {}", *ssh_private_key);
    todo!()
}
