// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "General purpose timers" ]
# [ repr ( C ) ]
pub struct Tim4 {
    # [ doc = "0x00 - control register 1" ]
    pub cr1: volatile::ReadWrite<Cr1>,
    # [ doc = "0x04 - control register 2" ]
    pub cr2: volatile::ReadWrite<Cr2>,
    # [ doc = "0x08 - slave mode control register" ]
    pub smcr: volatile::ReadWrite<Smcr>,
    # [ doc = "0x0c - DMA/Interrupt enable register" ]
    pub dier: volatile::ReadWrite<Dier>,
    # [ doc = "0x10 - status register" ]
    pub sr: volatile::ReadWrite<Sr>,
    # [ doc = "0x14 - event generation register" ]
    pub egr: volatile::WriteOnly<Egr>,
    # [ doc = "0x18 - capture/compare mode register 1 (output mode)" ]
    pub ccmr1_output: volatile::ReadWrite<Ccmr1Output>,
    # [ doc = "0x1c - capture/compare mode register 2 (output mode)" ]
    pub ccmr2_output: volatile::ReadWrite<Ccmr2Output>,
    # [ doc = "0x20 - capture/compare enable register" ]
    pub ccer: volatile::ReadWrite<Ccer>,
    # [ doc = "0x24 - counter" ]
    pub cnt: volatile::ReadWrite<Cnt>,
    # [ doc = "0x28 - prescaler" ]
    pub psc: volatile::ReadWrite<Psc>,
    # [ doc = "0x2c - auto-reload register" ]
    pub arr: volatile::ReadWrite<Arr>,
    _reserved0: [u8; 4usize],
    # [ doc = "0x34 - capture/compare register 1" ]
    pub ccr1: volatile::ReadWrite<Ccr1>,
    # [ doc = "0x38 - capture/compare register 2" ]
    pub ccr2: volatile::ReadWrite<Ccr2>,
    # [ doc = "0x3c - capture/compare register 3" ]
    pub ccr3: volatile::ReadWrite<Ccr3>,
    # [ doc = "0x40 - capture/compare register 4" ]
    pub ccr4: volatile::ReadWrite<Ccr4>,
    _reserved1: [u8; 4usize],
    # [ doc = "0x48 - DMA control register" ]
    pub dcr: volatile::ReadWrite<Dcr>,
    # [ doc = "0x4c - DMA address for full transfer" ]
    pub dmar: volatile::ReadWrite<Dmar>,
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Cr1 {
    bits: u32,
}

impl Cr1 {
    # [ doc = "Bits 8:9 - Clock division" ]
    pub fn ckd(&self) -> u8 {
        self.bits.get_range(8u8..10u8) as u8
    }
    # [ doc = "Bit 7 - Auto-reload preload enable" ]
    pub fn arpe(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 5:6 - Center-aligned mode selection" ]
    pub fn cms(&self) -> u8 {
        self.bits.get_range(5u8..7u8) as u8
    }
    # [ doc = "Bit 4 - Direction" ]
    pub fn dir(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - One-pulse mode" ]
    pub fn opm(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Update request source" ]
    pub fn urs(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Update disable" ]
    pub fn udis(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Counter enable" ]
    pub fn cen(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Cr1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cr1 { bits: 0u32 }
    }
}

impl Cr1 {
    # [ doc = "Bits 8:9 - Clock division" ]
    pub fn set_ckd(&mut self, value: u8) {
        self.bits.set_range(8u8..10u8, value as u32);
    }
    # [ doc = "Bit 7 - Auto-reload preload enable" ]
    pub fn set_arpe(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 5:6 - Center-aligned mode selection" ]
    pub fn set_cms(&mut self, value: u8) {
        self.bits.set_range(5u8..7u8, value as u32);
    }
    # [ doc = "Bit 4 - Direction" ]
    pub fn set_dir(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - One-pulse mode" ]
    pub fn set_opm(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Update request source" ]
    pub fn set_urs(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Update disable" ]
    pub fn set_udis(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Counter enable" ]
    pub fn set_cen(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Cr2 {
    bits: u32,
}

impl Cr2 {
    # [ doc = "Bit 7 - TI1 selection" ]
    pub fn ti1s(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 4:6 - Master mode selection" ]
    pub fn mms(&self) -> u8 {
        self.bits.get_range(4u8..7u8) as u8
    }
    # [ doc = "Bit 3 - Capture/compare DMA selection" ]
    pub fn ccds(&self) -> bool {
        self.bits.get_bit(3u8)
    }
}

impl Default for Cr2 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cr2 { bits: 0u32 }
    }
}

impl Cr2 {
    # [ doc = "Bit 7 - TI1 selection" ]
    pub fn set_ti1s(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 4:6 - Master mode selection" ]
    pub fn set_mms(&mut self, value: u8) {
        self.bits.set_range(4u8..7u8, value as u32);
    }
    # [ doc = "Bit 3 - Capture/compare DMA selection" ]
    pub fn set_ccds(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Smcr {
    bits: u32,
}

impl Smcr {
    # [ doc = "Bits 0:2 - Slave mode selection" ]
    pub fn sms(&self) -> u8 {
        self.bits.get_range(0u8..3u8) as u8
    }
    # [ doc = "Bits 4:6 - Trigger selection" ]
    pub fn ts(&self) -> u8 {
        self.bits.get_range(4u8..7u8) as u8
    }
    # [ doc = "Bit 7 - Master/Slave mode" ]
    pub fn msm(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 8:11 - External trigger filter" ]
    pub fn etf(&self) -> u8 {
        self.bits.get_range(8u8..12u8) as u8
    }
    # [ doc = "Bits 12:13 - External trigger prescaler" ]
    pub fn etps(&self) -> u8 {
        self.bits.get_range(12u8..14u8) as u8
    }
    # [ doc = "Bit 14 - External clock enable" ]
    pub fn ece(&self) -> bool {
        self.bits.get_bit(14u8)
    }
    # [ doc = "Bit 15 - External trigger polarity" ]
    pub fn etp(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bit 16 - Slave model selection - bit[3]" ]
    pub fn sms_3(&self) -> bool {
        self.bits.get_bit(16u8)
    }
}

impl Default for Smcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Smcr { bits: 0u32 }
    }
}

impl Smcr {
    # [ doc = "Bits 0:2 - Slave mode selection" ]
    pub fn set_sms(&mut self, value: u8) {
        self.bits.set_range(0u8..3u8, value as u32);
    }
    # [ doc = "Bits 4:6 - Trigger selection" ]
    pub fn set_ts(&mut self, value: u8) {
        self.bits.set_range(4u8..7u8, value as u32);
    }
    # [ doc = "Bit 7 - Master/Slave mode" ]
    pub fn set_msm(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 8:11 - External trigger filter" ]
    pub fn set_etf(&mut self, value: u8) {
        self.bits.set_range(8u8..12u8, value as u32);
    }
    # [ doc = "Bits 12:13 - External trigger prescaler" ]
    pub fn set_etps(&mut self, value: u8) {
        self.bits.set_range(12u8..14u8, value as u32);
    }
    # [ doc = "Bit 14 - External clock enable" ]
    pub fn set_ece(&mut self, value: bool) {
        self.bits.set_bit(14u8, value);
    }
    # [ doc = "Bit 15 - External trigger polarity" ]
    pub fn set_etp(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bit 16 - Slave model selection - bit[3]" ]
    pub fn set_sms_3(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Dier {
    bits: u32,
}

impl Dier {
    # [ doc = "Bit 14 - Trigger DMA request enable" ]
    pub fn tde(&self) -> bool {
        self.bits.get_bit(14u8)
    }
    # [ doc = "Bit 12 - Capture/Compare 4 DMA request enable" ]
    pub fn cc4de(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 11 - Capture/Compare 3 DMA request enable" ]
    pub fn cc3de(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Capture/Compare 2 DMA request enable" ]
    pub fn cc2de(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - Capture/Compare 1 DMA request enable" ]
    pub fn cc1de(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Update DMA request enable" ]
    pub fn ude(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 6 - Trigger interrupt enable" ]
    pub fn tie(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 4 - Capture/Compare 4 interrupt enable" ]
    pub fn cc4ie(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Capture/Compare 3 interrupt enable" ]
    pub fn cc3ie(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
    pub fn cc2ie(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
    pub fn cc1ie(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Update interrupt enable" ]
    pub fn uie(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Dier {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Dier { bits: 0u32 }
    }
}

impl Dier {
    # [ doc = "Bit 14 - Trigger DMA request enable" ]
    pub fn set_tde(&mut self, value: bool) {
        self.bits.set_bit(14u8, value);
    }
    # [ doc = "Bit 12 - Capture/Compare 4 DMA request enable" ]
    pub fn set_cc4de(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 11 - Capture/Compare 3 DMA request enable" ]
    pub fn set_cc3de(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Capture/Compare 2 DMA request enable" ]
    pub fn set_cc2de(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - Capture/Compare 1 DMA request enable" ]
    pub fn set_cc1de(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Update DMA request enable" ]
    pub fn set_ude(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bit 6 - Trigger interrupt enable" ]
    pub fn set_tie(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 4 - Capture/Compare 4 interrupt enable" ]
    pub fn set_cc4ie(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - Capture/Compare 3 interrupt enable" ]
    pub fn set_cc3ie(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
    pub fn set_cc2ie(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
    pub fn set_cc1ie(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Update interrupt enable" ]
    pub fn set_uie(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Sr {
    bits: u32,
}

impl Sr {
    # [ doc = "Bit 12 - Capture/Compare 4 overcapture flag" ]
    pub fn cc4of(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 11 - Capture/Compare 3 overcapture flag" ]
    pub fn cc3of(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
    pub fn cc2of(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
    pub fn cc1of(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 6 - Trigger interrupt flag" ]
    pub fn tif(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 4 - Capture/Compare 4 interrupt flag" ]
    pub fn cc4if(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Capture/Compare 3 interrupt flag" ]
    pub fn cc3if(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
    pub fn cc2if(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
    pub fn cc1if(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Update interrupt flag" ]
    pub fn uif(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Sr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Sr { bits: 0u32 }
    }
}

impl Sr {
    # [ doc = "Bit 12 - Capture/Compare 4 overcapture flag" ]
    pub fn set_cc4of(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 11 - Capture/Compare 3 overcapture flag" ]
    pub fn set_cc3of(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
    pub fn set_cc2of(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
    pub fn set_cc1of(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 6 - Trigger interrupt flag" ]
    pub fn set_tif(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 4 - Capture/Compare 4 interrupt flag" ]
    pub fn set_cc4if(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - Capture/Compare 3 interrupt flag" ]
    pub fn set_cc3if(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
    pub fn set_cc2if(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
    pub fn set_cc1if(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Update interrupt flag" ]
    pub fn set_uif(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Egr {
    bits: u32,
}

impl Egr {
    # [ doc = "Bit 6 - Trigger generation" ]
    pub fn tg(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 4 - Capture/compare 4 generation" ]
    pub fn cc4g(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Capture/compare 3 generation" ]
    pub fn cc3g(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Capture/compare 2 generation" ]
    pub fn cc2g(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Capture/compare 1 generation" ]
    pub fn cc1g(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Update generation" ]
    pub fn ug(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Egr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Egr { bits: 0u32 }
    }
}

impl Egr {
    # [ doc = "Bit 6 - Trigger generation" ]
    pub fn set_tg(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 4 - Capture/compare 4 generation" ]
    pub fn set_cc4g(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - Capture/compare 3 generation" ]
    pub fn set_cc3g(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Capture/compare 2 generation" ]
    pub fn set_cc2g(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Capture/compare 1 generation" ]
    pub fn set_cc1g(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Update generation" ]
    pub fn set_ug(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Ccmr1Output {
    bits: u32,
}

impl Ccmr1Output {
    # [ doc = "Bit 15 - OC2CE" ]
    pub fn oc2ce(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bits 12:14 - OC2M" ]
    pub fn oc2m(&self) -> u8 {
        self.bits.get_range(12u8..15u8) as u8
    }
    # [ doc = "Bit 11 - OC2PE" ]
    pub fn oc2pe(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - OC2FE" ]
    pub fn oc2fe(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bits 8:9 - CC2S" ]
    pub fn cc2s(&self) -> u8 {
        self.bits.get_range(8u8..10u8) as u8
    }
    # [ doc = "Bit 7 - OC1CE" ]
    pub fn oc1ce(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 4:6 - OC1M" ]
    pub fn oc1m(&self) -> u8 {
        self.bits.get_range(4u8..7u8) as u8
    }
    # [ doc = "Bit 3 - OC1PE" ]
    pub fn oc1pe(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - OC1FE" ]
    pub fn oc1fe(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bits 0:1 - CC1S" ]
    pub fn cc1s(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
}

impl Default for Ccmr1Output {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccmr1Output { bits: 0u32 }
    }
}

impl Ccmr1Output {
    # [ doc = "Bit 15 - OC2CE" ]
    pub fn set_oc2ce(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bits 12:14 - OC2M" ]
    pub fn set_oc2m(&mut self, value: u8) {
        self.bits.set_range(12u8..15u8, value as u32);
    }
    # [ doc = "Bit 11 - OC2PE" ]
    pub fn set_oc2pe(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - OC2FE" ]
    pub fn set_oc2fe(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bits 8:9 - CC2S" ]
    pub fn set_cc2s(&mut self, value: u8) {
        self.bits.set_range(8u8..10u8, value as u32);
    }
    # [ doc = "Bit 7 - OC1CE" ]
    pub fn set_oc1ce(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 4:6 - OC1M" ]
    pub fn set_oc1m(&mut self, value: u8) {
        self.bits.set_range(4u8..7u8, value as u32);
    }
    # [ doc = "Bit 3 - OC1PE" ]
    pub fn set_oc1pe(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - OC1FE" ]
    pub fn set_oc1fe(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bits 0:1 - CC1S" ]
    pub fn set_cc1s(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Ccmr1Input {
    bits: u32,
}

impl Ccmr1Input {
    # [ doc = "Bits 12:15 - Input capture 2 filter" ]
    pub fn ic2f(&self) -> u8 {
        self.bits.get_range(12u8..16u8) as u8
    }
    # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
    pub fn ic2pcs(&self) -> u8 {
        self.bits.get_range(10u8..12u8) as u8
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn cc2s(&self) -> u8 {
        self.bits.get_range(8u8..10u8) as u8
    }
    # [ doc = "Bits 4:7 - Input capture 1 filter" ]
    pub fn ic1f(&self) -> u8 {
        self.bits.get_range(4u8..8u8) as u8
    }
    # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
    pub fn icpcs(&self) -> u8 {
        self.bits.get_range(2u8..4u8) as u8
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn cc1s(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
}

impl Default for Ccmr1Input {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccmr1Input { bits: 0u32 }
    }
}

impl Ccmr1Input {
    # [ doc = "Bits 12:15 - Input capture 2 filter" ]
    pub fn set_ic2f(&mut self, value: u8) {
        self.bits.set_range(12u8..16u8, value as u32);
    }
    # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
    pub fn set_ic2pcs(&mut self, value: u8) {
        self.bits.set_range(10u8..12u8, value as u32);
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn set_cc2s(&mut self, value: u8) {
        self.bits.set_range(8u8..10u8, value as u32);
    }
    # [ doc = "Bits 4:7 - Input capture 1 filter" ]
    pub fn set_ic1f(&mut self, value: u8) {
        self.bits.set_range(4u8..8u8, value as u32);
    }
    # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
    pub fn set_icpcs(&mut self, value: u8) {
        self.bits.set_range(2u8..4u8, value as u32);
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn set_cc1s(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Ccmr2Output {
    bits: u32,
}

impl Ccmr2Output {
    # [ doc = "Bit 15 - O24CE" ]
    pub fn o24ce(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bits 12:14 - OC4M" ]
    pub fn oc4m(&self) -> u8 {
        self.bits.get_range(12u8..15u8) as u8
    }
    # [ doc = "Bit 11 - OC4PE" ]
    pub fn oc4pe(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - OC4FE" ]
    pub fn oc4fe(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bits 8:9 - CC4S" ]
    pub fn cc4s(&self) -> u8 {
        self.bits.get_range(8u8..10u8) as u8
    }
    # [ doc = "Bit 7 - OC3CE" ]
    pub fn oc3ce(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 4:6 - OC3M" ]
    pub fn oc3m(&self) -> u8 {
        self.bits.get_range(4u8..7u8) as u8
    }
    # [ doc = "Bit 3 - OC3PE" ]
    pub fn oc3pe(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - OC3FE" ]
    pub fn oc3fe(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bits 0:1 - CC3S" ]
    pub fn cc3s(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
}

impl Default for Ccmr2Output {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccmr2Output { bits: 0u32 }
    }
}

impl Ccmr2Output {
    # [ doc = "Bit 15 - O24CE" ]
    pub fn set_o24ce(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bits 12:14 - OC4M" ]
    pub fn set_oc4m(&mut self, value: u8) {
        self.bits.set_range(12u8..15u8, value as u32);
    }
    # [ doc = "Bit 11 - OC4PE" ]
    pub fn set_oc4pe(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - OC4FE" ]
    pub fn set_oc4fe(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bits 8:9 - CC4S" ]
    pub fn set_cc4s(&mut self, value: u8) {
        self.bits.set_range(8u8..10u8, value as u32);
    }
    # [ doc = "Bit 7 - OC3CE" ]
    pub fn set_oc3ce(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 4:6 - OC3M" ]
    pub fn set_oc3m(&mut self, value: u8) {
        self.bits.set_range(4u8..7u8, value as u32);
    }
    # [ doc = "Bit 3 - OC3PE" ]
    pub fn set_oc3pe(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - OC3FE" ]
    pub fn set_oc3fe(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bits 0:1 - CC3S" ]
    pub fn set_cc3s(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Ccmr2Input {
    bits: u32,
}

impl Ccmr2Input {
    # [ doc = "Bits 12:15 - Input capture 4 filter" ]
    pub fn ic4f(&self) -> u8 {
        self.bits.get_range(12u8..16u8) as u8
    }
    # [ doc = "Bits 10:11 - Input capture 4 prescaler" ]
    pub fn ic4psc(&self) -> u8 {
        self.bits.get_range(10u8..12u8) as u8
    }
    # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
    pub fn cc4s(&self) -> u8 {
        self.bits.get_range(8u8..10u8) as u8
    }
    # [ doc = "Bits 4:7 - Input capture 3 filter" ]
    pub fn ic3f(&self) -> u8 {
        self.bits.get_range(4u8..8u8) as u8
    }
    # [ doc = "Bits 2:3 - Input capture 3 prescaler" ]
    pub fn ic3psc(&self) -> u8 {
        self.bits.get_range(2u8..4u8) as u8
    }
    # [ doc = "Bits 0:1 - Capture/compare 3 selection" ]
    pub fn cc3s(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
}

impl Default for Ccmr2Input {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccmr2Input { bits: 0u32 }
    }
}

impl Ccmr2Input {
    # [ doc = "Bits 12:15 - Input capture 4 filter" ]
    pub fn set_ic4f(&mut self, value: u8) {
        self.bits.set_range(12u8..16u8, value as u32);
    }
    # [ doc = "Bits 10:11 - Input capture 4 prescaler" ]
    pub fn set_ic4psc(&mut self, value: u8) {
        self.bits.set_range(10u8..12u8, value as u32);
    }
    # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
    pub fn set_cc4s(&mut self, value: u8) {
        self.bits.set_range(8u8..10u8, value as u32);
    }
    # [ doc = "Bits 4:7 - Input capture 3 filter" ]
    pub fn set_ic3f(&mut self, value: u8) {
        self.bits.set_range(4u8..8u8, value as u32);
    }
    # [ doc = "Bits 2:3 - Input capture 3 prescaler" ]
    pub fn set_ic3psc(&mut self, value: u8) {
        self.bits.set_range(2u8..4u8, value as u32);
    }
    # [ doc = "Bits 0:1 - Capture/compare 3 selection" ]
    pub fn set_cc3s(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Ccer {
    bits: u32,
}

impl Ccer {
    # [ doc = "Bit 15 - Capture/Compare 4 output Polarity" ]
    pub fn cc4np(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bit 13 - Capture/Compare 3 output Polarity" ]
    pub fn cc4p(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bit 12 - Capture/Compare 4 output enable" ]
    pub fn cc4e(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 11 - Capture/Compare 3 output Polarity" ]
    pub fn cc3np(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 9 - Capture/Compare 3 output Polarity" ]
    pub fn cc3p(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Capture/Compare 3 output enable" ]
    pub fn cc3e(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
    pub fn cc2np(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
    pub fn cc2p(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
    pub fn cc2e(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
    pub fn cc1np(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
    pub fn cc1p(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
    pub fn cc1e(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Ccer {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccer { bits: 0u32 }
    }
}

impl Ccer {
    # [ doc = "Bit 15 - Capture/Compare 4 output Polarity" ]
    pub fn set_cc4np(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bit 13 - Capture/Compare 3 output Polarity" ]
    pub fn set_cc4p(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bit 12 - Capture/Compare 4 output enable" ]
    pub fn set_cc4e(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 11 - Capture/Compare 3 output Polarity" ]
    pub fn set_cc3np(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 9 - Capture/Compare 3 output Polarity" ]
    pub fn set_cc3p(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Capture/Compare 3 output enable" ]
    pub fn set_cc3e(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
    pub fn set_cc2np(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
    pub fn set_cc2p(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
    pub fn set_cc2e(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
    pub fn set_cc1np(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
    pub fn set_cc1p(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
    pub fn set_cc1e(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Cnt {
    bits: u32,
}

impl Cnt {
    # [ doc = "Bits 16:31 - High counter value" ]
    pub fn cnt_h(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 0:15 - Low counter value" ]
    pub fn cnt_l(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Cnt {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cnt { bits: 0u32 }
    }
}

impl Cnt {
    # [ doc = "Bits 16:31 - High counter value" ]
    pub fn set_cnt_h(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 0:15 - Low counter value" ]
    pub fn set_cnt_l(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Psc {
    bits: u32,
}

impl Psc {
    # [ doc = "Bits 0:15 - Prescaler value" ]
    pub fn psc(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Psc {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Psc { bits: 0u32 }
    }
}

impl Psc {
    # [ doc = "Bits 0:15 - Prescaler value" ]
    pub fn set_psc(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Arr {
    bits: u32,
}

impl Arr {
    # [ doc = "Bits 16:31 - High Auto-reload value" ]
    pub fn arr_h(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 0:15 - Low Auto-reload value" ]
    pub fn arr_l(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Arr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Arr { bits: 0u32 }
    }
}

impl Arr {
    # [ doc = "Bits 16:31 - High Auto-reload value" ]
    pub fn set_arr_h(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 0:15 - Low Auto-reload value" ]
    pub fn set_arr_l(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Ccr1 {
    bits: u32,
}

impl Ccr1 {
    # [ doc = "Bits 16:31 - High Capture/Compare 1 value" ]
    pub fn ccr1_h(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 0:15 - Low Capture/Compare 1 value" ]
    pub fn ccr1_l(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Ccr1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccr1 { bits: 0u32 }
    }
}

impl Ccr1 {
    # [ doc = "Bits 16:31 - High Capture/Compare 1 value" ]
    pub fn set_ccr1_h(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 0:15 - Low Capture/Compare 1 value" ]
    pub fn set_ccr1_l(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Ccr2 {
    bits: u32,
}

impl Ccr2 {
    # [ doc = "Bits 16:31 - High Capture/Compare 2 value" ]
    pub fn ccr2_h(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 0:15 - Low Capture/Compare 2 value" ]
    pub fn ccr2_l(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Ccr2 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccr2 { bits: 0u32 }
    }
}

impl Ccr2 {
    # [ doc = "Bits 16:31 - High Capture/Compare 2 value" ]
    pub fn set_ccr2_h(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 0:15 - Low Capture/Compare 2 value" ]
    pub fn set_ccr2_l(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Ccr3 {
    bits: u32,
}

impl Ccr3 {
    # [ doc = "Bits 16:31 - High Capture/Compare value" ]
    pub fn ccr3_h(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 0:15 - Low Capture/Compare value" ]
    pub fn ccr3_l(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Ccr3 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccr3 { bits: 0u32 }
    }
}

impl Ccr3 {
    # [ doc = "Bits 16:31 - High Capture/Compare value" ]
    pub fn set_ccr3_h(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 0:15 - Low Capture/Compare value" ]
    pub fn set_ccr3_l(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Ccr4 {
    bits: u32,
}

impl Ccr4 {
    # [ doc = "Bits 16:31 - High Capture/Compare value" ]
    pub fn ccr4_h(&self) -> u16 {
        self.bits.get_range(16u8..32u8) as u16
    }
    # [ doc = "Bits 0:15 - Low Capture/Compare value" ]
    pub fn ccr4_l(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Ccr4 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Ccr4 { bits: 0u32 }
    }
}

impl Ccr4 {
    # [ doc = "Bits 16:31 - High Capture/Compare value" ]
    pub fn set_ccr4_h(&mut self, value: u16) {
        self.bits.set_range(16u8..32u8, value as u32);
    }
    # [ doc = "Bits 0:15 - Low Capture/Compare value" ]
    pub fn set_ccr4_l(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Dcr {
    bits: u32,
}

impl Dcr {
    # [ doc = "Bits 8:12 - DMA burst length" ]
    pub fn dbl(&self) -> u8 {
        self.bits.get_range(8u8..13u8) as u8
    }
    # [ doc = "Bits 0:4 - DMA base address" ]
    pub fn dba(&self) -> u8 {
        self.bits.get_range(0u8..5u8) as u8
    }
}

impl Default for Dcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Dcr { bits: 0u32 }
    }
}

impl Dcr {
    # [ doc = "Bits 8:12 - DMA burst length" ]
    pub fn set_dbl(&mut self, value: u8) {
        self.bits.set_range(8u8..13u8, value as u32);
    }
    # [ doc = "Bits 0:4 - DMA base address" ]
    pub fn set_dba(&mut self, value: u8) {
        self.bits.set_range(0u8..5u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Dmar {
    bits: u32,
}

impl Dmar {
    # [ doc = "Bits 0:15 - DMA register for burst accesses" ]
    pub fn dmab(&self) -> u16 {
        self.bits.get_range(0u8..16u8) as u16
    }
}

impl Default for Dmar {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Dmar { bits: 0u32 }
    }
}

impl Dmar {
    # [ doc = "Bits 0:15 - DMA register for burst accesses" ]
    pub fn set_dmab(&mut self, value: u16) {
        self.bits.set_range(0u8..16u8, value as u32);
    }
}
