// cargo-deps: sequoia-openpgp = "*"
// Run: cargo script simple-encrypt-decrypt.rs
use std::io::Write;
use std::io::Read;
use std::fs::File;
use sequoia_openpgp as openpgp;

use openpgp::cert::prelude::*;
use openpgp::serialize::stream::*;
use openpgp::parse::{Parse, stream::*, PacketParser, PacketParserResult};
use openpgp::policy::StandardPolicy;
use openpgp::serialize::Serialize;
use openpgp::Packet;

const MESSAGE: &str = "Hello there!";
const ENCRYPTED_FILE: &str = "message.pgp";
const PUBLIC_KEY_FILE: &str = "public.pgp";
const SECRET_KEY_FILE: &str = "secret.pgp";

fn main() -> openpgp::Result<()> {
    let policy = &StandardPolicy::new();

    // Generate a key for encryption
    let (cert, _revocation) = CertBuilder::new()
        .add_userid("someone@example.org")
        .add_transport_encryption_subkey()
        .generate()?;

    // Export the public key in ASCII armor format
    {
        let mut public_key_file = File::create(PUBLIC_KEY_FILE)?;
        cert.armored().serialize(&mut public_key_file)?;
    }
    println!("Public key exported to {}", PUBLIC_KEY_FILE);

    // Export the secret key in ASCII armor format
    {
        let mut secret_key_file = File::create(SECRET_KEY_FILE)?;
        cert.as_tsk().armored().serialize(&mut secret_key_file)?;
    }
    println!("Secret key exported to {}", SECRET_KEY_FILE);

    // Encrypt the message and save to file
    {
        let mut encrypted_file = File::create(ENCRYPTED_FILE)?;
        let message = Message::new(&mut encrypted_file);
        let message = Armorer::new(message).build()?;
        let encryptor = Encryptor::for_recipients(message, vec![cert.keys()
            .with_policy(policy, None)
            .supported()
            .alive()
            .revoked(false)
            .for_transport_encryption()
            .next()
            .unwrap()])
            .build()?;
        let mut writer = LiteralWriter::new(encryptor).build()?;
        writer.write_all(MESSAGE.as_bytes())?;
        writer.finalize()?;
    }

    println!("\nMessage encrypted and saved to {}", ENCRYPTED_FILE);

    // Parse and print packet structure
    println!("\nPacket structure of encrypted message:");
    let encrypted_data = std::fs::read(ENCRYPTED_FILE)?;
    let mut ppr = PacketParser::from_bytes(&encrypted_data)?;

    while let PacketParserResult::Some(mut pp) = ppr {
        println!("Found packet type: {}", packet_type_name(&pp.packet));
        
        if let Packet::PKESK(pkesk) = &pp.packet {
            println!("PKESK details: {:?}", pkesk);
        }
        
        if let Packet::SEIP(pack) = &pp.packet {
            println!("SEIP packet: {:?}", pack);
            let mut content = Vec::new();
            pp.read_to_end(&mut content)?;
            println!("SEIP version: {}", content[0]);
            println!("SEIP prefix: {:x?}", &content[1..17]);  // Now in hex
            println!("SEIP main content: {:x?}", &content[17..content.len()-20]);
            println!("SEIP hash: {:x?}", &content[content.len()-20..]);
            
            // Add hex dump of decrypted content
            println!("\nDecrypted content hex dump:");
            for (i, chunk) in content.chunks(16).enumerate() {
                print!("{:08x}  ", i * 16);
                for byte in chunk {
                    print!("{:02x} ", byte);
                }
                print!("  ");
                // Print ASCII representation
                for byte in chunk {
                    if byte.is_ascii_graphic() || *byte == b' ' {
                        print!("{}", *byte as char);
                    } else {
                        print!(".");
                    }
                }
                println!();
            }

            println!("\nAnalyzing packet header:");
            let header = &content[16..21];  // The 5 header bytes
            println!("Header bytes: {:02x?}", header);

            // Parse OpenPGP packet header
            let tag = header[0];
            let length_type = header[1];
            println!("Tag byte: {:02x} ({:08b})", tag, tag);
            println!("Length type byte: {:02x} ({:08b})", length_type, length_type);

            // Parse length - more carefully this time
            if length_type & 0x80 != 0 {  // Multi-byte length
                if length_type & 0x40 != 0 {  // Format 11xxxxxx
                    let len_bytes = (length_type & 0x3f) as usize;
                    println!("New format length field ({} bytes)", len_bytes);
                    if len_bytes <= 3 && len_bytes <= (header.len() - 2) {  // Safety check
                        let mut length = 0u32;
                        for &b in &header[2..2+len_bytes] {
                            length = (length << 8) | b as u32;
                        }
                        println!("Decoded length: {}", length);
                    }
                } else {  // Format 10xxxxxx
                    let len_bytes = 1 << (length_type & 0x1f);
                    println!("Old format length field ({} bytes)", len_bytes);
                }
            } else {  // Single byte length
                println!("Single byte length: {}", length_type);
            }
        }
        
        ppr = pp.recurse()?.1;
    }

    // Read and decrypt the message
    let mut decrypted = Vec::new();
    {
        let helper = Helper { cert: &cert, policy };
        let mut decryptor = DecryptorBuilder::from_bytes(&encrypted_data)?
            .with_policy(policy, None, helper)?;
        std::io::copy(&mut decryptor, &mut decrypted)?;
    }

    println!("\nDecrypted message:");
    println!("{}", String::from_utf8_lossy(&decrypted));

    Ok(())
}

