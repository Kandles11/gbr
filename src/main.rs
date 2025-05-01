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

pub fn decode_opcode(opcode: u8) -> Instruction {
    match opcode {
        0x40 => Instruction::LD_8BIT(Register::B, Register::B),
        0x41 => Instruction::LD_8BIT(Register::B, Register::C),
        0x42 => Instruction::LD_8BIT(Register::B, Register::D),
        0x43 => Instruction::LD_8BIT(Register::B, Register::E),
        0x44 => Instruction::LD_8BIT(Register::B, Register::H),
        0x45 => Instruction::LD_8BIT(Register::B, Register::L),

        0x47 => Instruction::LD_8BIT(Register::B, Register::A),
        0x48 => Instruction::LD_8BIT(Register::C, Register::B),
        0x49 => Instruction::LD_8BIT(Register::C, Register::C),
        0x4A => Instruction::LD_8BIT(Register::C, Register::D),
        0x4B => Instruction::LD_8BIT(Register::C, Register::E),
        0x4C => Instruction::LD_8BIT(Register::C, Register::H),
        0x4D => Instruction::LD_8BIT(Register::C, Register::L),

        0x4F => Instruction::LD_8BIT(Register::C, Register::A),
        0x50 => Instruction::LD_8BIT(Register::D, Register::B),
        0x51 => Instruction::LD_8BIT(Register::D, Register::C),
        0x52 => Instruction::LD_8BIT(Register::D, Register::D),
        0x53 => Instruction::LD_8BIT(Register::D, Register::E),
        0x54 => Instruction::LD_8BIT(Register::D, Register::H),
        0x55 => Instruction::LD_8BIT(Register::D, Register::L),

        0x57 => Instruction::LD_8BIT(Register::D, Register::A),
        0x58 => Instruction::LD_8BIT(Register::E, Register::B),
        0x59 => Instruction::LD_8BIT(Register::E, Register::C),
        0x5A => Instruction::LD_8BIT(Register::E, Register::D),
        0x5B => Instruction::LD_8BIT(Register::E, Register::E),
        0x5C => Instruction::LD_8BIT(Register::E, Register::H),
        0x5D => Instruction::LD_8BIT(Register::E, Register::L),

        0x5F => Instruction::LD_8BIT(Register::E, Register::A),

        _=> panic!("Unknown opcode: 0x{:02X}", opcode),
    }
}

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum Instruction {
    LD_8BIT(Register, Register),
}

struct CPU {
    //registers
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    sp: u16,
    pc: u16,

    //flags
    z: bool, // Zero flag
    n: bool, // Subtract flag
    half_carry: bool, // Half carry flag
    carry: bool, // Carry flag
}

impl CPU {
    pub fn read_register(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            _ => 0xFF, // Invalid register
        }
    }

    pub fn write_register(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            _ => {}
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LD_8BIT(dest, src) => {
                // Load the value from src register to dest register
                let value = self.read_register(src);
                self.write_register(dest, value);
            }
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

    let mut gb_cpu = CPU {
        a: 0,
        b: 0,
        c: 5,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        sp: 0xFFFE,
        pc: 0x0100,
        z: false,
        n: false,
        half_carry: false,
        carry: false,
    };
    println!("Initial B state: {:?}", gb_cpu.read_register(Register::B));
    println!("Initial C state: {:?}", gb_cpu.read_register(Register::C));

    let opcode = 0x41;
    let decoded_instruction = decode_opcode(opcode);
    gb_cpu.execute_instruction(decoded_instruction);

    println!("B state after LD B, C: {:?}", gb_cpu.read_register(Register::B));
    println!("C state after LD B, C: {:?}", gb_cpu.read_register(Register::C));
    
    Ok(())
}