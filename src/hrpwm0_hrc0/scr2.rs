#[doc = "Register `SCR2` reader"]
pub type R = crate::R<Scr2Spec>;
#[doc = "Register `SCR2` writer"]
pub type W = crate::W<Scr2Spec>;
#[doc = "Field `SCR2` reader - High resolution rising edge value"]
pub type Scr2R = crate::FieldReader;
#[doc = "Field `SCR2` writer - High resolution rising edge value"]
pub type Scr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn scr2(&self) -> Scr2R {
        Scr2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    #[must_use]
    pub fn scr2(&mut self) -> Scr2W<Scr2Spec> {
        Scr2W::new(self, 0)
    }
}
#[doc = "HRC shadow falling edge value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr2Spec;
impl crate::RegisterSpec for Scr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr2::R`](R) reader structure"]
impl crate::Readable for Scr2Spec {}
#[doc = "`write(|w| ..)` method takes [`scr2::W`](W) writer structure"]
impl crate::Writable for Scr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR2 to value 0"]
impl crate::Resettable for Scr2Spec {
    const RESET_VALUE: u32 = 0;
}
