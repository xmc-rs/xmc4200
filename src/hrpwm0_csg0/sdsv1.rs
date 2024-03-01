#[doc = "Register `SDSV1` reader"]
pub type R = crate::R<Sdsv1Spec>;
#[doc = "Register `SDSV1` writer"]
pub type W = crate::W<Sdsv1Spec>;
#[doc = "Field `SDSV1` reader - Shadow DAC reference value 1"]
pub type Sdsv1R = crate::FieldReader<u16>;
#[doc = "Field `SDSV1` writer - Shadow DAC reference value 1"]
pub type Sdsv1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Shadow DAC reference value 1"]
    #[inline(always)]
    pub fn sdsv1(&self) -> Sdsv1R {
        Sdsv1R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Shadow DAC reference value 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdsv1(&mut self) -> Sdsv1W<Sdsv1Spec> {
        Sdsv1W::new(self, 0)
    }
}
#[doc = "Shadow reference value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdsv1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdsv1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdsv1Spec;
impl crate::RegisterSpec for Sdsv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdsv1::R`](R) reader structure"]
impl crate::Readable for Sdsv1Spec {}
#[doc = "`write(|w| ..)` method takes [`sdsv1::W`](W) writer structure"]
impl crate::Writable for Sdsv1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDSV1 to value 0"]
impl crate::Resettable for Sdsv1Spec {
    const RESET_VALUE: u32 = 0;
}
