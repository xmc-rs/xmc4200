#[doc = "Register `LPACCONF` reader"]
pub type R = crate::R<LpacconfSpec>;
#[doc = "Register `LPACCONF` writer"]
pub type W = crate::W<LpacconfSpec>;
#[doc = "Compare Enable for Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpen {
    #[doc = "0: Comparator permanently in power down"]
    Value1 = 0,
    #[doc = "1: Comparator activated for VBAT input"]
    Value2 = 1,
    #[doc = "2: Comparator activated for HIB_IO_0 input"]
    Value3 = 2,
    #[doc = "4: Comparator activated for HIB_IO_1 input"]
    Value4 = 4,
}
impl From<Cmpen> for u8 {
    #[inline(always)]
    fn from(variant: Cmpen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpen {
    type Ux = u8;
}
impl crate::IsEnum for Cmpen {}
#[doc = "Field `CMPEN` reader - Compare Enable for Input Selection"]
pub type CmpenR = crate::FieldReader<Cmpen>;
impl CmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmpen> {
        match self.bits {
            0 => Some(Cmpen::Value1),
            1 => Some(Cmpen::Value2),
            2 => Some(Cmpen::Value3),
            4 => Some(Cmpen::Value4),
            _ => None,
        }
    }
    #[doc = "Comparator permanently in power down"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cmpen::Value1
    }
    #[doc = "Comparator activated for VBAT input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cmpen::Value2
    }
    #[doc = "Comparator activated for HIB_IO_0 input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cmpen::Value3
    }
    #[doc = "Comparator activated for HIB_IO_1 input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cmpen::Value4
    }
}
#[doc = "Field `CMPEN` writer - Compare Enable for Input Selection"]
pub type CmpenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmpen>;
impl<'a, REG> CmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator permanently in power down"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpen::Value1)
    }
    #[doc = "Comparator activated for VBAT input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpen::Value2)
    }
    #[doc = "Comparator activated for HIB_IO_0 input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpen::Value3)
    }
    #[doc = "Comparator activated for HIB_IO_1 input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpen::Value4)
    }
}
#[doc = "Analog Compare Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsel {
    #[doc = "0: Sub-second interval counter"]
    Value1 = 0,
    #[doc = "1: RTC alarm event"]
    Value2 = 1,
    #[doc = "2: RTC periodic event"]
    Value3 = 2,
    #[doc = "3: On digital WKUP input positive edge event"]
    Value4 = 3,
    #[doc = "5: On digital WKUP input negative edge event"]
    Value5 = 5,
    #[doc = "6: Continuous measurement"]
    Value6 = 6,
    #[doc = "7: Single-shot on software request"]
    Value7 = 7,
}
impl From<Trigsel> for u8 {
    #[inline(always)]
    fn from(variant: Trigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsel {
    type Ux = u8;
}
impl crate::IsEnum for Trigsel {}
#[doc = "Field `TRIGSEL` reader - Analog Compare Trigger Select"]
pub type TrigselR = crate::FieldReader<Trigsel>;
impl TrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigsel> {
        match self.bits {
            0 => Some(Trigsel::Value1),
            1 => Some(Trigsel::Value2),
            2 => Some(Trigsel::Value3),
            3 => Some(Trigsel::Value4),
            5 => Some(Trigsel::Value5),
            6 => Some(Trigsel::Value6),
            7 => Some(Trigsel::Value7),
            _ => None,
        }
    }
    #[doc = "Sub-second interval counter"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trigsel::Value1
    }
    #[doc = "RTC alarm event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trigsel::Value2
    }
    #[doc = "RTC periodic event"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Trigsel::Value3
    }
    #[doc = "On digital WKUP input positive edge event"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Trigsel::Value4
    }
    #[doc = "On digital WKUP input negative edge event"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Trigsel::Value5
    }
    #[doc = "Continuous measurement"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Trigsel::Value6
    }
    #[doc = "Single-shot on software request"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Trigsel::Value7
    }
}
#[doc = "Field `TRIGSEL` writer - Analog Compare Trigger Select"]
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trigsel>;
impl<'a, REG> TrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sub-second interval counter"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Value1)
    }
    #[doc = "RTC alarm event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Value2)
    }
    #[doc = "RTC periodic event"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Value3)
    }
    #[doc = "On digital WKUP input positive edge event"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Value4)
    }
    #[doc = "On digital WKUP input negative edge event"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Value5)
    }
    #[doc = "Continuous measurement"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Value6)
    }
    #[doc = "Single-shot on software request"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Value7)
    }
}
#[doc = "Field `CONVDEL` reader - Conversion Delay"]
pub type ConvdelR = crate::BitReader;
#[doc = "Field `CONVDEL` writer - Conversion Delay"]
pub type ConvdelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERVCNT` reader - Sub-second Interval Counter"]
pub type IntervcntR = crate::FieldReader<u16>;
#[doc = "Field `INTERVCNT` writer - Sub-second Interval Counter"]
pub type IntervcntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SETTLECNT` reader - LPAC Settle Time Counter"]
pub type SettlecntR = crate::FieldReader;
#[doc = "Field `SETTLECNT` writer - LPAC Settle Time Counter"]
pub type SettlecntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline(always)]
    pub fn convdel(&self) -> ConvdelR {
        ConvdelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline(always)]
    pub fn intervcnt(&self) -> IntervcntR {
        IntervcntR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline(always)]
    pub fn settlecnt(&self) -> SettlecntR {
        SettlecntR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CmpenW<LpacconfSpec> {
        CmpenW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TrigselW<LpacconfSpec> {
        TrigselW::new(self, 4)
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline(always)]
    #[must_use]
    pub fn convdel(&mut self) -> ConvdelW<LpacconfSpec> {
        ConvdelW::new(self, 12)
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline(always)]
    #[must_use]
    pub fn intervcnt(&mut self) -> IntervcntW<LpacconfSpec> {
        IntervcntW::new(self, 16)
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline(always)]
    #[must_use]
    pub fn settlecnt(&mut self) -> SettlecntW<LpacconfSpec> {
        SettlecntW::new(self, 28)
    }
}
#[doc = "Analog Wake-up Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpacconfSpec;
impl crate::RegisterSpec for LpacconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpacconf::R`](R) reader structure"]
impl crate::Readable for LpacconfSpec {}
#[doc = "`write(|w| ..)` method takes [`lpacconf::W`](W) writer structure"]
impl crate::Writable for LpacconfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPACCONF to value 0x7000_0000"]
impl crate::Resettable for LpacconfSpec {
    const RESET_VALUE: u32 = 0x7000_0000;
}
