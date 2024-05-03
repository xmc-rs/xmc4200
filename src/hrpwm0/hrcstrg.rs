#[doc = "Register `HRCSTRG` writer"]
pub type W = crate::W<HRCSTRG_SPEC>;
#[doc = "Field `H0ES` writer - HRC0 high resolution values shadow transfer Enable Set"]
pub type H0ES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H0DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H0DES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H1ES` writer - HRC1 high resolution values shadow transfer Enable Set"]
pub type H1ES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H1DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H1DES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2ES` writer - HRC2 high resolution values shadow transfer Enable Set"]
pub type H2ES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H2DES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H3ES` writer - HRC3 high resolution values shadow transfer Enable Set"]
pub type H3ES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H3DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H3DES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h0es(&mut self) -> H0ES_W<HRCSTRG_SPEC> {
        H0ES_W::new(self, 0)
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h0des(&mut self) -> H0DES_W<HRCSTRG_SPEC> {
        H0DES_W::new(self, 1)
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h1es(&mut self) -> H1ES_W<HRCSTRG_SPEC> {
        H1ES_W::new(self, 4)
    }
    #[doc = "Bit 5 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h1des(&mut self) -> H1DES_W<HRCSTRG_SPEC> {
        H1DES_W::new(self, 5)
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h2es(&mut self) -> H2ES_W<HRCSTRG_SPEC> {
        H2ES_W::new(self, 8)
    }
    #[doc = "Bit 9 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h2des(&mut self) -> H2DES_W<HRCSTRG_SPEC> {
        H2DES_W::new(self, 9)
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h3es(&mut self) -> H3ES_W<HRCSTRG_SPEC> {
        H3ES_W::new(self, 12)
    }
    #[doc = "Bit 13 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h3des(&mut self) -> H3DES_W<HRCSTRG_SPEC> {
        H3DES_W::new(self, 13)
    }
}
#[doc = "Global HRC shadow trigger set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrcstrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRCSTRG_SPEC;
impl crate::RegisterSpec for HRCSTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hrcstrg::W`](W) writer structure"]
impl crate::Writable for HRCSTRG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRCSTRG to value 0"]
impl crate::Resettable for HRCSTRG_SPEC {
    const RESET_VALUE: u32 = 0;
}
