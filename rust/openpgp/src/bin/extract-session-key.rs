use std::env;
use sequoia_openpgp as openpgp;

use openpgp::cert::prelude::*;
use openpgp::parse::{Parse, stream::*};
use openpgp::policy::StandardPolicy;
use anyhow::anyhow;

fn main() -> openpgp::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(anyhow!("Usage: {} <secret-key.pgp> <encrypted.pgp>", args[0]));
    }

    let policy = &StandardPolicy::new();
    
    // Read the secret key
    let cert = Cert::from_file(&args[1])?;
    
    // Read encrypted message
    let encrypted_data = std::fs::read(&args[2])?;
    
    // Create decryption helper
    let helper = Helper { cert: &cert, policy };
    let mut _decryptor = DecryptorBuilder::from_bytes(&encrypted_data)?
        .with_policy(policy, None, helper)?;

    Ok(())
}

struct Helper<'a> {
    cert: &'a openpgp::Cert,
    policy: &'a dyn openpgp::policy::Policy,
}

impl<'a> VerificationHelper for Helper<'a> {
    fn get_certs(&mut self, _ids: &[openpgp::KeyHandle]) -> openpgp::Result<Vec<openpgp::Cert>> {
        Ok(Vec::new())
    }
    fn check(&mut self, _: MessageStructure) -> openpgp::Result<()> { Ok(()) }
}

impl<'a> DecryptionHelper for Helper<'a> {
    fn decrypt<D>(&mut self,
                  pkesks: &[openpgp::packet::PKESK],
                  _skesks: &[openpgp::packet::SKESK],
                  sym_algo: Option<openpgp::types::SymmetricAlgorithm>,
                  mut decrypt: D)
                  -> openpgp::Result<Option<openpgp::Fingerprint>>
                  where D: FnMut(openpgp::types::SymmetricAlgorithm,
                    &openpgp::crypto::SessionKey) -> bool
  {
        let key = self.cert.keys().unencrypted_secret()
            .with_policy(self.policy, None)
            .for_transport_encryption()
            .next()
            .unwrap()
            .key()
            .clone();

        let mut pair = key.into_keypair()?;
        if let Some(decrypted) = pkesks[0].decrypt(&mut pair, sym_algo) {
            println!("Decrypted type: {:?}", decrypted);
            let (algo, session_key) = decrypted;
            println!("Algorithm: {:?}", algo);
            println!("export KEY={}", session_key.as_ref().iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(""));
            decrypt(algo, &session_key);
        }

        Ok(None)
    }
} 