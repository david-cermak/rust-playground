use std::env;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use sequoia_openpgp as openpgp;

use openpgp::parse::{Parse, PacketParser, PacketParserResult};
use openpgp::Packet;

fn main() -> openpgp::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow::anyhow!("Usage: {} <encrypted.pgp>", args[0]));
    }

    let encrypted_data = std::fs::read(&args[1])?;
    let mut ppr = PacketParser::from_bytes(&encrypted_data)?;

    while let PacketParserResult::Some(mut pp) = ppr {
        if let Packet::SEIP(_) = &pp.packet {
            let mut content = Vec::new();
            pp.read_to_end(&mut content)?;
            
            println!("Full content length: {}", content.len());
            println!("Content structure:");
            println!("  Version (1 byte):  {:02x}", content[0]);
            println!("  Prefix (16 bytes): {}", hex_dump(&content[1..17]));
            println!("  Data until hash:   {}", hex_dump(&content[17..content.len()-20]));
            println!("  Hash (20 bytes):   {}", hex_dump(&content[content.len()-20..]));
            
            // Write the full encrypted data (including prefix)
            File::create("encrypted.bin")?.write_all(&content[0..content.len()])?;
            
            println!("\nOpenSSL command:");
            println!("openssl enc -aes-256-cfb -d \\");
            println!("  -K $KEY \\");
            println!("  -iv {} \\", hex_dump(&content[1..17]));
            println!("  -in encrypted.bin -out decrypted.bin");
        }
        ppr = pp.recurse()?.1;
    }

    Ok(())
}

fn hex_dump(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("")
} 