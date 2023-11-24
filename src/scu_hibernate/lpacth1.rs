#[doc = "Register `LPACTH1` reader"]
pub type R = crate::R<LPACTH1_SPEC>;
#[doc = "Register `LPACTH1` writer"]
pub type W = crate::W<LPACTH1_SPEC>;
#[doc = "Field `AHIBIO0LO` reader - Analog HIB_IO_0 Lower Threshold Value"]
pub type AHIBIO0LO_R = crate::FieldReader;
#[doc = "Field `AHIBIO0LO` writer - Analog HIB_IO_0 Lower Threshold Value"]
pub type AHIBIO0LO_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AHIBIO0HI` reader - Analog HIB_IO_0 Upper Threshold Value"]
pub type AHIBIO0HI_R = crate::FieldReader;
#[doc = "Field `AHIBIO0HI` writer - Analog HIB_IO_0 Upper Threshold Value"]
pub type AHIBIO0HI_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio0lo(&self) -> AHIBIO0LO_R {
        AHIBIO0LO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio0hi(&self) -> AHIBIO0HI_R {
        AHIBIO0HI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0lo(&mut self) -> AHIBIO0LO_W<LPACTH1_SPEC> {
        AHIBIO0LO_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0hi(&mut self) -> AHIBIO0HI_W<LPACTH1_SPEC> {
        AHIBIO0HI_W::new(self, 8)
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
#[doc = "LPAC Threshold Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacth1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacth1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPACTH1_SPEC;
impl crate::RegisterSpec for LPACTH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpacth1::R`](R) reader structure"]
impl crate::Readable for LPACTH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpacth1::W`](W) writer structure"]
impl crate::Writable for LPACTH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPACTH1 to value 0"]
impl crate::Resettable for LPACTH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
