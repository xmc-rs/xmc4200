#[doc = "Register `DSV2` reader"]
pub type R = crate::R<Dsv2Spec>;
#[doc = "Register `DSV2` writer"]
pub type W = crate::W<Dsv2Spec>;
#[doc = "Field `DSV2` reader - DAC reference value 2"]
pub type Dsv2R = crate::FieldReader<u16>;
#[doc = "Field `DSV2` writer - DAC reference value 2"]
pub type Dsv2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC reference value 2"]
    #[inline(always)]
    pub fn dsv2(&self) -> Dsv2R {
        Dsv2R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC reference value 2"]
    #[inline(always)]
    #[must_use]
    pub fn dsv2(&mut self) -> Dsv2W<Dsv2Spec> {
        Dsv2W::new(self, 0)
    }
}
#[doc = "DAC reference value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsv2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsv2Spec;
impl crate::RegisterSpec for Dsv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsv2::R`](R) reader structure"]
impl crate::Readable for Dsv2Spec {}
#[doc = "`write(|w| ..)` method takes [`dsv2::W`](W) writer structure"]
impl crate::Writable for Dsv2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSV2 to value 0"]
impl crate::Resettable for Dsv2Spec {
    const RESET_VALUE: u32 = 0;
}
