#[doc = "Register `GLBANA` reader"]
pub type R = crate::R<GlbanaSpec>;
#[doc = "Register `GLBANA` writer"]
pub type W = crate::W<GlbanaSpec>;
#[doc = "Field `SLDLY` reader - Delay of lock detection"]
pub type SldlyR = crate::FieldReader;
#[doc = "Field `SLDLY` writer - Delay of lock detection"]
pub type SldlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUP` reader - Force chargepump up"]
pub type FupR = crate::BitReader;
#[doc = "Field `FUP` writer - Force chargepump up"]
pub type FupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDN` reader - Force chargepump down"]
pub type FdnR = crate::BitReader;
#[doc = "Field `FDN` writer - Force chargepump down"]
pub type FdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLCP` reader - HRCs chargepump current selection"]
pub type SlcpR = crate::FieldReader;
#[doc = "Field `SLCP` writer - HRCs chargepump current selection"]
pub type SlcpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SLIBLDO` reader - HRCs LDO bias current"]
pub type SlibldoR = crate::FieldReader;
#[doc = "Field `SLIBLDO` writer - HRCs LDO bias current"]
pub type SlibldoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLIBLF` reader - HRCs loop filter bias current"]
pub type SliblfR = crate::FieldReader;
#[doc = "Field `SLIBLF` writer - HRCs loop filter bias current"]
pub type SliblfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLVREF` reader - Reference voltage for chargepump and loop filter"]
pub type SlvrefR = crate::FieldReader;
#[doc = "Field `SLVREF` writer - Reference voltage for chargepump and loop filter"]
pub type SlvrefW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIBIAS` reader - Bias trimming"]
pub type TribiasR = crate::FieldReader;
#[doc = "Field `TRIBIAS` writer - Bias trimming"]
pub type TribiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Force chargepump down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ghren {
    #[doc = "0: Global high resolution generation is enabled"]
    Value1 = 0,
    #[doc = "1: Global high resolution generation is disabled"]
    Value2 = 1,
}
impl From<Ghren> for bool {
    #[inline(always)]
    fn from(variant: Ghren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GHREN` reader - Force chargepump down"]
pub type GhrenR = crate::BitReader<Ghren>;
impl GhrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ghren {
        match self.bits {
            false => Ghren::Value1,
            true => Ghren::Value2,
        }
    }
    #[doc = "Global high resolution generation is enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ghren::Value1
    }
    #[doc = "Global high resolution generation is disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ghren::Value2
    }
}
#[doc = "Field `GHREN` writer - Force chargepump down"]
pub type GhrenW<'a, REG> = crate::BitWriter<'a, REG, Ghren>;
impl<'a, REG> GhrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Global high resolution generation is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ghren::Value1)
    }
    #[doc = "Global high resolution generation is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ghren::Value2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline(always)]
    pub fn sldly(&self) -> SldlyR {
        SldlyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline(always)]
    pub fn fup(&self) -> FupR {
        FupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline(always)]
    pub fn fdn(&self) -> FdnR {
        FdnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline(always)]
    pub fn slcp(&self) -> SlcpR {
        SlcpR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline(always)]
    pub fn slibldo(&self) -> SlibldoR {
        SlibldoR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline(always)]
    pub fn sliblf(&self) -> SliblfR {
        SliblfR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline(always)]
    pub fn slvref(&self) -> SlvrefR {
        SlvrefR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline(always)]
    pub fn tribias(&self) -> TribiasR {
        TribiasR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline(always)]
    pub fn ghren(&self) -> GhrenR {
        GhrenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline(always)]
    #[must_use]
    pub fn sldly(&mut self) -> SldlyW<GlbanaSpec> {
        SldlyW::new(self, 0)
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline(always)]
    #[must_use]
    pub fn fup(&mut self) -> FupW<GlbanaSpec> {
        FupW::new(self, 2)
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline(always)]
    #[must_use]
    pub fn fdn(&mut self) -> FdnW<GlbanaSpec> {
        FdnW::new(self, 3)
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline(always)]
    #[must_use]
    pub fn slcp(&mut self) -> SlcpW<GlbanaSpec> {
        SlcpW::new(self, 6)
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline(always)]
    #[must_use]
    pub fn slibldo(&mut self) -> SlibldoW<GlbanaSpec> {
        SlibldoW::new(self, 9)
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline(always)]
    #[must_use]
    pub fn sliblf(&mut self) -> SliblfW<GlbanaSpec> {
        SliblfW::new(self, 11)
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline(always)]
    #[must_use]
    pub fn slvref(&mut self) -> SlvrefW<GlbanaSpec> {
        SlvrefW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline(always)]
    #[must_use]
    pub fn tribias(&mut self) -> TribiasW<GlbanaSpec> {
        TribiasW::new(self, 16)
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline(always)]
    #[must_use]
    pub fn ghren(&mut self) -> GhrenW<GlbanaSpec> {
        GhrenW::new(self, 18)
    }
}
#[doc = "Global Analog Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glbana::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glbana::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbanaSpec;
impl crate::RegisterSpec for GlbanaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glbana::R`](R) reader structure"]
impl crate::Readable for GlbanaSpec {}
#[doc = "`write(|w| ..)` method takes [`glbana::W`](W) writer structure"]
impl crate::Writable for GlbanaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLBANA to value 0x4b8c"]
impl crate::Resettable for GlbanaSpec {
    const RESET_VALUE: u32 = 0x4b8c;
}
