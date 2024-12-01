#[doc = "Register `HRCCTRG` writer"]
pub type W = crate::W<HRCCTRG_SPEC>;
#[doc = "Field `H0EC` writer - HRC0 high resolution values shadow transfer Enable Clear"]
pub type H0EC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H0DEC` writer - HRC0 dead time value shadow transfer Enable Clear"]
pub type H0DEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H1EC` writer - HRC1 high resolution values shadow transfer Enable Clear"]
pub type H1EC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H1DEC` writer - HRC1 dead time value shadow transfer Enable Clear"]
pub type H1DEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2CEC` writer - HRC2 high resolution values shadow transfer Enable Clear"]
pub type H2CEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2DEC` writer - HRC2 dead time value shadow transfer Enable Clear"]
pub type H2DEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H3EC` writer - HRC3 high resolution values shadow transfer Enable Clear"]
pub type H3EC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H3DEC` writer - HRC3 dead time value shadow transfer Enable Clear"]
pub type H3DEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h0ec(&mut self) -> H0EC_W<HRCCTRG_SPEC> {
        H0EC_W::new(self, 0)
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h0dec(&mut self) -> H0DEC_W<HRCCTRG_SPEC> {
        H0DEC_W::new(self, 1)
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h1ec(&mut self) -> H1EC_W<HRCCTRG_SPEC> {
        H1EC_W::new(self, 4)
    }
    #[doc = "Bit 5 - HRC1 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h1dec(&mut self) -> H1DEC_W<HRCCTRG_SPEC> {
        H1DEC_W::new(self, 5)
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h2cec(&mut self) -> H2CEC_W<HRCCTRG_SPEC> {
        H2CEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - HRC2 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h2dec(&mut self) -> H2DEC_W<HRCCTRG_SPEC> {
        H2DEC_W::new(self, 9)
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h3ec(&mut self) -> H3EC_W<HRCCTRG_SPEC> {
        H3EC_W::new(self, 12)
    }
    #[doc = "Bit 13 - HRC3 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h3dec(&mut self) -> H3DEC_W<HRCCTRG_SPEC> {
        H3DEC_W::new(self, 13)
    }
}
#[doc = "Global HRC shadow trigger clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrcctrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRCCTRG_SPEC;
impl crate::RegisterSpec for HRCCTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hrcctrg::W`](W) writer structure"]
impl crate::Writable for HRCCTRG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRCCTRG to value 0"]
impl crate::Resettable for HRCCTRG_SPEC {
    const RESET_VALUE: u32 = 0;
}
