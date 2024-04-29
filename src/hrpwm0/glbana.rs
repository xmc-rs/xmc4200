#[doc = "Register `GLBANA` reader"]
pub type R = crate::R<GLBANA_SPEC>;
#[doc = "Register `GLBANA` writer"]
pub type W = crate::W<GLBANA_SPEC>;
#[doc = "Field `SLDLY` reader - Delay of lock detection"]
pub type SLDLY_R = crate::FieldReader;
#[doc = "Field `SLDLY` writer - Delay of lock detection"]
pub type SLDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUP` reader - Force chargepump up"]
pub type FUP_R = crate::BitReader;
#[doc = "Field `FUP` writer - Force chargepump up"]
pub type FUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDN` reader - Force chargepump down"]
pub type FDN_R = crate::BitReader;
#[doc = "Field `FDN` writer - Force chargepump down"]
pub type FDN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLCP` reader - HRCs chargepump current selection"]
pub type SLCP_R = crate::FieldReader;
#[doc = "Field `SLCP` writer - HRCs chargepump current selection"]
pub type SLCP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SLIBLDO` reader - HRCs LDO bias current"]
pub type SLIBLDO_R = crate::FieldReader;
#[doc = "Field `SLIBLDO` writer - HRCs LDO bias current"]
pub type SLIBLDO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLIBLF` reader - HRCs loop filter bias current"]
pub type SLIBLF_R = crate::FieldReader;
#[doc = "Field `SLIBLF` writer - HRCs loop filter bias current"]
pub type SLIBLF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLVREF` reader - Reference voltage for chargepump and loop filter"]
pub type SLVREF_R = crate::FieldReader;
#[doc = "Field `SLVREF` writer - Reference voltage for chargepump and loop filter"]
pub type SLVREF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIBIAS` reader - Bias trimming"]
pub type TRIBIAS_R = crate::FieldReader;
#[doc = "Field `TRIBIAS` writer - Bias trimming"]
pub type TRIBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Force chargepump down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GHREN_A {
    #[doc = "0: Global high resolution generation is enabled"]
    VALUE1 = 0,
    #[doc = "1: Global high resolution generation is disabled"]
    VALUE2 = 1,
}
impl From<GHREN_A> for bool {
    #[inline(always)]
    fn from(variant: GHREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GHREN` reader - Force chargepump down"]
pub type GHREN_R = crate::BitReader<GHREN_A>;
impl GHREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GHREN_A {
        match self.bits {
            false => GHREN_A::VALUE1,
            true => GHREN_A::VALUE2,
        }
    }
    #[doc = "Global high resolution generation is enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GHREN_A::VALUE1
    }
    #[doc = "Global high resolution generation is disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GHREN_A::VALUE2
    }
}
#[doc = "Field `GHREN` writer - Force chargepump down"]
pub type GHREN_W<'a, REG> = crate::BitWriter<'a, REG, GHREN_A>;
impl<'a, REG> GHREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Global high resolution generation is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GHREN_A::VALUE1)
    }
    #[doc = "Global high resolution generation is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GHREN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline(always)]
    pub fn sldly(&self) -> SLDLY_R {
        SLDLY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline(always)]
    pub fn fdn(&self) -> FDN_R {
        FDN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline(always)]
    pub fn slcp(&self) -> SLCP_R {
        SLCP_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline(always)]
    pub fn slibldo(&self) -> SLIBLDO_R {
        SLIBLDO_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline(always)]
    pub fn sliblf(&self) -> SLIBLF_R {
        SLIBLF_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline(always)]
    pub fn slvref(&self) -> SLVREF_R {
        SLVREF_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline(always)]
    pub fn tribias(&self) -> TRIBIAS_R {
        TRIBIAS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline(always)]
    pub fn ghren(&self) -> GHREN_R {
        GHREN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline(always)]
    #[must_use]
    pub fn sldly(&mut self) -> SLDLY_W<GLBANA_SPEC> {
        SLDLY_W::new(self, 0)
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline(always)]
    #[must_use]
    pub fn fup(&mut self) -> FUP_W<GLBANA_SPEC> {
        FUP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline(always)]
    #[must_use]
    pub fn fdn(&mut self) -> FDN_W<GLBANA_SPEC> {
        FDN_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline(always)]
    #[must_use]
    pub fn slcp(&mut self) -> SLCP_W<GLBANA_SPEC> {
        SLCP_W::new(self, 6)
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline(always)]
    #[must_use]
    pub fn slibldo(&mut self) -> SLIBLDO_W<GLBANA_SPEC> {
        SLIBLDO_W::new(self, 9)
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline(always)]
    #[must_use]
    pub fn sliblf(&mut self) -> SLIBLF_W<GLBANA_SPEC> {
        SLIBLF_W::new(self, 11)
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline(always)]
    #[must_use]
    pub fn slvref(&mut self) -> SLVREF_W<GLBANA_SPEC> {
        SLVREF_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline(always)]
    #[must_use]
    pub fn tribias(&mut self) -> TRIBIAS_W<GLBANA_SPEC> {
        TRIBIAS_W::new(self, 16)
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline(always)]
    #[must_use]
    pub fn ghren(&mut self) -> GHREN_W<GLBANA_SPEC> {
        GHREN_W::new(self, 18)
    }
}
#[doc = "Global Analog Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glbana::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glbana::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLBANA_SPEC;
impl crate::RegisterSpec for GLBANA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glbana::R`](R) reader structure"]
impl crate::Readable for GLBANA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`glbana::W`](W) writer structure"]
impl crate::Writable for GLBANA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLBANA to value 0x4b8c"]
impl crate::Resettable for GLBANA_SPEC {
    const RESET_VALUE: u32 = 0x4b8c;
}
