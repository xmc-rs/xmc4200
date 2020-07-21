#[doc = "Reader of register GUSBCFG"]
pub type R = crate::R<u32, super::GUSBCFG>;
#[doc = "Writer for register GUSBCFG"]
pub type W = crate::W<u32, super::GUSBCFG>;
#[doc = "Register GUSBCFG `reset()`'s with value 0x1440"]
impl crate::ResetValue for super::GUSBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1440
    }
}
#[doc = "Reader of field `TOutCal`"]
pub type TOUTCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOutCal`"]
pub struct TOUTCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "USB 1.1 Full-Speed Serial Transceiver Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYSEL_A {
    #[doc = "1: USB 1.1 full-speed serial transceiver"]
    VALUE2 = 1,
}
impl From<PHYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PHYSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHYSel`"]
pub type PHYSEL_R = crate::R<bool, PHYSEL_A>;
impl PHYSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PHYSEL_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PHYSEL_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PHYSEL_A::VALUE2
    }
}
#[doc = "Reader of field `USBTrdTim`"]
pub type USBTRDTIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBTrdTim`"]
pub struct USBTRDTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBTRDTIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Tx End Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDDELAY_A {
    #[doc = "0: Normal mode"]
    VALUE1 = 0,
    #[doc = "1: Introduce Tx end delay timers"]
    VALUE2 = 1,
}
impl From<TXENDDELAY_A> for bool {
    #[inline(always)]
    fn from(variant: TXENDDELAY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TxEndDelay`"]
pub type TXENDDELAY_R = crate::R<bool, TXENDDELAY_A>;
impl TXENDDELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENDDELAY_A {
        match self.bits {
            false => TXENDDELAY_A::VALUE1,
            true => TXENDDELAY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXENDDELAY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXENDDELAY_A::VALUE2
    }
}
#[doc = "Write proxy for field `TxEndDelay`"]
pub struct TXENDDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXENDDELAY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXENDDELAY_A::VALUE1)
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXENDDELAY_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Force Device Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEDEVMODE_A {
    #[doc = "0: Normal Mode"]
    VALUE1 = 0,
    #[doc = "1: Force Device Mode"]
    VALUE2 = 1,
}
impl From<FORCEDEVMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEDEVMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ForceDevMode`"]
pub type FORCEDEVMODE_R = crate::R<bool, FORCEDEVMODE_A>;
impl FORCEDEVMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEDEVMODE_A {
        match self.bits {
            false => FORCEDEVMODE_A::VALUE1,
            true => FORCEDEVMODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCEDEVMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCEDEVMODE_A::VALUE2
    }
}
#[doc = "Write proxy for field `ForceDevMode`"]
pub struct FORCEDEVMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEDEVMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEDEVMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FORCEDEVMODE_A::VALUE1)
    }
    #[doc = "Force Device Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FORCEDEVMODE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CTP`"]
pub type CTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTP`"]
pub struct CTP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    pub fn tout_cal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 6 - USB 1.1 Full-Speed Serial Transceiver Select"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrd_tim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn tx_end_delay(&self) -> TXENDDELAY_R {
        TXENDDELAY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    pub fn force_dev_mode(&self) -> FORCEDEVMODE_R {
        FORCEDEVMODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    pub fn tout_cal(&mut self) -> TOUTCAL_W {
        TOUTCAL_W { w: self }
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrd_tim(&mut self) -> USBTRDTIM_W {
        USBTRDTIM_W { w: self }
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn tx_end_delay(&mut self) -> TXENDDELAY_W {
        TXENDDELAY_W { w: self }
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    pub fn force_dev_mode(&mut self) -> FORCEDEVMODE_W {
        FORCEDEVMODE_W { w: self }
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctp(&mut self) -> CTP_W {
        CTP_W { w: self }
    }
}
