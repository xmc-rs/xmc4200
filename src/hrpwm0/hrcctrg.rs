#[doc = "Register `HRCCTRG` writer"]
pub type W = crate::W<HrcctrgSpec>;
#[doc = "Field `H0EC` writer - HRC0 high resolution values shadow transfer Enable Clear"]
pub type H0ecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H0DEC` writer - HRC0 dead time value shadow transfer Enable Clear"]
pub type H0decW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H1EC` writer - HRC1 high resolution values shadow transfer Enable Clear"]
pub type H1ecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H1DEC` writer - HRC1 dead time value shadow transfer Enable Clear"]
pub type H1decW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2CEC` writer - HRC2 high resolution values shadow transfer Enable Clear"]
pub type H2cecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2DEC` writer - HRC2 dead time value shadow transfer Enable Clear"]
pub type H2decW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H3EC` writer - HRC3 high resolution values shadow transfer Enable Clear"]
pub type H3ecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H3DEC` writer - HRC3 dead time value shadow transfer Enable Clear"]
pub type H3decW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h0ec(&mut self) -> H0ecW<HrcctrgSpec> {
        H0ecW::new(self, 0)
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h0dec(&mut self) -> H0decW<HrcctrgSpec> {
        H0decW::new(self, 1)
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h1ec(&mut self) -> H1ecW<HrcctrgSpec> {
        H1ecW::new(self, 4)
    }
    #[doc = "Bit 5 - HRC1 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h1dec(&mut self) -> H1decW<HrcctrgSpec> {
        H1decW::new(self, 5)
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h2cec(&mut self) -> H2cecW<HrcctrgSpec> {
        H2cecW::new(self, 8)
    }
    #[doc = "Bit 9 - HRC2 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h2dec(&mut self) -> H2decW<HrcctrgSpec> {
        H2decW::new(self, 9)
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h3ec(&mut self) -> H3ecW<HrcctrgSpec> {
        H3ecW::new(self, 12)
    }
    #[doc = "Bit 13 - HRC3 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h3dec(&mut self) -> H3decW<HrcctrgSpec> {
        H3decW::new(self, 13)
    }
}
#[doc = "Global HRC shadow trigger clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrcctrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrcctrgSpec;
impl crate::RegisterSpec for HrcctrgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hrcctrg::W`](W) writer structure"]
impl crate::Writable for HrcctrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRCCTRG to value 0"]
impl crate::Resettable for HrcctrgSpec {
    const RESET_VALUE: u32 = 0;
}
