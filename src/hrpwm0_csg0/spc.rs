#[doc = "Register `SPC` reader"]
pub type R = crate::R<SpcSpec>;
#[doc = "Register `SPC` writer"]
pub type W = crate::W<SpcSpec>;
#[doc = "Field `SPSWV` reader - Shadow pulse swallow value"]
pub type SpswvR = crate::FieldReader;
#[doc = "Field `SPSWV` writer - Shadow pulse swallow value"]
pub type SpswvW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Shadow pulse swallow value"]
    #[inline(always)]
    pub fn spswv(&self) -> SpswvR {
        SpswvR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Shadow pulse swallow value"]
    #[inline(always)]
    #[must_use]
    pub fn spswv(&mut self) -> SpswvW<SpcSpec> {
        SpswvW::new(self, 0)
    }
}
#[doc = "Shadow Pulse swallow value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpcSpec;
impl crate::RegisterSpec for SpcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spc::R`](R) reader structure"]
impl crate::Readable for SpcSpec {}
#[doc = "`write(|w| ..)` method takes [`spc::W`](W) writer structure"]
impl crate::Writable for SpcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPC to value 0"]
impl crate::Resettable for SpcSpec {
    const RESET_VALUE: u32 = 0;
}
