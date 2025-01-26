use std::env;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use sequoia_openpgp as openpgp;

use openpgp::parse::{Parse, PacketParser, PacketParserResult};
use openpgp::Packet;
use sequoia_openpgp::serialize::MarshalInto;

fn main() -> openpgp::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow::anyhow!("Usage: {} <encrypted.pgp>", args[0]));
    }

    let encrypted_data = std::fs::read(&args[1])?;
    let mut ppr = PacketParser::from_bytes(&encrypted_data)?;

    while let PacketParserResult::Some(mut pp) = ppr {
        println!("{:?}", pp.packet);
        if let Packet::PKESK(pkesk) = &pp.packet {
            println!("{:?}", pkesk.esk().to_vec());
            let size = pkesk.esk().to_vec()?.len();
            File::create("pkesk.bin")?.write_all(&pkesk.esk().to_vec().unwrap()[2..])?;
            // let mut content = Vec::new();
            // pp.read_to_end(&mut content)?;
            // println!("Full content length: {}", content.len());
            // File::create("pkesk.bin")?.write_all(&pp.packet.)?;
            // if let openpgp::Packet::PKESK(pkesk) = packet {
            //     println!("# off=0 ctb=85 tag=1 hlen=3 plen=524");
            //     println!(":pubkey enc packet: version {}", pkesk.version());
            //     println!("\tkeyid: {}", pkesk.recipient());
            //     println!("\tpubkey algo: {}", pkesk.pk_algo());

        }
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
            println!("openssl enc -aes-256-cfb -d -K $KEY -iv {} -in encrypted.bin", hex_dump(&content[1..17]));
            // println!("  -K $KEY \\");
            // println!("  -iv {} \\", hex_dump(&content[3..19]));
            // println!("  -in encrypted.bin -out decrypted.bin");
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