#[doc = "Register `SCR1` reader"]
pub type R = crate::R<Scr1Spec>;
#[doc = "Register `SCR1` writer"]
pub type W = crate::W<Scr1Spec>;
#[doc = "Field `SCR1` reader - High resolution falling edge value"]
pub type Scr1R = crate::FieldReader;
#[doc = "Field `SCR1` writer - High resolution falling edge value"]
pub type Scr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    pub fn scr1(&self) -> Scr1R {
        Scr1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    #[must_use]
    pub fn scr1(&mut self) -> Scr1W<Scr1Spec> {
        Scr1W::new(self, 0)
    }
}
#[doc = "HRC shadow rising edge value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr1Spec;
impl crate::RegisterSpec for Scr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr1::R`](R) reader structure"]
impl crate::Readable for Scr1Spec {}
#[doc = "`write(|w| ..)` method takes [`scr1::W`](W) writer structure"]
impl crate::Writable for Scr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR1 to value 0"]
impl crate::Resettable for Scr1Spec {
    const RESET_VALUE: u32 = 0;
}
