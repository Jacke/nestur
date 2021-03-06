use super::{Cartridge, Mapper, Mirror};

pub struct Nrom {
    cart: Cartridge,
}

impl Nrom {
    pub fn new(cart: Cartridge) -> Self {
        Nrom{
            cart: cart,
        }
    }
}

impl Mapper for Nrom {
    fn read(&mut self, address: usize) -> u8 {
        let cl = self.cart.chr_rom.len();
        let pl = self.cart.prg_rom.len();
        let addr = address % 0x4000;
        match address {
            0x0000..=0x1FFF => {
                if cl > 0 {
                    self.cart.chr_rom[cl-1][address]
                } else {
                    0
                }
            },
            0x8000..=0xBFFF => {
                self.cart.prg_rom[0][addr]
            },
            0xC000..=0xFFFF => {
                self.cart.prg_rom[pl-1][addr]
            },
            _ => {println!("bad address read from NROM mapper: 0x{:X}", address); 0},
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        let cl = self.cart.chr_rom.len();
        let pl = self.cart.prg_rom.len();
        let addr = address % 0x4000;
        match address {
            0x0000..=0x1FFF => {
                if cl > 0 {
                    self.cart.chr_rom[cl-1][address] = value;
                }
            },
            0x8000..=0xBFFF => self.cart.prg_rom[0][addr] = value,
            0xC000..=0xFFFF => self.cart.prg_rom[pl-1][addr] = value,
            _ => println!("bad address written to NROM mapper: 0x{:X}", address),
        }
    }

    fn get_mirroring(&mut self) -> Mirror {
        self.cart.mirroring
    }

    fn load_battery_backed_ram(&mut self) {}
    fn save_battery_backed_ram(&self) {}
    fn clock(&mut self) {}
    fn check_irq(&mut self) -> bool {false}
}
