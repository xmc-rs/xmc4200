#[doc = "Register `CLKSTAT` reader"]
pub type R = crate::R<ClkstatSpec>;
#[doc = "USB Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcst {
    #[doc = "0: Clock disabled"]
    Value1 = 0,
    #[doc = "1: Clock enabled"]
    Value2 = 1,
}
impl From<Usbcst> for bool {
    #[inline(always)]
    fn from(variant: Usbcst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCST` reader - USB Clock Status"]
pub type UsbcstR = crate::BitReader<Usbcst>;
impl UsbcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbcst {
        match self.bits {
            false => Usbcst::Value1,
            true => Usbcst::Value2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usbcst::Value1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usbcst::Value2
    }
}
#[doc = "CCU Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccucst {
    #[doc = "0: Clock disabled"]
    Value1 = 0,
    #[doc = "1: Clock enabled"]
    Value2 = 1,
}
impl From<Ccucst> for bool {
    #[inline(always)]
    fn from(variant: Ccucst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCST` reader - CCU Clock Status"]
pub type CcucstR = crate::BitReader<Ccucst>;
impl CcucstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccucst {
        match self.bits {
            false => Ccucst::Value1,
            true => Ccucst::Value2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccucst::Value1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccucst::Value2
    }
}
#[doc = "WDT Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtcst {
    #[doc = "0: Clock disabled"]
    Value1 = 0,
    #[doc = "1: Clock enabled"]
    Value2 = 1,
}
impl From<Wdtcst> for bool {
    #[inline(always)]
    fn from(variant: Wdtcst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCST` reader - WDT Clock Status"]
pub type WdtcstR = crate::BitReader<Wdtcst>;
impl WdtcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtcst {
        match self.bits {
            false => Wdtcst::Value1,
            true => Wdtcst::Value2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wdtcst::Value1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wdtcst::Value2
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Status"]
    #[inline(always)]
    pub fn usbcst(&self) -> UsbcstR {
        UsbcstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CCU Clock Status"]
    #[inline(always)]
    pub fn ccucst(&self) -> CcucstR {
        CcucstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDT Clock Status"]
    #[inline(always)]
    pub fn wdtcst(&self) -> WdtcstR {
        WdtcstR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkstatSpec;
impl crate::RegisterSpec for ClkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkstat::R`](R) reader structure"]
impl crate::Readable for ClkstatSpec {}
#[doc = "`reset()` method sets CLKSTAT to value 0"]
impl crate::Resettable for ClkstatSpec {
    const RESET_VALUE: u32 = 0;
}
