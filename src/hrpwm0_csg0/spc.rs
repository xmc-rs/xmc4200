#[doc = "Register `SPC` reader"]
pub type R = crate::R<SPC_SPEC>;
#[doc = "Register `SPC` writer"]
pub type W = crate::W<SPC_SPEC>;
#[doc = "Field `SPSWV` reader - Shadow pulse swallow value"]
pub type SPSWV_R = crate::FieldReader;
#[doc = "Field `SPSWV` writer - Shadow pulse swallow value"]
pub type SPSWV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Shadow pulse swallow value"]
    #[inline(always)]
    pub fn spswv(&self) -> SPSWV_R {
        SPSWV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Shadow pulse swallow value"]
    #[inline(always)]
    #[must_use]
    pub fn spswv(&mut self) -> SPSWV_W<SPC_SPEC> {
        SPSWV_W::new(self, 0)
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
#[doc = "Shadow Pulse swallow value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPC_SPEC;
impl crate::RegisterSpec for SPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spc::R`](R) reader structure"]
impl crate::Readable for SPC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spc::W`](W) writer structure"]
impl crate::Writable for SPC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPC to value 0"]
impl crate::Resettable for SPC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
