use std::fs;
use std::io::{self, Read};

// memory map
// 0x0000 - 0x3FFF: ROM Bank 0 (32KB)
// 0x4000 - 0x7FFF: ROM Bank 1 (switchable, 32KB)
// 0x8000 - 0x9FFF: Video RAM (8KB)
// 0xA000 - 0xBFFF: External RAM (switchable, 8KB)
// 0xC000 - 0xCFFF: Work RAM Bank 0 (4KB)
// 0xD000 - 0xDFFF: Work RAM Bank 1 (switchable, 4KB)
// 0xE000 - 0xFDFF: Echo RAM (4KB)
// 0xFE00 - 0xFE9F: OAM (160 bytes)
// 0xFEA0 - 0xFEFF: Unusable
// 0xFF00 - 0xFF7F: I/O Ports
// 0xFF80 - 0xFFFE: High RAM (HRAM, 127 bytes)
// 0xFFFF: Interrupt Enable Register

struct Cartridge {
    rom: Vec<u8>, // 32KB
    title: String
}

struct Bus {
    cartridge: Cartridge,
    vram: [u8; 0x2000],
    wram: [u8; 0x2000],
    cartridge_ram: [u8; 0x2000],
    echo_ram: [u8; 0x2000],
    oam: [u8; 0xA0],
    io_ports: [u8; 0x80],
    hram: [u8; 0x7F],
    ie_register: u8,
}

impl Cartridge {
    pub fn read(&self, addr: u16) -> u8 {
        if addr < 0x8000 {
            self.rom[addr as usize]
        } else {
            0xFF
        }
    }

    pub fn read_cartridge_file(&mut self, filepath: &str) -> io::Result<()> {
        let mut cartridge_file = fs::File::open(filepath)?;
        let mut buffer = Vec::new();
        cartridge_file.read_to_end(&mut buffer)?;
        self.rom = buffer;
        self.title = String::from_utf8_lossy(&self.rom[0x0134..0x0144]).to_string();
        Ok(())
    }
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read(addr), // Cartridge ROM access
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.cartridge_ram[(addr - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize], 
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF, // Not usable
            0xFF00..=0xFF7F => self.io_ports[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => {}
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.cartridge_ram[(addr - 0xA000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {}, // Not usable
            0xFF00..=0xFF7F => self.io_ports[(addr - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.ie_register = value,
            _ => {},
        }
    }
}

fn main() -> io::Result<()> {
    let mut cartridge = Cartridge {
        rom: Vec::new(),
        title: String::new(),
    };

    cartridge.read_cartridge_file("../cpu_instrs.gb")?;

    println!("File size: {} bytes", cartridge.rom.len());
    println!("First 10 bytes: {:?}", &cartridge.rom[0..10]);
    println!("First 10 bytes as hex: {:?}", &cartridge.rom[0..10].iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>());

    let _manufacturer_bytes = &cartridge.rom[0x013F..0x0143];
    let _cgb_flag_bytes = &cartridge.rom[0x0143..0x0144];
    let licensee_bytes = &cartridge.rom[0x0144..0x0146];
    let _sgb_flag_bytes = &cartridge.rom[0x0146..0x0147];
    let _cart_type_bytes = &cartridge.rom[0x0147..0x0148];
    let _rom_size_bytes = &cartridge.rom[0x0148..0x0149];
    let _ram_size_bytes = &cartridge.rom[0x0149..0x014A];
    let _dest_code_bytes = &cartridge.rom[0x014A..0x014B];
    let _old_licensee_bytes = &cartridge.rom[0x014B..0x014C];
    let _rom_version_bytes = &cartridge.rom[0x014C..0x014D];
    let _header_checksum_bytes = &cartridge.rom[0x014D..0x014E];
    let _global_checksum_bytes = &cartridge.rom[0x014E..0x0150];

    println!("Title: {:?}", cartridge.title);
    println!("Licensee: {:?}", String::from_utf8_lossy(&licensee_bytes));

    let mut bus = Bus {
        cartridge: cartridge,
        vram: [0; 0x2000],
        wram: [0; 0x2000],
        cartridge_ram: [0; 0x2000],
        echo_ram: [0; 0x2000],
        oam: [0; 0xA0],
        io_ports: [0; 0x80],
        hram: [0; 0x7F],
        ie_register: 0,
    };
    
    Ok(())
}