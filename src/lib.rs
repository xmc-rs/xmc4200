#![doc = "Peripheral access API for XMC4200 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 6;
#[cfg(feature = "rt")]
extern "C" {
    fn SCU_0();
    fn ERU0_0();
    fn ERU0_1();
    fn ERU0_2();
    fn ERU0_3();
    fn ERU1_0();
    fn ERU1_1();
    fn ERU1_2();
    fn ERU1_3();
    fn PMU0_0();
    fn VADC0_C0_0();
    fn VADC0_C0_1();
    fn VADC0_C0_2();
    fn VADC0_C0_3();
    fn VADC0_G0_0();
    fn VADC0_G0_1();
    fn VADC0_G0_2();
    fn VADC0_G0_3();
    fn VADC0_G1_0();
    fn VADC0_G1_1();
    fn VADC0_G1_2();
    fn VADC0_G1_3();
    fn DAC0_0();
    fn DAC0_1();
    fn CCU40_0();
    fn CCU40_1();
    fn CCU40_2();
    fn CCU40_3();
    fn CCU41_0();
    fn CCU41_1();
    fn CCU41_2();
    fn CCU41_3();
    fn CCU80_0();
    fn CCU80_1();
    fn CCU80_2();
    fn CCU80_3();
    fn POSIF0_0();
    fn POSIF0_1();
    fn HRPWM_0();
    fn HRPWM_1();
    fn HRPWM_2();
    fn HRPWM_3();
    fn CAN0_0();
    fn CAN0_1();
    fn CAN0_2();
    fn CAN0_3();
    fn CAN0_4();
    fn CAN0_5();
    fn CAN0_6();
    fn CAN0_7();
    fn USIC0_0();
    fn USIC0_1();
    fn USIC0_2();
    fn USIC0_3();
    fn USIC0_4();
    fn USIC0_5();
    fn USIC1_0();
    fn USIC1_1();
    fn USIC1_2();
    fn USIC1_3();
    fn USIC1_4();
    fn USIC1_5();
    fn LEDTS0_0();
    fn FCE0_0();
    fn GPDMA0_0();
    fn USB0_0();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 108] = [
    Vector { _handler: SCU_0 },
    Vector { _handler: ERU0_0 },
    Vector { _handler: ERU0_1 },
    Vector { _handler: ERU0_2 },
    Vector { _handler: ERU0_3 },
    Vector { _handler: ERU1_0 },
    Vector { _handler: ERU1_1 },
    Vector { _handler: ERU1_2 },
    Vector { _handler: ERU1_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PMU0_0 },
    Vector { _reserved: 0 },
    Vector { _handler: VADC0_C0_0 },
    Vector { _handler: VADC0_C0_1 },
    Vector { _handler: VADC0_C0_2 },
    Vector { _handler: VADC0_C0_3 },
    Vector { _handler: VADC0_G0_0 },
    Vector { _handler: VADC0_G0_1 },
    Vector { _handler: VADC0_G0_2 },
    Vector { _handler: VADC0_G0_3 },
    Vector { _handler: VADC0_G1_0 },
    Vector { _handler: VADC0_G1_1 },
    Vector { _handler: VADC0_G1_2 },
    Vector { _handler: VADC0_G1_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DAC0_0 },
    Vector { _handler: DAC0_1 },
    Vector { _handler: CCU40_0 },
    Vector { _handler: CCU40_1 },
    Vector { _handler: CCU40_2 },
    Vector { _handler: CCU40_3 },
    Vector { _handler: CCU41_0 },
    Vector { _handler: CCU41_1 },
    Vector { _handler: CCU41_2 },
    Vector { _handler: CCU41_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CCU80_0 },
    Vector { _handler: CCU80_1 },
    Vector { _handler: CCU80_2 },
    Vector { _handler: CCU80_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: POSIF0_0 },
    Vector { _handler: POSIF0_1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: HRPWM_0 },
    Vector { _handler: HRPWM_1 },
    Vector { _handler: HRPWM_2 },
    Vector { _handler: HRPWM_3 },
    Vector { _handler: CAN0_0 },
    Vector { _handler: CAN0_1 },
    Vector { _handler: CAN0_2 },
    Vector { _handler: CAN0_3 },
    Vector { _handler: CAN0_4 },
    Vector { _handler: CAN0_5 },
    Vector { _handler: CAN0_6 },
    Vector { _handler: CAN0_7 },
    Vector { _handler: USIC0_0 },
    Vector { _handler: USIC0_1 },
    Vector { _handler: USIC0_2 },
    Vector { _handler: USIC0_3 },
    Vector { _handler: USIC0_4 },
    Vector { _handler: USIC0_5 },
    Vector { _handler: USIC1_0 },
    Vector { _handler: USIC1_1 },
    Vector { _handler: USIC1_2 },
    Vector { _handler: USIC1_3 },
    Vector { _handler: USIC1_4 },
    Vector { _handler: USIC1_5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LEDTS0_0 },
    Vector { _reserved: 0 },
    Vector { _handler: FCE0_0 },
    Vector { _handler: GPDMA0_0 },
    Vector { _reserved: 0 },
    Vector { _handler: USB0_0 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - System Control"]
    SCU_0 = 0,
    #[doc = "1 - External Request Unit 0"]
    ERU0_0 = 1,
    #[doc = "2 - External Request Unit 0"]
    ERU0_1 = 2,
    #[doc = "3 - External Request Unit 0"]
    ERU0_2 = 3,
    #[doc = "4 - External Request Unit 0"]
    ERU0_3 = 4,
    #[doc = "5 - External Request Unit 1"]
    ERU1_0 = 5,
    #[doc = "6 - External Request Unit 1"]
    ERU1_1 = 6,
    #[doc = "7 - External Request Unit 1"]
    ERU1_2 = 7,
    #[doc = "8 - External Request Unit 1"]
    ERU1_3 = 8,
    #[doc = "12 - Program Management Unit"]
    PMU0_0 = 12,
    #[doc = "14 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_0 = 14,
    #[doc = "15 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_1 = 15,
    #[doc = "16 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_2 = 16,
    #[doc = "17 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_3 = 17,
    #[doc = "18 - Analog to Digital Converter Group 0"]
    VADC0_G0_0 = 18,
    #[doc = "19 - Analog to Digital Converter Group 0"]
    VADC0_G0_1 = 19,
    #[doc = "20 - Analog to Digital Converter Group 0"]
    VADC0_G0_2 = 20,
    #[doc = "21 - Analog to Digital Converter Group 0"]
    VADC0_G0_3 = 21,
    #[doc = "22 - Analog to Digital Converter Group 1"]
    VADC0_G1_0 = 22,
    #[doc = "23 - Analog to Digital Converter Group 1"]
    VADC0_G1_1 = 23,
    #[doc = "24 - Analog to Digital Converter Group 1"]
    VADC0_G1_2 = 24,
    #[doc = "25 - Analog to Digital Converter Group 1"]
    VADC0_G1_3 = 25,
    #[doc = "42 - Digital to Analog Converter"]
    DAC0_0 = 42,
    #[doc = "43 - Digital to Analog Converter"]
    DAC0_1 = 43,
    #[doc = "44 - Capture Compare Unit 4 (Module 0)"]
    CCU40_0 = 44,
    #[doc = "45 - Capture Compare Unit 4 (Module 0)"]
    CCU40_1 = 45,
    #[doc = "46 - Capture Compare Unit 4 (Module 0)"]
    CCU40_2 = 46,
    #[doc = "47 - Capture Compare Unit 4 (Module 0)"]
    CCU40_3 = 47,
    #[doc = "48 - Capture Compare Unit 4 (Module 1)"]
    CCU41_0 = 48,
    #[doc = "49 - Capture Compare Unit 4 (Module 1)"]
    CCU41_1 = 49,
    #[doc = "50 - Capture Compare Unit 4 (Module 1)"]
    CCU41_2 = 50,
    #[doc = "51 - Capture Compare Unit 4 (Module 1)"]
    CCU41_3 = 51,
    #[doc = "60 - Capture Compare Unit 8 (Module 0)"]
    CCU80_0 = 60,
    #[doc = "61 - Capture Compare Unit 8 (Module 0)"]
    CCU80_1 = 61,
    #[doc = "62 - Capture Compare Unit 8 (Module 0)"]
    CCU80_2 = 62,
    #[doc = "63 - Capture Compare Unit 8 (Module 0)"]
    CCU80_3 = 63,
    #[doc = "68 - Position Interface (Module 0)"]
    POSIF0_0 = 68,
    #[doc = "69 - Position Interface (Module 0)"]
    POSIF0_1 = 69,
    #[doc = "72 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_0 = 72,
    #[doc = "73 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_1 = 73,
    #[doc = "74 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_2 = 74,
    #[doc = "75 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_3 = 75,
    #[doc = "76 - MultiCAN"]
    CAN0_0 = 76,
    #[doc = "77 - MultiCAN"]
    CAN0_1 = 77,
    #[doc = "78 - MultiCAN"]
    CAN0_2 = 78,
    #[doc = "79 - MultiCAN"]
    CAN0_3 = 79,
    #[doc = "80 - MultiCAN"]
    CAN0_4 = 80,
    #[doc = "81 - MultiCAN"]
    CAN0_5 = 81,
    #[doc = "82 - MultiCAN"]
    CAN0_6 = 82,
    #[doc = "83 - MultiCAN"]
    CAN0_7 = 83,
    #[doc = "84 - Universal Serial Interface Channel (Module 0)"]
    USIC0_0 = 84,
    #[doc = "85 - Universal Serial Interface Channel (Module 0)"]
    USIC0_1 = 85,
    #[doc = "86 - Universal Serial Interface Channel (Module 0)"]
    USIC0_2 = 86,
    #[doc = "87 - Universal Serial Interface Channel (Module 0)"]
    USIC0_3 = 87,
    #[doc = "88 - Universal Serial Interface Channel (Module 0)"]
    USIC0_4 = 88,
    #[doc = "89 - Universal Serial Interface Channel (Module 0)"]
    USIC0_5 = 89,
    #[doc = "90 - Universal Serial Interface Channel (Module 1)"]
    USIC1_0 = 90,
    #[doc = "91 - Universal Serial Interface Channel (Module 1)"]
    USIC1_1 = 91,
    #[doc = "92 - Universal Serial Interface Channel (Module 1)"]
    USIC1_2 = 92,
    #[doc = "93 - Universal Serial Interface Channel (Module 1)"]
    USIC1_3 = 93,
    #[doc = "94 - Universal Serial Interface Channel (Module 1)"]
    USIC1_4 = 94,
    #[doc = "95 - Universal Serial Interface Channel (Module 1)"]
    USIC1_5 = 95,
    #[doc = "102 - LED and Touch Sense Control Unit (Module 0)"]
    LEDTS0_0 = 102,
    #[doc = "104 - Flexible CRC Engine"]
    FCE0_0 = 104,
    #[doc = "105 - General Purpose DMA Unit 0"]
    GPDMA0_0 = 105,
    #[doc = "107 - Universal Serial Bus (Module 0)"]
    USB0_0 = 107,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Cortex-M4 Private Peripheral Block"]
pub struct PPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPB {}
impl PPB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ppb::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for PPB {
    type Target = ppb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PPB::ptr() }
    }
}
#[doc = "Cortex-M4 Private Peripheral Block"]
pub mod ppb;
#[doc = "DMA Line Router"]
pub struct DLR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLR {}
impl DLR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlr::RegisterBlock {
        0x5000_4900 as *const _
    }
}
impl Deref for DLR {
    type Target = dlr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLR::ptr() }
    }
}
#[doc = "DMA Line Router"]
pub mod dlr;
#[doc = "Event Request Unit 0"]
pub struct ERU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ERU0 {}
impl ERU0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eru0::RegisterBlock {
        0x5000_4800 as *const _
    }
}
impl Deref for ERU0 {
    type Target = eru0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ERU0::ptr() }
    }
}
#[doc = "Event Request Unit 0"]
pub mod eru0;
#[doc = "Event Request Unit 1"]
pub struct ERU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ERU1 {}
impl ERU1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eru0::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for ERU1 {
    type Target = eru0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ERU1::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0 {}
impl GPDMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0::RegisterBlock {
        0x5001_42c0 as *const _
    }
}
impl Deref for GPDMA0 {
    type Target = gpdma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH0 {}
impl GPDMA0_CH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch0::RegisterBlock {
        0x5001_4000 as *const _
    }
}
impl Deref for GPDMA0_CH0 {
    type Target = gpdma0_ch0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH0::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0_ch0;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH1 {}
impl GPDMA0_CH1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch0::RegisterBlock {
        0x5001_4058 as *const _
    }
}
impl Deref for GPDMA0_CH1 {
    type Target = gpdma0_ch0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH1::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH2 {}
impl GPDMA0_CH2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_40b0 as *const _
    }
}
impl Deref for GPDMA0_CH2 {
    type Target = gpdma0_ch2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH2::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0_ch2;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH3 {}
impl GPDMA0_CH3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_4108 as *const _
    }
}
impl Deref for GPDMA0_CH3 {
    type Target = gpdma0_ch2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH3::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH4 {}
impl GPDMA0_CH4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_4160 as *const _
    }
}
impl Deref for GPDMA0_CH4 {
    type Target = gpdma0_ch2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH4::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH5 {}
impl GPDMA0_CH5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_41b8 as *const _
    }
}
impl Deref for GPDMA0_CH5 {
    type Target = gpdma0_ch2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH5::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH6 {}
impl GPDMA0_CH6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_4210 as *const _
    }
}
impl Deref for GPDMA0_CH6 {
    type Target = gpdma0_ch2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH6::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH7 {}
impl GPDMA0_CH7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        0x5001_4268 as *const _
    }
}
impl Deref for GPDMA0_CH7 {
    type Target = gpdma0_ch2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA0_CH7::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub struct FCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE {}
impl FCE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce::RegisterBlock {
        0x5002_0000 as *const _
    }
}
impl Deref for FCE {
    type Target = fce::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub mod fce;
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE0 {}
impl FCE_KE0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce_ke0::RegisterBlock {
        0x5002_0020 as *const _
    }
}
impl Deref for FCE_KE0 {
    type Target = fce_ke0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE_KE0::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub mod fce_ke0;
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE1 {}
impl FCE_KE1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce_ke0::RegisterBlock {
        0x5002_0040 as *const _
    }
}
impl Deref for FCE_KE1 {
    type Target = fce_ke0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE_KE1::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE2 {}
impl FCE_KE2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce_ke0::RegisterBlock {
        0x5002_0060 as *const _
    }
}
impl Deref for FCE_KE2 {
    type Target = fce_ke0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE_KE2::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE3 {}
impl FCE_KE3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fce_ke0::RegisterBlock {
        0x5002_0080 as *const _
    }
}
impl Deref for FCE_KE3 {
    type Target = fce_ke0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCE_KE3::ptr() }
    }
}
#[doc = "Peripheral Bridge AHB 0"]
pub struct PBA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PBA0 {}
impl PBA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pba0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for PBA0 {
    type Target = pba0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PBA0::ptr() }
    }
}
#[doc = "Peripheral Bridge AHB 0"]
pub mod pba0;
#[doc = "Peripheral Bridge AHB 1"]
pub struct PBA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PBA1 {}
impl PBA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pba0::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for PBA1 {
    type Target = pba0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PBA1::ptr() }
    }
}
#[doc = "Flash Memory Controller"]
pub struct FLASH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH0 {}
impl FLASH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash0::RegisterBlock {
        0x5800_1000 as *const _
    }
}
impl Deref for FLASH0 {
    type Target = flash0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH0::ptr() }
    }
}
#[doc = "Flash Memory Controller"]
pub mod flash0;
#[doc = "Prefetch Unit"]
pub struct PREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PREF {}
impl PREF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pref::RegisterBlock {
        0x5800_4000 as *const _
    }
}
impl Deref for PREF {
    type Target = pref::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PREF::ptr() }
    }
}
#[doc = "Prefetch Unit"]
pub mod pref;
#[doc = "Program Management Unit"]
pub struct PMU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU0 {}
impl PMU0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu0::RegisterBlock {
        0x5800_0508 as *const _
    }
}
impl Deref for PMU0 {
    type Target = pmu0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMU0::ptr() }
    }
}
#[doc = "Program Management Unit"]
pub mod pmu0;
#[doc = "Watch Dog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watch Dog Timer"]
pub mod wdt;
#[doc = "Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x5000_4a00 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock"]
pub mod rtc;
#[doc = "System Control Unit"]
pub struct SCU_CLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_CLK {}
impl SCU_CLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_clk::RegisterBlock {
        0x5000_4600 as *const _
    }
}
impl Deref for SCU_CLK {
    type Target = scu_clk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_CLK::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_clk;
#[doc = "System Control Unit"]
pub struct SCU_OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_OSC {}
impl SCU_OSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_osc::RegisterBlock {
        0x5000_4700 as *const _
    }
}
impl Deref for SCU_OSC {
    type Target = scu_osc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_OSC::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_osc;
#[doc = "System Control Unit"]
pub struct SCU_PLL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_PLL {}
impl SCU_PLL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_pll::RegisterBlock {
        0x5000_4710 as *const _
    }
}
impl Deref for SCU_PLL {
    type Target = scu_pll::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_PLL::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_pll;
#[doc = "System Control Unit"]
pub struct SCU_GENERAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_GENERAL {}
impl SCU_GENERAL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_general::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for SCU_GENERAL {
    type Target = scu_general::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_GENERAL::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_general;
#[doc = "System Control Unit"]
pub struct SCU_INTERRUPT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_INTERRUPT {}
impl SCU_INTERRUPT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_interrupt::RegisterBlock {
        0x5000_4074 as *const _
    }
}
impl Deref for SCU_INTERRUPT {
    type Target = scu_interrupt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_INTERRUPT::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_interrupt;
#[doc = "System Control Unit"]
pub struct SCU_PARITY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_PARITY {}
impl SCU_PARITY {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_parity::RegisterBlock {
        0x5000_413c as *const _
    }
}
impl Deref for SCU_PARITY {
    type Target = scu_parity::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_PARITY::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_parity;
#[doc = "System Control Unit"]
pub struct SCU_TRAP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_TRAP {}
impl SCU_TRAP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_trap::RegisterBlock {
        0x5000_4160 as *const _
    }
}
impl Deref for SCU_TRAP {
    type Target = scu_trap::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_TRAP::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_trap;
#[doc = "System Control Unit"]
pub struct SCU_HIBERNATE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_HIBERNATE {}
impl SCU_HIBERNATE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_hibernate::RegisterBlock {
        0x5000_4300 as *const _
    }
}
impl Deref for SCU_HIBERNATE {
    type Target = scu_hibernate::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_HIBERNATE::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_hibernate;
#[doc = "System Control Unit"]
pub struct SCU_POWER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_POWER {}
impl SCU_POWER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_power::RegisterBlock {
        0x5000_4200 as *const _
    }
}
impl Deref for SCU_POWER {
    type Target = scu_power::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_POWER::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_power;
#[doc = "System Control Unit"]
pub struct SCU_RESET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_RESET {}
impl SCU_RESET {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scu_reset::RegisterBlock {
        0x5000_4400 as *const _
    }
}
impl Deref for SCU_RESET {
    type Target = scu_reset::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCU_RESET::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_reset;
#[doc = "LED and Touch Sense Unit 0"]
pub struct LEDTS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEDTS0 {}
impl LEDTS0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ledts0::RegisterBlock {
        0x4801_0000 as *const _
    }
}
impl Deref for LEDTS0 {
    type Target = ledts0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LEDTS0::ptr() }
    }
}
#[doc = "LED and Touch Sense Unit 0"]
pub mod ledts0;
#[doc = "Universal Serial Bus"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0::RegisterBlock {
        0x5004_0000 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP0 {}
impl USB0_EP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep0::RegisterBlock {
        0x5004_0900 as *const _
    }
}
impl Deref for USB0_EP0 {
    type Target = usb0_ep0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP0::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0_ep0;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP1 {}
impl USB0_EP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_0920 as *const _
    }
}
impl Deref for USB0_EP1 {
    type Target = usb0_ep1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP1::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0_ep1;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP2 {}
impl USB0_EP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_0940 as *const _
    }
}
impl Deref for USB0_EP2 {
    type Target = usb0_ep1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP2::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP3 {}
impl USB0_EP3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_0960 as *const _
    }
}
impl Deref for USB0_EP3 {
    type Target = usb0_ep1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP3::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP4 {}
impl USB0_EP4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_0980 as *const _
    }
}
impl Deref for USB0_EP4 {
    type Target = usb0_ep1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP4::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP5 {}
impl USB0_EP5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_09a0 as *const _
    }
}
impl Deref for USB0_EP5 {
    type Target = usb0_ep1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP5::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP6 {}
impl USB0_EP6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0_ep1::RegisterBlock {
        0x5004_09c0 as *const _
    }
}
impl Deref for USB0_EP6 {
    type Target = usb0_ep1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0_EP6::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0 {}
impl USIC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0::RegisterBlock {
        0x4003_0008 as *const _
    }
}
impl Deref for USIC0 {
    type Target = usic0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0;
#[doc = "Universal Serial Interface Controller 1"]
pub struct USIC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC1 {}
impl USIC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0::RegisterBlock {
        0x4802_0008 as *const _
    }
}
impl Deref for USIC1 {
    type Target = usic0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC1::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0_CH0 {}
impl USIC0_CH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0_ch0::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for USIC0_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC0_CH0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0_ch0;
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0_CH1 {}
impl USIC0_CH1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0_ch0::RegisterBlock {
        0x4003_0200 as *const _
    }
}
impl Deref for USIC0_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC0_CH1::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 1"]
pub struct USIC1_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC1_CH0 {}
impl USIC1_CH0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0_ch0::RegisterBlock {
        0x4802_0000 as *const _
    }
}
impl Deref for USIC1_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC1_CH0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 1"]
pub struct USIC1_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC1_CH1 {}
impl USIC1_CH1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usic0_ch0::RegisterBlock {
        0x4802_0200 as *const _
    }
}
impl Deref for USIC1_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USIC1_CH1::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN {}
impl CAN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can::RegisterBlock {
        0x4801_4000 as *const _
    }
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub mod can;
#[doc = "Controller Area Networks"]
pub struct CAN_NODE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_NODE0 {}
impl CAN_NODE0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_node0::RegisterBlock {
        0x4801_4200 as *const _
    }
}
impl Deref for CAN_NODE0 {
    type Target = can_node0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_NODE0::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub mod can_node0;
#[doc = "Controller Area Networks"]
pub struct CAN_NODE1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_NODE1 {}
impl CAN_NODE1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_node0::RegisterBlock {
        0x4801_4300 as *const _
    }
}
impl Deref for CAN_NODE1 {
    type Target = can_node0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_NODE1::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO0 {}
impl CAN_MO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5000 as *const _
    }
}
impl Deref for CAN_MO0 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO0::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub mod can_mo0;
#[doc = "Controller Area Networks"]
pub struct CAN_MO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO1 {}
impl CAN_MO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5020 as *const _
    }
}
impl Deref for CAN_MO1 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO1::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO2 {}
impl CAN_MO2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5040 as *const _
    }
}
impl Deref for CAN_MO2 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO2::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO3 {}
impl CAN_MO3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5060 as *const _
    }
}
impl Deref for CAN_MO3 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO3::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO4 {}
impl CAN_MO4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5080 as *const _
    }
}
impl Deref for CAN_MO4 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO4::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO5 {}
impl CAN_MO5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_50a0 as *const _
    }
}
impl Deref for CAN_MO5 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO5::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO6 {}
impl CAN_MO6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_50c0 as *const _
    }
}
impl Deref for CAN_MO6 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO6::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO7 {}
impl CAN_MO7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_50e0 as *const _
    }
}
impl Deref for CAN_MO7 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO7::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO8 {}
impl CAN_MO8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5100 as *const _
    }
}
impl Deref for CAN_MO8 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO8::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO9 {}
impl CAN_MO9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5120 as *const _
    }
}
impl Deref for CAN_MO9 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO9::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO10 {}
impl CAN_MO10 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5140 as *const _
    }
}
impl Deref for CAN_MO10 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO10::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO11 {}
impl CAN_MO11 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5160 as *const _
    }
}
impl Deref for CAN_MO11 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO11::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO12 {}
impl CAN_MO12 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5180 as *const _
    }
}
impl Deref for CAN_MO12 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO12::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO13 {}
impl CAN_MO13 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_51a0 as *const _
    }
}
impl Deref for CAN_MO13 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO13::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO14 {}
impl CAN_MO14 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_51c0 as *const _
    }
}
impl Deref for CAN_MO14 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO14::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO15 {}
impl CAN_MO15 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_51e0 as *const _
    }
}
impl Deref for CAN_MO15 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO15::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO16 {}
impl CAN_MO16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5200 as *const _
    }
}
impl Deref for CAN_MO16 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO16::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO17 {}
impl CAN_MO17 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5220 as *const _
    }
}
impl Deref for CAN_MO17 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO17::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO18 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO18 {}
impl CAN_MO18 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5240 as *const _
    }
}
impl Deref for CAN_MO18 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO18::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO19 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO19 {}
impl CAN_MO19 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5260 as *const _
    }
}
impl Deref for CAN_MO19 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO19::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO20 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO20 {}
impl CAN_MO20 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5280 as *const _
    }
}
impl Deref for CAN_MO20 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO20::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO21 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO21 {}
impl CAN_MO21 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_52a0 as *const _
    }
}
impl Deref for CAN_MO21 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO21::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO22 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO22 {}
impl CAN_MO22 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_52c0 as *const _
    }
}
impl Deref for CAN_MO22 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO22::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO23 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO23 {}
impl CAN_MO23 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_52e0 as *const _
    }
}
impl Deref for CAN_MO23 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO23::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO24 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO24 {}
impl CAN_MO24 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5300 as *const _
    }
}
impl Deref for CAN_MO24 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO24::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO25 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO25 {}
impl CAN_MO25 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5320 as *const _
    }
}
impl Deref for CAN_MO25 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO25::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO26 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO26 {}
impl CAN_MO26 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5340 as *const _
    }
}
impl Deref for CAN_MO26 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO26::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO27 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO27 {}
impl CAN_MO27 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5360 as *const _
    }
}
impl Deref for CAN_MO27 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO27::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO28 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO28 {}
impl CAN_MO28 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5380 as *const _
    }
}
impl Deref for CAN_MO28 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO28::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO29 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO29 {}
impl CAN_MO29 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_53a0 as *const _
    }
}
impl Deref for CAN_MO29 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO29::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO30 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO30 {}
impl CAN_MO30 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_53c0 as *const _
    }
}
impl Deref for CAN_MO30 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO30::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO31 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO31 {}
impl CAN_MO31 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_53e0 as *const _
    }
}
impl Deref for CAN_MO31 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO31::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO32 {}
impl CAN_MO32 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5400 as *const _
    }
}
impl Deref for CAN_MO32 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO32::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO33 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO33 {}
impl CAN_MO33 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5420 as *const _
    }
}
impl Deref for CAN_MO33 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO33::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO34 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO34 {}
impl CAN_MO34 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5440 as *const _
    }
}
impl Deref for CAN_MO34 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO34::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO35 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO35 {}
impl CAN_MO35 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5460 as *const _
    }
}
impl Deref for CAN_MO35 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO35::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO36 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO36 {}
impl CAN_MO36 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5480 as *const _
    }
}
impl Deref for CAN_MO36 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO36::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO37 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO37 {}
impl CAN_MO37 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_54a0 as *const _
    }
}
impl Deref for CAN_MO37 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO37::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO38 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO38 {}
impl CAN_MO38 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_54c0 as *const _
    }
}
impl Deref for CAN_MO38 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO38::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO39 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO39 {}
impl CAN_MO39 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_54e0 as *const _
    }
}
impl Deref for CAN_MO39 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO39::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO40 {}
impl CAN_MO40 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5500 as *const _
    }
}
impl Deref for CAN_MO40 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO40::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO41 {}
impl CAN_MO41 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5520 as *const _
    }
}
impl Deref for CAN_MO41 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO41::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO42 {}
impl CAN_MO42 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5540 as *const _
    }
}
impl Deref for CAN_MO42 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO42::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO43 {}
impl CAN_MO43 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5560 as *const _
    }
}
impl Deref for CAN_MO43 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO43::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO44 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO44 {}
impl CAN_MO44 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5580 as *const _
    }
}
impl Deref for CAN_MO44 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO44::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO45 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO45 {}
impl CAN_MO45 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_55a0 as *const _
    }
}
impl Deref for CAN_MO45 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO45::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO46 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO46 {}
impl CAN_MO46 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_55c0 as *const _
    }
}
impl Deref for CAN_MO46 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO46::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO47 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO47 {}
impl CAN_MO47 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_55e0 as *const _
    }
}
impl Deref for CAN_MO47 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO47::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO48 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO48 {}
impl CAN_MO48 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5600 as *const _
    }
}
impl Deref for CAN_MO48 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO48::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO49 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO49 {}
impl CAN_MO49 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5620 as *const _
    }
}
impl Deref for CAN_MO49 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO49::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO50 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO50 {}
impl CAN_MO50 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5640 as *const _
    }
}
impl Deref for CAN_MO50 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO50::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO51 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO51 {}
impl CAN_MO51 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5660 as *const _
    }
}
impl Deref for CAN_MO51 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO51::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO52 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO52 {}
impl CAN_MO52 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5680 as *const _
    }
}
impl Deref for CAN_MO52 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO52::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO53 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO53 {}
impl CAN_MO53 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_56a0 as *const _
    }
}
impl Deref for CAN_MO53 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO53::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO54 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO54 {}
impl CAN_MO54 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_56c0 as *const _
    }
}
impl Deref for CAN_MO54 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO54::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO55 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO55 {}
impl CAN_MO55 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_56e0 as *const _
    }
}
impl Deref for CAN_MO55 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO55::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO56 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO56 {}
impl CAN_MO56 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5700 as *const _
    }
}
impl Deref for CAN_MO56 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO56::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO57 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO57 {}
impl CAN_MO57 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5720 as *const _
    }
}
impl Deref for CAN_MO57 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO57::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO58 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO58 {}
impl CAN_MO58 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5740 as *const _
    }
}
impl Deref for CAN_MO58 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO58::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO59 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO59 {}
impl CAN_MO59 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5760 as *const _
    }
}
impl Deref for CAN_MO59 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO59::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO60 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO60 {}
impl CAN_MO60 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_5780 as *const _
    }
}
impl Deref for CAN_MO60 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO60::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO61 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO61 {}
impl CAN_MO61 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_57a0 as *const _
    }
}
impl Deref for CAN_MO61 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO61::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO62 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO62 {}
impl CAN_MO62 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_57c0 as *const _
    }
}
impl Deref for CAN_MO62 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO62::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO63 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO63 {}
impl CAN_MO63 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_mo0::RegisterBlock {
        0x4801_57e0 as *const _
    }
}
impl Deref for CAN_MO63 {
    type Target = can_mo0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_MO63::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub struct VADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC {}
impl VADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vadc::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for VADC {
    type Target = vadc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VADC::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod vadc;
#[doc = "Analog to Digital Converter"]
pub struct VADC_G0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC_G0 {}
impl VADC_G0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vadc_g0::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for VADC_G0 {
    type Target = vadc_g0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VADC_G0::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod vadc_g0;
#[doc = "Analog to Digital Converter"]
pub struct VADC_G1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC_G1 {}
impl VADC_G1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vadc_g0::RegisterBlock {
        0x4000_4800 as *const _
    }
}
impl Deref for VADC_G1 {
    type Target = vadc_g0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VADC_G1::ptr() }
    }
}
#[doc = "Digital to Analog Converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4801_8000 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital to Analog Converter"]
pub mod dac;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40 {}
impl CCU40 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for CCU40 {
    type Target = ccu40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40;
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41 {}
impl CCU41 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for CCU41 {
    type Target = ccu40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC40 {}
impl CCU40_CC40 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4000_c100 as *const _
    }
}
impl Deref for CCU40_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40_CC40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40_cc40;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC41 {}
impl CCU40_CC41 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4000_c200 as *const _
    }
}
impl Deref for CCU40_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40_CC41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC42 {}
impl CCU40_CC42 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4000_c300 as *const _
    }
}
impl Deref for CCU40_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40_CC42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC43 {}
impl CCU40_CC43 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4000_c400 as *const _
    }
}
impl Deref for CCU40_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU40_CC43::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC40 {}
impl CCU41_CC40 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4001_0100 as *const _
    }
}
impl Deref for CCU41_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41_CC40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC41 {}
impl CCU41_CC41 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4001_0200 as *const _
    }
}
impl Deref for CCU41_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41_CC41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC42 {}
impl CCU41_CC42 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4001_0300 as *const _
    }
}
impl Deref for CCU41_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41_CC42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC43 {}
impl CCU41_CC43 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu40_cc40::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for CCU41_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU41_CC43::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80 {}
impl CCU80 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for CCU80 {
    type Target = ccu80::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub mod ccu80;
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC80 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC80 {}
impl CCU80_CC80 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80_cc80::RegisterBlock {
        0x4002_0100 as *const _
    }
}
impl Deref for CCU80_CC80 {
    type Target = ccu80_cc80::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80_CC80::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub mod ccu80_cc80;
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC81 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC81 {}
impl CCU80_CC81 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80_cc80::RegisterBlock {
        0x4002_0200 as *const _
    }
}
impl Deref for CCU80_CC81 {
    type Target = ccu80_cc80::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80_CC81::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC82 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC82 {}
impl CCU80_CC82 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80_cc80::RegisterBlock {
        0x4002_0300 as *const _
    }
}
impl Deref for CCU80_CC82 {
    type Target = ccu80_cc80::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80_CC82::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC83 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC83 {}
impl CCU80_CC83 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu80_cc80::RegisterBlock {
        0x4002_0400 as *const _
    }
}
impl Deref for CCU80_CC83 {
    type Target = ccu80_cc80::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU80_CC83::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0 {}
impl HRPWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrpwm0::RegisterBlock {
        0x4002_0900 as *const _
    }
}
impl Deref for HRPWM0 {
    type Target = hrpwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRPWM0::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub mod hrpwm0;
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_CSG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_CSG0 {}
impl HRPWM0_CSG0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrpwm0_csg0::RegisterBlock {
        0x4002_0a00 as *const _
    }
}
impl Deref for HRPWM0_CSG0 {
    type Target = hrpwm0_csg0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRPWM0_CSG0::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub mod hrpwm0_csg0;
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_CSG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_CSG1 {}
impl HRPWM0_CSG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrpwm0_csg0::RegisterBlock {
        0x4002_0b00 as *const _
    }
}
impl Deref for HRPWM0_CSG1 {
    type Target = hrpwm0_csg0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRPWM0_CSG1::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_CSG2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_CSG2 {}
impl HRPWM0_CSG2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrpwm0_csg0::RegisterBlock {
        0x4002_0c00 as *const _
    }
}
impl Deref for HRPWM0_CSG2 {
    type Target = hrpwm0_csg0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRPWM0_CSG2::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_HRC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_HRC0 {}
impl HRPWM0_HRC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrpwm0_hrc0::RegisterBlock {
        0x4002_1300 as *const _
    }
}
impl Deref for HRPWM0_HRC0 {
    type Target = hrpwm0_hrc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRPWM0_HRC0::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub mod hrpwm0_hrc0;
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_HRC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_HRC1 {}
impl HRPWM0_HRC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrpwm0_hrc0::RegisterBlock {
        0x4002_1400 as *const _
    }
}
impl Deref for HRPWM0_HRC1 {
    type Target = hrpwm0_hrc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRPWM0_HRC1::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_HRC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_HRC2 {}
impl HRPWM0_HRC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrpwm0_hrc0::RegisterBlock {
        0x4002_1500 as *const _
    }
}
impl Deref for HRPWM0_HRC2 {
    type Target = hrpwm0_hrc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRPWM0_HRC2::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_HRC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_HRC3 {}
impl HRPWM0_HRC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrpwm0_hrc0::RegisterBlock {
        0x4002_1600 as *const _
    }
}
impl Deref for HRPWM0_HRC3 {
    type Target = hrpwm0_hrc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRPWM0_HRC3::ptr() }
    }
}
#[doc = "Position Interface 0"]
pub struct POSIF0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POSIF0 {}
impl POSIF0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const posif0::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for POSIF0 {
    type Target = posif0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*POSIF0::ptr() }
    }
}
#[doc = "Position Interface 0"]
pub mod posif0;
#[doc = "Port 0"]
pub struct PORT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT0 {}
impl PORT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port0::RegisterBlock {
        0x4802_8000 as *const _
    }
}
impl Deref for PORT0 {
    type Target = port0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT0::ptr() }
    }
}
#[doc = "Port 0"]
pub mod port0;
#[doc = "Port 1"]
pub struct PORT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT1 {}
impl PORT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port1::RegisterBlock {
        0x4802_8100 as *const _
    }
}
impl Deref for PORT1 {
    type Target = port1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT1::ptr() }
    }
}
#[doc = "Port 1"]
pub mod port1;
#[doc = "Port 2"]
pub struct PORT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT2 {}
impl PORT2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port2::RegisterBlock {
        0x4802_8200 as *const _
    }
}
impl Deref for PORT2 {
    type Target = port2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT2::ptr() }
    }
}
#[doc = "Port 2"]
pub mod port2;
#[doc = "Port 3"]
pub struct PORT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT3 {}
impl PORT3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port3::RegisterBlock {
        0x4802_8300 as *const _
    }
}
impl Deref for PORT3 {
    type Target = port3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT3::ptr() }
    }
}
#[doc = "Port 3"]
pub mod port3;
#[doc = "Port 14"]
pub struct PORT14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT14 {}
impl PORT14 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port14::RegisterBlock {
        0x4802_8e00 as *const _
    }
}
impl Deref for PORT14 {
    type Target = port14::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT14::ptr() }
    }
}
#[doc = "Port 14"]
pub mod port14;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PPB"]
    pub PPB: PPB,
    #[doc = "DLR"]
    pub DLR: DLR,
    #[doc = "ERU0"]
    pub ERU0: ERU0,
    #[doc = "ERU1"]
    pub ERU1: ERU1,
    #[doc = "GPDMA0"]
    pub GPDMA0: GPDMA0,
    #[doc = "GPDMA0_CH0"]
    pub GPDMA0_CH0: GPDMA0_CH0,
    #[doc = "GPDMA0_CH1"]
    pub GPDMA0_CH1: GPDMA0_CH1,
    #[doc = "GPDMA0_CH2"]
    pub GPDMA0_CH2: GPDMA0_CH2,
    #[doc = "GPDMA0_CH3"]
    pub GPDMA0_CH3: GPDMA0_CH3,
    #[doc = "GPDMA0_CH4"]
    pub GPDMA0_CH4: GPDMA0_CH4,
    #[doc = "GPDMA0_CH5"]
    pub GPDMA0_CH5: GPDMA0_CH5,
    #[doc = "GPDMA0_CH6"]
    pub GPDMA0_CH6: GPDMA0_CH6,
    #[doc = "GPDMA0_CH7"]
    pub GPDMA0_CH7: GPDMA0_CH7,
    #[doc = "FCE"]
    pub FCE: FCE,
    #[doc = "FCE_KE0"]
    pub FCE_KE0: FCE_KE0,
    #[doc = "FCE_KE1"]
    pub FCE_KE1: FCE_KE1,
    #[doc = "FCE_KE2"]
    pub FCE_KE2: FCE_KE2,
    #[doc = "FCE_KE3"]
    pub FCE_KE3: FCE_KE3,
    #[doc = "PBA0"]
    pub PBA0: PBA0,
    #[doc = "PBA1"]
    pub PBA1: PBA1,
    #[doc = "FLASH0"]
    pub FLASH0: FLASH0,
    #[doc = "PREF"]
    pub PREF: PREF,
    #[doc = "PMU0"]
    pub PMU0: PMU0,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SCU_CLK"]
    pub SCU_CLK: SCU_CLK,
    #[doc = "SCU_OSC"]
    pub SCU_OSC: SCU_OSC,
    #[doc = "SCU_PLL"]
    pub SCU_PLL: SCU_PLL,
    #[doc = "SCU_GENERAL"]
    pub SCU_GENERAL: SCU_GENERAL,
    #[doc = "SCU_INTERRUPT"]
    pub SCU_INTERRUPT: SCU_INTERRUPT,
    #[doc = "SCU_PARITY"]
    pub SCU_PARITY: SCU_PARITY,
    #[doc = "SCU_TRAP"]
    pub SCU_TRAP: SCU_TRAP,
    #[doc = "SCU_HIBERNATE"]
    pub SCU_HIBERNATE: SCU_HIBERNATE,
    #[doc = "SCU_POWER"]
    pub SCU_POWER: SCU_POWER,
    #[doc = "SCU_RESET"]
    pub SCU_RESET: SCU_RESET,
    #[doc = "LEDTS0"]
    pub LEDTS0: LEDTS0,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "USB0_EP0"]
    pub USB0_EP0: USB0_EP0,
    #[doc = "USB0_EP1"]
    pub USB0_EP1: USB0_EP1,
    #[doc = "USB0_EP2"]
    pub USB0_EP2: USB0_EP2,
    #[doc = "USB0_EP3"]
    pub USB0_EP3: USB0_EP3,
    #[doc = "USB0_EP4"]
    pub USB0_EP4: USB0_EP4,
    #[doc = "USB0_EP5"]
    pub USB0_EP5: USB0_EP5,
    #[doc = "USB0_EP6"]
    pub USB0_EP6: USB0_EP6,
    #[doc = "USIC0"]
    pub USIC0: USIC0,
    #[doc = "USIC1"]
    pub USIC1: USIC1,
    #[doc = "USIC0_CH0"]
    pub USIC0_CH0: USIC0_CH0,
    #[doc = "USIC0_CH1"]
    pub USIC0_CH1: USIC0_CH1,
    #[doc = "USIC1_CH0"]
    pub USIC1_CH0: USIC1_CH0,
    #[doc = "USIC1_CH1"]
    pub USIC1_CH1: USIC1_CH1,
    #[doc = "CAN"]
    pub CAN: CAN,
    #[doc = "CAN_NODE0"]
    pub CAN_NODE0: CAN_NODE0,
    #[doc = "CAN_NODE1"]
    pub CAN_NODE1: CAN_NODE1,
    #[doc = "CAN_MO0"]
    pub CAN_MO0: CAN_MO0,
    #[doc = "CAN_MO1"]
    pub CAN_MO1: CAN_MO1,
    #[doc = "CAN_MO2"]
    pub CAN_MO2: CAN_MO2,
    #[doc = "CAN_MO3"]
    pub CAN_MO3: CAN_MO3,
    #[doc = "CAN_MO4"]
    pub CAN_MO4: CAN_MO4,
    #[doc = "CAN_MO5"]
    pub CAN_MO5: CAN_MO5,
    #[doc = "CAN_MO6"]
    pub CAN_MO6: CAN_MO6,
    #[doc = "CAN_MO7"]
    pub CAN_MO7: CAN_MO7,
    #[doc = "CAN_MO8"]
    pub CAN_MO8: CAN_MO8,
    #[doc = "CAN_MO9"]
    pub CAN_MO9: CAN_MO9,
    #[doc = "CAN_MO10"]
    pub CAN_MO10: CAN_MO10,
    #[doc = "CAN_MO11"]
    pub CAN_MO11: CAN_MO11,
    #[doc = "CAN_MO12"]
    pub CAN_MO12: CAN_MO12,
    #[doc = "CAN_MO13"]
    pub CAN_MO13: CAN_MO13,
    #[doc = "CAN_MO14"]
    pub CAN_MO14: CAN_MO14,
    #[doc = "CAN_MO15"]
    pub CAN_MO15: CAN_MO15,
    #[doc = "CAN_MO16"]
    pub CAN_MO16: CAN_MO16,
    #[doc = "CAN_MO17"]
    pub CAN_MO17: CAN_MO17,
    #[doc = "CAN_MO18"]
    pub CAN_MO18: CAN_MO18,
    #[doc = "CAN_MO19"]
    pub CAN_MO19: CAN_MO19,
    #[doc = "CAN_MO20"]
    pub CAN_MO20: CAN_MO20,
    #[doc = "CAN_MO21"]
    pub CAN_MO21: CAN_MO21,
    #[doc = "CAN_MO22"]
    pub CAN_MO22: CAN_MO22,
    #[doc = "CAN_MO23"]
    pub CAN_MO23: CAN_MO23,
    #[doc = "CAN_MO24"]
    pub CAN_MO24: CAN_MO24,
    #[doc = "CAN_MO25"]
    pub CAN_MO25: CAN_MO25,
    #[doc = "CAN_MO26"]
    pub CAN_MO26: CAN_MO26,
    #[doc = "CAN_MO27"]
    pub CAN_MO27: CAN_MO27,
    #[doc = "CAN_MO28"]
    pub CAN_MO28: CAN_MO28,
    #[doc = "CAN_MO29"]
    pub CAN_MO29: CAN_MO29,
    #[doc = "CAN_MO30"]
    pub CAN_MO30: CAN_MO30,
    #[doc = "CAN_MO31"]
    pub CAN_MO31: CAN_MO31,
    #[doc = "CAN_MO32"]
    pub CAN_MO32: CAN_MO32,
    #[doc = "CAN_MO33"]
    pub CAN_MO33: CAN_MO33,
    #[doc = "CAN_MO34"]
    pub CAN_MO34: CAN_MO34,
    #[doc = "CAN_MO35"]
    pub CAN_MO35: CAN_MO35,
    #[doc = "CAN_MO36"]
    pub CAN_MO36: CAN_MO36,
    #[doc = "CAN_MO37"]
    pub CAN_MO37: CAN_MO37,
    #[doc = "CAN_MO38"]
    pub CAN_MO38: CAN_MO38,
    #[doc = "CAN_MO39"]
    pub CAN_MO39: CAN_MO39,
    #[doc = "CAN_MO40"]
    pub CAN_MO40: CAN_MO40,
    #[doc = "CAN_MO41"]
    pub CAN_MO41: CAN_MO41,
    #[doc = "CAN_MO42"]
    pub CAN_MO42: CAN_MO42,
    #[doc = "CAN_MO43"]
    pub CAN_MO43: CAN_MO43,
    #[doc = "CAN_MO44"]
    pub CAN_MO44: CAN_MO44,
    #[doc = "CAN_MO45"]
    pub CAN_MO45: CAN_MO45,
    #[doc = "CAN_MO46"]
    pub CAN_MO46: CAN_MO46,
    #[doc = "CAN_MO47"]
    pub CAN_MO47: CAN_MO47,
    #[doc = "CAN_MO48"]
    pub CAN_MO48: CAN_MO48,
    #[doc = "CAN_MO49"]
    pub CAN_MO49: CAN_MO49,
    #[doc = "CAN_MO50"]
    pub CAN_MO50: CAN_MO50,
    #[doc = "CAN_MO51"]
    pub CAN_MO51: CAN_MO51,
    #[doc = "CAN_MO52"]
    pub CAN_MO52: CAN_MO52,
    #[doc = "CAN_MO53"]
    pub CAN_MO53: CAN_MO53,
    #[doc = "CAN_MO54"]
    pub CAN_MO54: CAN_MO54,
    #[doc = "CAN_MO55"]
    pub CAN_MO55: CAN_MO55,
    #[doc = "CAN_MO56"]
    pub CAN_MO56: CAN_MO56,
    #[doc = "CAN_MO57"]
    pub CAN_MO57: CAN_MO57,
    #[doc = "CAN_MO58"]
    pub CAN_MO58: CAN_MO58,
    #[doc = "CAN_MO59"]
    pub CAN_MO59: CAN_MO59,
    #[doc = "CAN_MO60"]
    pub CAN_MO60: CAN_MO60,
    #[doc = "CAN_MO61"]
    pub CAN_MO61: CAN_MO61,
    #[doc = "CAN_MO62"]
    pub CAN_MO62: CAN_MO62,
    #[doc = "CAN_MO63"]
    pub CAN_MO63: CAN_MO63,
    #[doc = "VADC"]
    pub VADC: VADC,
    #[doc = "VADC_G0"]
    pub VADC_G0: VADC_G0,
    #[doc = "VADC_G1"]
    pub VADC_G1: VADC_G1,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "CCU40"]
    pub CCU40: CCU40,
    #[doc = "CCU41"]
    pub CCU41: CCU41,
    #[doc = "CCU40_CC40"]
    pub CCU40_CC40: CCU40_CC40,
    #[doc = "CCU40_CC41"]
    pub CCU40_CC41: CCU40_CC41,
    #[doc = "CCU40_CC42"]
    pub CCU40_CC42: CCU40_CC42,
    #[doc = "CCU40_CC43"]
    pub CCU40_CC43: CCU40_CC43,
    #[doc = "CCU41_CC40"]
    pub CCU41_CC40: CCU41_CC40,
    #[doc = "CCU41_CC41"]
    pub CCU41_CC41: CCU41_CC41,
    #[doc = "CCU41_CC42"]
    pub CCU41_CC42: CCU41_CC42,
    #[doc = "CCU41_CC43"]
    pub CCU41_CC43: CCU41_CC43,
    #[doc = "CCU80"]
    pub CCU80: CCU80,
    #[doc = "CCU80_CC80"]
    pub CCU80_CC80: CCU80_CC80,
    #[doc = "CCU80_CC81"]
    pub CCU80_CC81: CCU80_CC81,
    #[doc = "CCU80_CC82"]
    pub CCU80_CC82: CCU80_CC82,
    #[doc = "CCU80_CC83"]
    pub CCU80_CC83: CCU80_CC83,
    #[doc = "HRPWM0"]
    pub HRPWM0: HRPWM0,
    #[doc = "HRPWM0_CSG0"]
    pub HRPWM0_CSG0: HRPWM0_CSG0,
    #[doc = "HRPWM0_CSG1"]
    pub HRPWM0_CSG1: HRPWM0_CSG1,
    #[doc = "HRPWM0_CSG2"]
    pub HRPWM0_CSG2: HRPWM0_CSG2,
    #[doc = "HRPWM0_HRC0"]
    pub HRPWM0_HRC0: HRPWM0_HRC0,
    #[doc = "HRPWM0_HRC1"]
    pub HRPWM0_HRC1: HRPWM0_HRC1,
    #[doc = "HRPWM0_HRC2"]
    pub HRPWM0_HRC2: HRPWM0_HRC2,
    #[doc = "HRPWM0_HRC3"]
    pub HRPWM0_HRC3: HRPWM0_HRC3,
    #[doc = "POSIF0"]
    pub POSIF0: POSIF0,
    #[doc = "PORT0"]
    pub PORT0: PORT0,
    #[doc = "PORT1"]
    pub PORT1: PORT1,
    #[doc = "PORT2"]
    pub PORT2: PORT2,
    #[doc = "PORT3"]
    pub PORT3: PORT3,
    #[doc = "PORT14"]
    pub PORT14: PORT14,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } { None } else { Some(unsafe { Peripherals::steal() }) })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PPB: PPB { _marker: PhantomData },
            DLR: DLR { _marker: PhantomData },
            ERU0: ERU0 { _marker: PhantomData },
            ERU1: ERU1 { _marker: PhantomData },
            GPDMA0: GPDMA0 { _marker: PhantomData },
            GPDMA0_CH0: GPDMA0_CH0 { _marker: PhantomData },
            GPDMA0_CH1: GPDMA0_CH1 { _marker: PhantomData },
            GPDMA0_CH2: GPDMA0_CH2 { _marker: PhantomData },
            GPDMA0_CH3: GPDMA0_CH3 { _marker: PhantomData },
            GPDMA0_CH4: GPDMA0_CH4 { _marker: PhantomData },
            GPDMA0_CH5: GPDMA0_CH5 { _marker: PhantomData },
            GPDMA0_CH6: GPDMA0_CH6 { _marker: PhantomData },
            GPDMA0_CH7: GPDMA0_CH7 { _marker: PhantomData },
            FCE: FCE { _marker: PhantomData },
            FCE_KE0: FCE_KE0 { _marker: PhantomData },
            FCE_KE1: FCE_KE1 { _marker: PhantomData },
            FCE_KE2: FCE_KE2 { _marker: PhantomData },
            FCE_KE3: FCE_KE3 { _marker: PhantomData },
            PBA0: PBA0 { _marker: PhantomData },
            PBA1: PBA1 { _marker: PhantomData },
            FLASH0: FLASH0 { _marker: PhantomData },
            PREF: PREF { _marker: PhantomData },
            PMU0: PMU0 { _marker: PhantomData },
            WDT: WDT { _marker: PhantomData },
            RTC: RTC { _marker: PhantomData },
            SCU_CLK: SCU_CLK { _marker: PhantomData },
            SCU_OSC: SCU_OSC { _marker: PhantomData },
            SCU_PLL: SCU_PLL { _marker: PhantomData },
            SCU_GENERAL: SCU_GENERAL { _marker: PhantomData },
            SCU_INTERRUPT: SCU_INTERRUPT { _marker: PhantomData },
            SCU_PARITY: SCU_PARITY { _marker: PhantomData },
            SCU_TRAP: SCU_TRAP { _marker: PhantomData },
            SCU_HIBERNATE: SCU_HIBERNATE { _marker: PhantomData },
            SCU_POWER: SCU_POWER { _marker: PhantomData },
            SCU_RESET: SCU_RESET { _marker: PhantomData },
            LEDTS0: LEDTS0 { _marker: PhantomData },
            USB0: USB0 { _marker: PhantomData },
            USB0_EP0: USB0_EP0 { _marker: PhantomData },
            USB0_EP1: USB0_EP1 { _marker: PhantomData },
            USB0_EP2: USB0_EP2 { _marker: PhantomData },
            USB0_EP3: USB0_EP3 { _marker: PhantomData },
            USB0_EP4: USB0_EP4 { _marker: PhantomData },
            USB0_EP5: USB0_EP5 { _marker: PhantomData },
            USB0_EP6: USB0_EP6 { _marker: PhantomData },
            USIC0: USIC0 { _marker: PhantomData },
            USIC1: USIC1 { _marker: PhantomData },
            USIC0_CH0: USIC0_CH0 { _marker: PhantomData },
            USIC0_CH1: USIC0_CH1 { _marker: PhantomData },
            USIC1_CH0: USIC1_CH0 { _marker: PhantomData },
            USIC1_CH1: USIC1_CH1 { _marker: PhantomData },
            CAN: CAN { _marker: PhantomData },
            CAN_NODE0: CAN_NODE0 { _marker: PhantomData },
            CAN_NODE1: CAN_NODE1 { _marker: PhantomData },
            CAN_MO0: CAN_MO0 { _marker: PhantomData },
            CAN_MO1: CAN_MO1 { _marker: PhantomData },
            CAN_MO2: CAN_MO2 { _marker: PhantomData },
            CAN_MO3: CAN_MO3 { _marker: PhantomData },
            CAN_MO4: CAN_MO4 { _marker: PhantomData },
            CAN_MO5: CAN_MO5 { _marker: PhantomData },
            CAN_MO6: CAN_MO6 { _marker: PhantomData },
            CAN_MO7: CAN_MO7 { _marker: PhantomData },
            CAN_MO8: CAN_MO8 { _marker: PhantomData },
            CAN_MO9: CAN_MO9 { _marker: PhantomData },
            CAN_MO10: CAN_MO10 { _marker: PhantomData },
            CAN_MO11: CAN_MO11 { _marker: PhantomData },
            CAN_MO12: CAN_MO12 { _marker: PhantomData },
            CAN_MO13: CAN_MO13 { _marker: PhantomData },
            CAN_MO14: CAN_MO14 { _marker: PhantomData },
            CAN_MO15: CAN_MO15 { _marker: PhantomData },
            CAN_MO16: CAN_MO16 { _marker: PhantomData },
            CAN_MO17: CAN_MO17 { _marker: PhantomData },
            CAN_MO18: CAN_MO18 { _marker: PhantomData },
            CAN_MO19: CAN_MO19 { _marker: PhantomData },
            CAN_MO20: CAN_MO20 { _marker: PhantomData },
            CAN_MO21: CAN_MO21 { _marker: PhantomData },
            CAN_MO22: CAN_MO22 { _marker: PhantomData },
            CAN_MO23: CAN_MO23 { _marker: PhantomData },
            CAN_MO24: CAN_MO24 { _marker: PhantomData },
            CAN_MO25: CAN_MO25 { _marker: PhantomData },
            CAN_MO26: CAN_MO26 { _marker: PhantomData },
            CAN_MO27: CAN_MO27 { _marker: PhantomData },
            CAN_MO28: CAN_MO28 { _marker: PhantomData },
            CAN_MO29: CAN_MO29 { _marker: PhantomData },
            CAN_MO30: CAN_MO30 { _marker: PhantomData },
            CAN_MO31: CAN_MO31 { _marker: PhantomData },
            CAN_MO32: CAN_MO32 { _marker: PhantomData },
            CAN_MO33: CAN_MO33 { _marker: PhantomData },
            CAN_MO34: CAN_MO34 { _marker: PhantomData },
            CAN_MO35: CAN_MO35 { _marker: PhantomData },
            CAN_MO36: CAN_MO36 { _marker: PhantomData },
            CAN_MO37: CAN_MO37 { _marker: PhantomData },
            CAN_MO38: CAN_MO38 { _marker: PhantomData },
            CAN_MO39: CAN_MO39 { _marker: PhantomData },
            CAN_MO40: CAN_MO40 { _marker: PhantomData },
            CAN_MO41: CAN_MO41 { _marker: PhantomData },
            CAN_MO42: CAN_MO42 { _marker: PhantomData },
            CAN_MO43: CAN_MO43 { _marker: PhantomData },
            CAN_MO44: CAN_MO44 { _marker: PhantomData },
            CAN_MO45: CAN_MO45 { _marker: PhantomData },
            CAN_MO46: CAN_MO46 { _marker: PhantomData },
            CAN_MO47: CAN_MO47 { _marker: PhantomData },
            CAN_MO48: CAN_MO48 { _marker: PhantomData },
            CAN_MO49: CAN_MO49 { _marker: PhantomData },
            CAN_MO50: CAN_MO50 { _marker: PhantomData },
            CAN_MO51: CAN_MO51 { _marker: PhantomData },
            CAN_MO52: CAN_MO52 { _marker: PhantomData },
            CAN_MO53: CAN_MO53 { _marker: PhantomData },
            CAN_MO54: CAN_MO54 { _marker: PhantomData },
            CAN_MO55: CAN_MO55 { _marker: PhantomData },
            CAN_MO56: CAN_MO56 { _marker: PhantomData },
            CAN_MO57: CAN_MO57 { _marker: PhantomData },
            CAN_MO58: CAN_MO58 { _marker: PhantomData },
            CAN_MO59: CAN_MO59 { _marker: PhantomData },
            CAN_MO60: CAN_MO60 { _marker: PhantomData },
            CAN_MO61: CAN_MO61 { _marker: PhantomData },
            CAN_MO62: CAN_MO62 { _marker: PhantomData },
            CAN_MO63: CAN_MO63 { _marker: PhantomData },
            VADC: VADC { _marker: PhantomData },
            VADC_G0: VADC_G0 { _marker: PhantomData },
            VADC_G1: VADC_G1 { _marker: PhantomData },
            DAC: DAC { _marker: PhantomData },
            CCU40: CCU40 { _marker: PhantomData },
            CCU41: CCU41 { _marker: PhantomData },
            CCU40_CC40: CCU40_CC40 { _marker: PhantomData },
            CCU40_CC41: CCU40_CC41 { _marker: PhantomData },
            CCU40_CC42: CCU40_CC42 { _marker: PhantomData },
            CCU40_CC43: CCU40_CC43 { _marker: PhantomData },
            CCU41_CC40: CCU41_CC40 { _marker: PhantomData },
            CCU41_CC41: CCU41_CC41 { _marker: PhantomData },
            CCU41_CC42: CCU41_CC42 { _marker: PhantomData },
            CCU41_CC43: CCU41_CC43 { _marker: PhantomData },
            CCU80: CCU80 { _marker: PhantomData },
            CCU80_CC80: CCU80_CC80 { _marker: PhantomData },
            CCU80_CC81: CCU80_CC81 { _marker: PhantomData },
            CCU80_CC82: CCU80_CC82 { _marker: PhantomData },
            CCU80_CC83: CCU80_CC83 { _marker: PhantomData },
            HRPWM0: HRPWM0 { _marker: PhantomData },
            HRPWM0_CSG0: HRPWM0_CSG0 { _marker: PhantomData },
            HRPWM0_CSG1: HRPWM0_CSG1 { _marker: PhantomData },
            HRPWM0_CSG2: HRPWM0_CSG2 { _marker: PhantomData },
            HRPWM0_HRC0: HRPWM0_HRC0 { _marker: PhantomData },
            HRPWM0_HRC1: HRPWM0_HRC1 { _marker: PhantomData },
            HRPWM0_HRC2: HRPWM0_HRC2 { _marker: PhantomData },
            HRPWM0_HRC3: HRPWM0_HRC3 { _marker: PhantomData },
            POSIF0: POSIF0 { _marker: PhantomData },
            PORT0: PORT0 { _marker: PhantomData },
            PORT1: PORT1 { _marker: PhantomData },
            PORT2: PORT2 { _marker: PhantomData },
            PORT3: PORT3 { _marker: PhantomData },
            PORT14: PORT14 { _marker: PhantomData },
        }
    }
}