fn packet_type_name(packet: &Packet) -> String {
    match packet {
        Packet::PublicKey(_) => "Public Key".into(),
        Packet::PublicSubkey(_) => "Public Subkey".into(),
        Packet::SecretKey(_) => "Secret Key".into(),
        Packet::SecretSubkey(_) => "Secret Subkey".into(),
        Packet::Signature(_) => "Signature".into(),
        Packet::OnePassSig(_) => "One Pass Signature".into(),
        Packet::UserID(_) => "User ID".into(),
        Packet::UserAttribute(_) => "User Attribute".into(),
        Packet::Marker(_) => "Marker".into(),
        Packet::Trust(_) => "Trust".into(),
        Packet::PKESK(_) => "Public-Key Encrypted Session Key".into(),
        Packet::SKESK(_) => "Symmetric-Key Encrypted Session Key".into(),
        Packet::SEIP(_) => "Symmetrically Encrypted Integrity Protected Data".into(),
        Packet::MDC(_) => "Modification Detection Code".into(),
        Packet::Literal(_) => "Literal Data".into(),
        Packet::CompressedData(_) => "Compressed Data".into(),
        Packet::Unknown(_) => "Unknown".into(),
        _ => "Other Packet Type".into(),
    }
}

struct Helper<'a> {
    cert: &'a openpgp::Cert,
    policy: &'a dyn openpgp::policy::Policy,
}

impl<'a> VerificationHelper for Helper<'a> {
    fn get_certs(&mut self, _ids: &[openpgp::KeyHandle])
        -> openpgp::Result<Vec<openpgp::Cert>> {
        Ok(Vec::new())
    }

    fn check(&mut self, _structure: MessageStructure) -> openpgp::Result<()> {
        Ok(())
    }
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
        println!("Decryption called with:");
        println!("  - PKESKs: {:?}", pkesks);
        println!("  - Symmetric algo: {:?}", sym_algo);

        let key = self.cert.keys().unencrypted_secret()
            .with_policy(self.policy, None)
            .for_transport_encryption()
            .next()
            .unwrap()
            .key()
            .clone();

        println!("Using key: {:?}", key);

        let mut pair = key.into_keypair()?;
        let result = pkesks[0].decrypt(&mut pair, sym_algo);
        println!("Decryption result: {:?}", result);
        
        result.map(|(algo, session_key)| {
            println!("Session key bytes: {:?}", session_key.as_ref());
            decrypt(algo, &session_key)
        });

        Ok(None)
    }
} 