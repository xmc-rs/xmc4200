#[doc = "Register `GINTSTS` reader"]
pub type R = crate::R<GintstsSpec>;
#[doc = "Register `GINTSTS` writer"]
pub type W = crate::W<GintstsSpec>;
#[doc = "Current Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurMod {
    #[doc = "0: Device mode"]
    Value1 = 0,
}
impl From<CurMod> for bool {
    #[inline(always)]
    fn from(variant: CurMod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CurMod` reader - Current Mode of Operation"]
pub type CurModR = crate::BitReader<CurMod>;
impl CurModR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CurMod> {
        match self.bits {
            false => Some(CurMod::Value1),
            _ => None,
        }
    }
    #[doc = "Device mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CurMod::Value1
    }
}
#[doc = "Field `Sof` reader - Start of Frame"]
pub type SofR = crate::BitReader;
#[doc = "Field `Sof` writer - Start of Frame"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFLvl` reader - RxFIFO Non-Empty"]
pub type RxFlvlR = crate::BitReader;
#[doc = "Field `GINNakEff` reader - Global IN Non-Periodic NAK Effective"]
pub type GinnakEffR = crate::BitReader;
#[doc = "Field `GOUTNakEff` reader - Global OUT NAK Effective"]
pub type GoutnakEffR = crate::BitReader;
#[doc = "Field `ErlySusp` reader - Early Suspend"]
pub type ErlySuspR = crate::BitReader;
#[doc = "Field `ErlySusp` writer - Early Suspend"]
pub type ErlySuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSusp` reader - USB Suspend"]
pub type UsbsuspR = crate::BitReader;
#[doc = "Field `USBSusp` writer - USB Suspend"]
pub type UsbsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRst` reader - USB Reset"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRst` writer - USB Reset"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnumDone` reader - Enumeration Done"]
pub type EnumDoneR = crate::BitReader;
#[doc = "Field `EnumDone` writer - Enumeration Done"]
pub type EnumDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOutDrop` reader - Isochronous OUT Packet Dropped Interrupt"]
pub type IsooutDropR = crate::BitReader;
#[doc = "Field `ISOOutDrop` writer - Isochronous OUT Packet Dropped Interrupt"]
pub type IsooutDropW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - End of Periodic Frame Interrupt"]
pub type EopfR = crate::BitReader;
#[doc = "Field `EOPF` writer - End of Periodic Frame Interrupt"]
pub type EopfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPInt` reader - IN Endpoints Interrupt"]
pub type IepintR = crate::BitReader;
#[doc = "Field `OEPInt` reader - OUT Endpoints Interrupt"]
pub type OepintR = crate::BitReader;
#[doc = "Field `incompISOIN` reader - Incomplete Isochronous IN Transfer"]
pub type IncompIsoinR = crate::BitReader;
#[doc = "Field `incompISOIN` writer - Incomplete Isochronous IN Transfer"]
pub type IncompIsoinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `incomplSOOUT` reader - Incomplete Isochronous OUT Transfer"]
pub type IncomplSooutR = crate::BitReader;
#[doc = "Field `incomplSOOUT` writer - Incomplete Isochronous OUT Transfer"]
pub type IncomplSooutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WkUpInt` reader - Resume/Remote Wakeup Detected Interrupt"]
pub type WkUpIntR = crate::BitReader;
#[doc = "Field `WkUpInt` writer - Resume/Remote Wakeup Detected Interrupt"]
pub type WkUpIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn cur_mod(&self) -> CurModR {
        CurModR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rx_flvl(&self) -> RxFlvlR {
        RxFlvlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN Non-Periodic NAK Effective"]
    #[inline(always)]
    pub fn ginnak_eff(&self) -> GinnakEffR {
        GinnakEffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective"]
    #[inline(always)]
    pub fn goutnak_eff(&self) -> GoutnakEffR {
        GoutnakEffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erly_susp(&self) -> ErlySuspR {
        ErlySuspR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> UsbsuspR {
        UsbsuspR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enum_done(&self) -> EnumDoneR {
        EnumDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    pub fn isoout_drop(&self) -> IsooutDropR {
        IsooutDropR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EopfR {
        EopfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        IepintR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incomp_isoin(&self) -> IncompIsoinR {
        IncompIsoinR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer"]
    #[inline(always)]
    pub fn incompl_soout(&self) -> IncomplSooutR {
        IncomplSooutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wk_up_int(&self) -> WkUpIntR {
        WkUpIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<GintstsSpec> {
        SofW::new(self, 3)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn erly_susp(&mut self) -> ErlySuspW<GintstsSpec> {
        ErlySuspW::new(self, 10)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp(&mut self) -> UsbsuspW<GintstsSpec> {
        UsbsuspW::new(self, 11)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<GintstsSpec> {
        UsbrstW::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    #[must_use]
    pub fn enum_done(&mut self) -> EnumDoneW<GintstsSpec> {
        EnumDoneW::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isoout_drop(&mut self) -> IsooutDropW<GintstsSpec> {
        IsooutDropW::new(self, 14)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eopf(&mut self) -> EopfW<GintstsSpec> {
        EopfW::new(self, 15)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incomp_isoin(&mut self) -> IncompIsoinW<GintstsSpec> {
        IncompIsoinW::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete Isochronous OUT Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incompl_soout(&mut self) -> IncomplSooutW<GintstsSpec> {
        IncomplSooutW::new(self, 21)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wk_up_int(&mut self) -> WkUpIntW<GintstsSpec> {
        WkUpIntW::new(self, 31)
    }
}
#[doc = "Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintstsSpec;
impl crate::RegisterSpec for GintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts::R`](R) reader structure"]
impl crate::Readable for GintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`gintsts::W`](W) writer structure"]
impl crate::Writable for GintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTSTS to value 0x1400_0020"]
impl crate::Resettable for GintstsSpec {
    const RESET_VALUE: u32 = 0x1400_0020;
}
