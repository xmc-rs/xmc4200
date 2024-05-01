#[doc = "Register `LPACTH0` reader"]
pub type R = crate::R<Lpacth0Spec>;
#[doc = "Register `LPACTH0` writer"]
pub type W = crate::W<Lpacth0Spec>;
#[doc = "Field `VBATLO` reader - VBAT Lower Threshold Value"]
pub type VbatloR = crate::FieldReader;
#[doc = "Field `VBATLO` writer - VBAT Lower Threshold Value"]
pub type VbatloW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VBATHI` reader - VBAT Upper Threshold Value"]
pub type VbathiR = crate::FieldReader;
#[doc = "Field `VBATHI` writer - VBAT Upper Threshold Value"]
pub type VbathiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - VBAT Lower Threshold Value"]
    #[inline(always)]
    pub fn vbatlo(&self) -> VbatloR {
        VbatloR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - VBAT Upper Threshold Value"]
    #[inline(always)]
    pub fn vbathi(&self) -> VbathiR {
        VbathiR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VBAT Lower Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn vbatlo(&mut self) -> VbatloW<Lpacth0Spec> {
        VbatloW::new(self, 0)
    }
    #[doc = "Bits 8:13 - VBAT Upper Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn vbathi(&mut self) -> VbathiW<Lpacth0Spec> {
        VbathiW::new(self, 8)
    }
}
#[doc = "LPAC Threshold Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacth0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacth0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpacth0Spec;
impl crate::RegisterSpec for Lpacth0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpacth0::R`](R) reader structure"]
impl crate::Readable for Lpacth0Spec {}
#[doc = "`write(|w| ..)` method takes [`lpacth0::W`](W) writer structure"]
impl crate::Writable for Lpacth0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPACTH0 to value 0"]
impl crate::Resettable for Lpacth0Spec {
    const RESET_VALUE: u32 = 0;
}
