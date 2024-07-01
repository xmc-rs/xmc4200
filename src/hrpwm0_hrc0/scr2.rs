#[doc = "Register `SCR2` reader"]
pub type R = crate::R<SCR2_SPEC>;
#[doc = "Register `SCR2` writer"]
pub type W = crate::W<SCR2_SPEC>;
#[doc = "Field `SCR2` reader - High resolution rising edge value"]
pub type SCR2_R = crate::FieldReader;
#[doc = "Field `SCR2` writer - High resolution rising edge value"]
pub type SCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn scr2(&self) -> SCR2_R {
        SCR2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    #[must_use]
    pub fn scr2(&mut self) -> SCR2_W<SCR2_SPEC> {
        SCR2_W::new(self, 0)
    }
}
#[doc = "HRC shadow falling edge value\n\nYou can [`read`](crate::Reg::read) this register and get [`scr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR2_SPEC;
impl crate::RegisterSpec for SCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr2::R`](R) reader structure"]
impl crate::Readable for SCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr2::W`](W) writer structure"]
impl crate::Writable for SCR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR2 to value 0"]
impl crate::Resettable for SCR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
