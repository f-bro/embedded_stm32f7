// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "FLASH" ]
# [ repr ( C ) ]
pub struct Flash {
    # [ doc = "0x00 - Flash access control register" ]
    pub acr: volatile::ReadWrite<Acr>,
    # [ doc = "0x04 - Flash key register" ]
    pub keyr: volatile::WriteOnly<Keyr>,
    # [ doc = "0x08 - Flash option key register" ]
    pub optkeyr: volatile::WriteOnly<Optkeyr>,
    # [ doc = "0x0c - Status register" ]
    pub sr: volatile::ReadWrite<Sr>,
    # [ doc = "0x10 - Control register" ]
    pub cr: volatile::ReadWrite<Cr>,
    # [ doc = "0x14 - Flash option control register" ]
    pub optcr: volatile::ReadWrite<Optcr>,
    # [ doc = "0x18 - Flash option control register 1" ]
    pub optcr1: volatile::ReadWrite<Optcr1>,
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Acr {
    bits: u32,
}

impl Acr {
    # [ doc = "Bits 0:2 - Latency" ]
    pub fn latency(&self) -> u8 {
        self.bits.get_range(0u8..3u8) as u8
    }
    # [ doc = "Bit 8 - Prefetch enable" ]
    pub fn prften(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 9 - Instruction cache enable" ]
    pub fn icen(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 10 - Data cache enable" ]
    pub fn dcen(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 12 - Data cache reset" ]
    pub fn dcrst(&self) -> bool {
        self.bits.get_bit(12u8)
    }
}

impl Default for Acr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Acr { bits: 0u32 }
    }
}

impl Acr {
    # [ doc = "Bits 0:2 - Latency" ]
    pub fn set_latency(&mut self, value: u8) {
        self.bits.set_range(0u8..3u8, value as u32);
    }
    # [ doc = "Bit 8 - Prefetch enable" ]
    pub fn set_prften(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bit 9 - Instruction cache enable" ]
    pub fn set_icen(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 10 - Data cache enable" ]
    pub fn set_dcen(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 11 - Instruction cache reset" ]
    pub fn set_icrst(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 12 - Data cache reset" ]
    pub fn set_dcrst(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Keyr {
    bits: u32,
}

impl Keyr {
    # [ doc = "Bits 0:31 - FPEC key" ]
    pub fn key(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Keyr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Keyr { bits: 0u32 }
    }
}

impl Keyr {
    # [ doc = "Bits 0:31 - FPEC key" ]
    pub fn set_key(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Optkeyr {
    bits: u32,
}

impl Optkeyr {
    # [ doc = "Bits 0:31 - Option byte key" ]
    pub fn optkey(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Optkeyr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Optkeyr { bits: 0u32 }
    }
}

impl Optkeyr {
    # [ doc = "Bits 0:31 - Option byte key" ]
    pub fn set_optkey(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Sr {
    bits: u32,
}

impl Sr {
    # [ doc = "Bit 0 - End of operation" ]
    pub fn eop(&self) -> bool {
        self.bits.get_bit(0u8)
    }
    # [ doc = "Bit 1 - Operation error" ]
    pub fn operr(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 4 - Write protection error" ]
    pub fn wrperr(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 5 - Programming alignment error" ]
    pub fn pgaerr(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 6 - Programming parallelism error" ]
    pub fn pgperr(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 7 - Programming sequence error" ]
    pub fn pgserr(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bit 16 - Busy" ]
    pub fn bsy(&self) -> bool {
        self.bits.get_bit(16u8)
    }
}

impl Default for Sr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Sr { bits: 0u32 }
    }
}

impl Sr {
    # [ doc = "Bit 0 - End of operation" ]
    pub fn set_eop(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
    # [ doc = "Bit 1 - Operation error" ]
    pub fn set_operr(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 4 - Write protection error" ]
    pub fn set_wrperr(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 5 - Programming alignment error" ]
    pub fn set_pgaerr(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 6 - Programming parallelism error" ]
    pub fn set_pgperr(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 7 - Programming sequence error" ]
    pub fn set_pgserr(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Cr {
    bits: u32,
}

impl Cr {
    # [ doc = "Bit 0 - Programming" ]
    pub fn pg(&self) -> bool {
        self.bits.get_bit(0u8)
    }
    # [ doc = "Bit 1 - Sector Erase" ]
    pub fn ser(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 2 - Mass Erase of sectors 0 to 11" ]
    pub fn mer(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bits 3:7 - Sector number" ]
    pub fn snb(&self) -> u8 {
        self.bits.get_range(3u8..8u8) as u8
    }
    # [ doc = "Bits 8:9 - Program size" ]
    pub fn psize(&self) -> u8 {
        self.bits.get_range(8u8..10u8) as u8
    }
    # [ doc = "Bit 15 - Mass Erase of sectors 12 to 23" ]
    pub fn mer1(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bit 16 - Start" ]
    pub fn strt(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bit 24 - End of operation interrupt enable" ]
    pub fn eopie(&self) -> bool {
        self.bits.get_bit(24u8)
    }
    # [ doc = "Bit 25 - Error interrupt enable" ]
    pub fn errie(&self) -> bool {
        self.bits.get_bit(25u8)
    }
    # [ doc = "Bit 31 - Lock" ]
    pub fn lock(&self) -> bool {
        self.bits.get_bit(31u8)
    }
}

impl Default for Cr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cr { bits: 2147483648u32 }
    }
}

impl Cr {
    # [ doc = "Bit 0 - Programming" ]
    pub fn set_pg(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
    # [ doc = "Bit 1 - Sector Erase" ]
    pub fn set_ser(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 2 - Mass Erase of sectors 0 to 11" ]
    pub fn set_mer(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bits 3:7 - Sector number" ]
    pub fn set_snb(&mut self, value: u8) {
        self.bits.set_range(3u8..8u8, value as u32);
    }
    # [ doc = "Bits 8:9 - Program size" ]
    pub fn set_psize(&mut self, value: u8) {
        self.bits.set_range(8u8..10u8, value as u32);
    }
    # [ doc = "Bit 15 - Mass Erase of sectors 12 to 23" ]
    pub fn set_mer1(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bit 16 - Start" ]
    pub fn set_strt(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bit 24 - End of operation interrupt enable" ]
    pub fn set_eopie(&mut self, value: bool) {
        self.bits.set_bit(24u8, value);
    }
    # [ doc = "Bit 25 - Error interrupt enable" ]
    pub fn set_errie(&mut self, value: bool) {
        self.bits.set_bit(25u8, value);
    }
    # [ doc = "Bit 31 - Lock" ]
    pub fn set_lock(&mut self, value: bool) {
        self.bits.set_bit(31u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Optcr {
    bits: u32,
}

impl Optcr {
    # [ doc = "Bit 0 - Option lock" ]
    pub fn optlock(&self) -> bool {
        self.bits.get_bit(0u8)
    }
    # [ doc = "Bit 1 - Option start" ]
    pub fn optstrt(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bits 2:3 - BOR reset Level" ]
    pub fn bor_lev(&self) -> u8 {
        self.bits.get_range(2u8..4u8) as u8
    }
    # [ doc = "Bit 5 - WDG_SW User option bytes" ]
    pub fn wdg_sw(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 6 - nRST_STOP User option bytes" ]
    pub fn n_rst_stop(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 7 - nRST_STDBY User option bytes" ]
    pub fn n_rst_stdby(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 8:15 - Read protect" ]
    pub fn rdp(&self) -> u8 {
        self.bits.get_range(8u8..16u8) as u8
    }
    # [ doc = "Bits 16:27 - Not write protect" ]
    pub fn n_wrp(&self) -> u16 {
        self.bits.get_range(16u8..28u8) as u16
    }
}

impl Default for Optcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Optcr { bits: 268413677u32 }
    }
}

impl Optcr {
    # [ doc = "Bit 0 - Option lock" ]
    pub fn set_optlock(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
    # [ doc = "Bit 1 - Option start" ]
    pub fn set_optstrt(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bits 2:3 - BOR reset Level" ]
    pub fn set_bor_lev(&mut self, value: u8) {
        self.bits.set_range(2u8..4u8, value as u32);
    }
    # [ doc = "Bit 5 - WDG_SW User option bytes" ]
    pub fn set_wdg_sw(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 6 - nRST_STOP User option bytes" ]
    pub fn set_n_rst_stop(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 7 - nRST_STDBY User option bytes" ]
    pub fn set_n_rst_stdby(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 8:15 - Read protect" ]
    pub fn set_rdp(&mut self, value: u8) {
        self.bits.set_range(8u8..16u8, value as u32);
    }
    # [ doc = "Bits 16:27 - Not write protect" ]
    pub fn set_n_wrp(&mut self, value: u16) {
        self.bits.set_range(16u8..28u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Optcr1 {
    bits: u32,
}

impl Optcr1 {
    # [ doc = "Bits 16:27 - Not write protect" ]
    pub fn n_wrp(&self) -> u16 {
        self.bits.get_range(16u8..28u8) as u16
    }
}

impl Default for Optcr1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Optcr1 { bits: 268369920u32 }
    }
}

impl Optcr1 {
    # [ doc = "Bits 16:27 - Not write protect" ]
    pub fn set_n_wrp(&mut self, value: u16) {
        self.bits.set_range(16u8..28u8, value as u32);
    }
}
