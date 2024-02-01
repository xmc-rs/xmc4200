#[doc = "Register `LPACTH0` reader"]
pub type R = crate::R<LPACTH0_SPEC>;
#[doc = "Register `LPACTH0` writer"]
pub type W = crate::W<LPACTH0_SPEC>;
#[doc = "Field `VBATLO` reader - VBAT Lower Threshold Value"]
pub type VBATLO_R = crate::FieldReader;
#[doc = "Field `VBATLO` writer - VBAT Lower Threshold Value"]
pub type VBATLO_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VBATHI` reader - VBAT Upper Threshold Value"]
pub type VBATHI_R = crate::FieldReader;
#[doc = "Field `VBATHI` writer - VBAT Upper Threshold Value"]
pub type VBATHI_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - VBAT Lower Threshold Value"]
    #[inline(always)]
    pub fn vbatlo(&self) -> VBATLO_R {
        VBATLO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - VBAT Upper Threshold Value"]
    #[inline(always)]
    pub fn vbathi(&self) -> VBATHI_R {
        VBATHI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VBAT Lower Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn vbatlo(&mut self) -> VBATLO_W<LPACTH0_SPEC> {
        VBATLO_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - VBAT Upper Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn vbathi(&mut self) -> VBATHI_W<LPACTH0_SPEC> {
        VBATHI_W::new(self, 8)
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
#[doc = "LPAC Threshold Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacth0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacth0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPACTH0_SPEC;
impl crate::RegisterSpec for LPACTH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpacth0::R`](R) reader structure"]
impl crate::Readable for LPACTH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpacth0::W`](W) writer structure"]
impl crate::Writable for LPACTH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPACTH0 to value 0"]
impl crate::Resettable for LPACTH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
