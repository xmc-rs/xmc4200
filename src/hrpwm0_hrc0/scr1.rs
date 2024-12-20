#[doc = "Register `SCR1` reader"]
pub type R = crate::R<SCR1_SPEC>;
#[doc = "Register `SCR1` writer"]
pub type W = crate::W<SCR1_SPEC>;
#[doc = "Field `SCR1` reader - High resolution falling edge value"]
pub type SCR1_R = crate::FieldReader;
#[doc = "Field `SCR1` writer - High resolution falling edge value"]
pub type SCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    pub fn scr1(&self) -> SCR1_R {
        SCR1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    pub fn scr1(&mut self) -> SCR1_W<SCR1_SPEC> {
        SCR1_W::new(self, 0)
    }
}
#[doc = "HRC shadow rising edge value\n\nYou can [`read`](crate::Reg::read) this register and get [`scr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR1_SPEC;
impl crate::RegisterSpec for SCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr1::R`](R) reader structure"]
impl crate::Readable for SCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr1::W`](W) writer structure"]
impl crate::Writable for SCR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR1 to value 0"]
impl crate::Resettable for SCR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
