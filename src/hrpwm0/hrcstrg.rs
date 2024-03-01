#[doc = "Register `HRCSTRG` writer"]
pub type W = crate::W<HrcstrgSpec>;
#[doc = "Field `H0ES` writer - HRC0 high resolution values shadow transfer Enable Set"]
pub type H0esW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H0DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H0desW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H1ES` writer - HRC1 high resolution values shadow transfer Enable Set"]
pub type H1esW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H1DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H1desW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2ES` writer - HRC2 high resolution values shadow transfer Enable Set"]
pub type H2esW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H2desW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H3ES` writer - HRC3 high resolution values shadow transfer Enable Set"]
pub type H3esW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H3DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H3desW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h0es(&mut self) -> H0esW<HrcstrgSpec> {
        H0esW::new(self, 0)
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h0des(&mut self) -> H0desW<HrcstrgSpec> {
        H0desW::new(self, 1)
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h1es(&mut self) -> H1esW<HrcstrgSpec> {
        H1esW::new(self, 4)
    }
    #[doc = "Bit 5 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h1des(&mut self) -> H1desW<HrcstrgSpec> {
        H1desW::new(self, 5)
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h2es(&mut self) -> H2esW<HrcstrgSpec> {
        H2esW::new(self, 8)
    }
    #[doc = "Bit 9 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h2des(&mut self) -> H2desW<HrcstrgSpec> {
        H2desW::new(self, 9)
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h3es(&mut self) -> H3esW<HrcstrgSpec> {
        H3esW::new(self, 12)
    }
    #[doc = "Bit 13 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h3des(&mut self) -> H3desW<HrcstrgSpec> {
        H3desW::new(self, 13)
    }
}
#[doc = "Global HRC shadow trigger set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrcstrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrcstrgSpec;
impl crate::RegisterSpec for HrcstrgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hrcstrg::W`](W) writer structure"]
impl crate::Writable for HrcstrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRCSTRG to value 0"]
impl crate::Resettable for HrcstrgSpec {
    const RESET_VALUE: u32 = 0;
}
