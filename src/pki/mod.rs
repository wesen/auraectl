mod ssh;

use ed25519_dalek::Keypair;
use rand_07::rngs::OsRng;
use ssh_key::public::Ed25519PublicKey;
use std::error::Error;

pub fn generate_keypair() -> Result<(), Box<dyn Error>> {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public;
    let ssh_ed25519_key: Ed25519PublicKey = keypair.public.into();
    let ssh_public_key = ssh_key::PublicKey::from(ssh_ed25519_key);
    let ssh_public_key = ssh_public_key.to_openssh()?;
    println!("public_key: {}", ssh_public_key);
    todo!()
}
