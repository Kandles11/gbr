use std::fs;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    //open the file
    let mut file = fs::File::open("../cpu_instrs.gb")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    println!("File size: {} bytes", buffer.len());
    println!("First 10 bytes: {:?}", &buffer[0..10]);
    println!("First 10 bytes as hex: {:?}", &buffer[0..10].iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>());

    let title_bytes = &buffer[0x0134..0x0144];
    let _manufacturer_bytes = &buffer[0x013F..0x0143];
    let _cgb_flag_bytes = &buffer[0x0143..0x0144];
    let licensee_bytes = &buffer[0x0144..0x0146];
    let _sgb_flag_bytes = &buffer[0x0146..0x0147];
    let _cart_type_bytes = &buffer[0x0147..0x0148];
    let _rom_size_bytes = &buffer[0x0148..0x0149];
    let _ram_size_bytes = &buffer[0x0149..0x014A];
    let _dest_code_bytes = &buffer[0x014A..0x014B];
    let _old_licensee_bytes = &buffer[0x014B..0x014C];
    let _rom_version_bytes = &buffer[0x014C..0x014D];
    let _header_checksum_bytes = &buffer[0x014D..0x014E];
    let _global_checksum_bytes = &buffer[0x014E..0x0150];

    let title_str = String::from_utf8_lossy(&title_bytes);
    println!("Title: {:?}", title_str);
    println!("Licensee: {:?}", String::from_utf8_lossy(&licensee_bytes));

    Ok(())
}
