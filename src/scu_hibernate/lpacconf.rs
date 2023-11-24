#[doc = "Register `LPACCONF` reader"]
pub type R = crate::R<LPACCONF_SPEC>;
#[doc = "Register `LPACCONF` writer"]
pub type W = crate::W<LPACCONF_SPEC>;
#[doc = "Field `CMPEN` reader - Compare Enable for Input Selection"]
pub type CMPEN_R = crate::FieldReader<CMPEN_A>;
#[doc = "Compare Enable for Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPEN_A {
    #[doc = "0: Comparator permanently in power down"]
    VALUE1 = 0,
    #[doc = "1: Comparator activated for VBAT input"]
    VALUE2 = 1,
    #[doc = "2: Comparator activated for HIB_IO_0 input"]
    VALUE3 = 2,
    #[doc = "4: Comparator activated for HIB_IO_1 input"]
    VALUE4 = 4,
}
impl From<CMPEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPEN_A {
    type Ux = u8;
}
impl CMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMPEN_A> {
        match self.bits {
            0 => Some(CMPEN_A::VALUE1),
            1 => Some(CMPEN_A::VALUE2),
            2 => Some(CMPEN_A::VALUE3),
            4 => Some(CMPEN_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Comparator permanently in power down"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMPEN_A::VALUE1
    }
    #[doc = "Comparator activated for VBAT input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMPEN_A::VALUE2
    }
    #[doc = "Comparator activated for HIB_IO_0 input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMPEN_A::VALUE3
    }
    #[doc = "Comparator activated for HIB_IO_1 input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMPEN_A::VALUE4
    }
}
#[doc = "Field `CMPEN` writer - Compare Enable for Input Selection"]
pub type CMPEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CMPEN_A>;
impl<'a, REG> CMPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator permanently in power down"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPEN_A::VALUE1)
    }
    #[doc = "Comparator activated for VBAT input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMPEN_A::VALUE2)
    }
    #[doc = "Comparator activated for HIB_IO_0 input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CMPEN_A::VALUE3)
    }
    #[doc = "Comparator activated for HIB_IO_1 input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CMPEN_A::VALUE4)
    }
}
#[doc = "Field `TRIGSEL` reader - Analog Compare Trigger Select"]
pub type TRIGSEL_R = crate::FieldReader<TRIGSEL_A>;
#[doc = "Analog Compare Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSEL_A {
    #[doc = "0: Sub-second interval counter"]
    VALUE1 = 0,
    #[doc = "1: RTC alarm event"]
    VALUE2 = 1,
    #[doc = "2: RTC periodic event"]
    VALUE3 = 2,
    #[doc = "3: On digital WKUP input positive edge event"]
    VALUE4 = 3,
    #[doc = "5: On digital WKUP input negative edge event"]
    VALUE5 = 5,
    #[doc = "6: Continuous measurement"]
    VALUE6 = 6,
    #[doc = "7: Single-shot on software request"]
    VALUE7 = 7,
}
impl From<TRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGSEL_A {
    type Ux = u8;
}
impl TRIGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRIGSEL_A> {
        match self.bits {
            0 => Some(TRIGSEL_A::VALUE1),
            1 => Some(TRIGSEL_A::VALUE2),
            2 => Some(TRIGSEL_A::VALUE3),
            3 => Some(TRIGSEL_A::VALUE4),
            5 => Some(TRIGSEL_A::VALUE5),
            6 => Some(TRIGSEL_A::VALUE6),
            7 => Some(TRIGSEL_A::VALUE7),
            _ => None,
        }
    }
    #[doc = "Sub-second interval counter"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRIGSEL_A::VALUE1
    }
    #[doc = "RTC alarm event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRIGSEL_A::VALUE2
    }
    #[doc = "RTC periodic event"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRIGSEL_A::VALUE3
    }
    #[doc = "On digital WKUP input positive edge event"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRIGSEL_A::VALUE4
    }
    #[doc = "On digital WKUP input negative edge event"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == TRIGSEL_A::VALUE5
    }
    #[doc = "Continuous measurement"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == TRIGSEL_A::VALUE6
    }
    #[doc = "Single-shot on software request"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == TRIGSEL_A::VALUE7
    }
}
#[doc = "Field `TRIGSEL` writer - Analog Compare Trigger Select"]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TRIGSEL_A>;
impl<'a, REG> TRIGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sub-second interval counter"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::VALUE1)
    }
    #[doc = "RTC alarm event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::VALUE2)
    }
    #[doc = "RTC periodic event"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::VALUE3)
    }
    #[doc = "On digital WKUP input positive edge event"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::VALUE4)
    }
    #[doc = "On digital WKUP input negative edge event"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::VALUE5)
    }
    #[doc = "Continuous measurement"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::VALUE6)
    }
    #[doc = "Single-shot on software request"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::VALUE7)
    }
}
#[doc = "Field `CONVDEL` reader - Conversion Delay"]
pub type CONVDEL_R = crate::BitReader;
#[doc = "Field `CONVDEL` writer - Conversion Delay"]
pub type CONVDEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERVCNT` reader - Sub-second Interval Counter"]
pub type INTERVCNT_R = crate::FieldReader<u16>;
#[doc = "Field `INTERVCNT` writer - Sub-second Interval Counter"]
pub type INTERVCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SETTLECNT` reader - LPAC Settle Time Counter"]
pub type SETTLECNT_R = crate::FieldReader;
#[doc = "Field `SETTLECNT` writer - LPAC Settle Time Counter"]
pub type SETTLECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline(always)]
    pub fn convdel(&self) -> CONVDEL_R {
        CONVDEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline(always)]
    pub fn intervcnt(&self) -> INTERVCNT_R {
        INTERVCNT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline(always)]
    pub fn settlecnt(&self) -> SETTLECNT_R {
        SETTLECNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<LPACCONF_SPEC> {
        CMPEN_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<LPACCONF_SPEC> {
        TRIGSEL_W::new(self, 4)
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline(always)]
    #[must_use]
    pub fn convdel(&mut self) -> CONVDEL_W<LPACCONF_SPEC> {
        CONVDEL_W::new(self, 12)
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline(always)]
    #[must_use]
    pub fn intervcnt(&mut self) -> INTERVCNT_W<LPACCONF_SPEC> {
        INTERVCNT_W::new(self, 16)
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline(always)]
    #[must_use]
    pub fn settlecnt(&mut self) -> SETTLECNT_W<LPACCONF_SPEC> {
        SETTLECNT_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Analog Wake-up Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPACCONF_SPEC;
impl crate::RegisterSpec for LPACCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpacconf::R`](R) reader structure"]
impl crate::Readable for LPACCONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpacconf::W`](W) writer structure"]
impl crate::Writable for LPACCONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPACCONF to value 0x7000_0000"]
impl crate::Resettable for LPACCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x7000_0000;
}
