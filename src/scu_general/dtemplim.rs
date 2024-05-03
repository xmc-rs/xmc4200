#[doc = "Register `DTEMPLIM` reader"]
pub type R = crate::R<DTEMPLIM_SPEC>;
#[doc = "Register `DTEMPLIM` writer"]
pub type W = crate::W<DTEMPLIM_SPEC>;
#[doc = "Field `LOWER` reader - Lower Limit"]
pub type LOWER_R = crate::FieldReader<u16>;
#[doc = "Field `LOWER` writer - Lower Limit"]
pub type LOWER_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `UPPER` reader - Upper Limit"]
pub type UPPER_R = crate::FieldReader<u16>;
#[doc = "Field `UPPER` writer - Upper Limit"]
pub type UPPER_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Lower Limit"]
    #[inline(always)]
    pub fn lower(&self) -> LOWER_R {
        LOWER_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Upper Limit"]
    #[inline(always)]
    pub fn upper(&self) -> UPPER_R {
        UPPER_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lower Limit"]
    #[inline(always)]
    #[must_use]
    pub fn lower(&mut self) -> LOWER_W<DTEMPLIM_SPEC> {
        LOWER_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Upper Limit"]
    #[inline(always)]
    #[must_use]
    pub fn upper(&mut self) -> UPPER_W<DTEMPLIM_SPEC> {
        UPPER_W::new(self, 16)
    }
}
#[doc = "Die Temperature Sensor Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtemplim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtemplim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTEMPLIM_SPEC;
impl crate::RegisterSpec for DTEMPLIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtemplim::R`](R) reader structure"]
impl crate::Readable for DTEMPLIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtemplim::W`](W) writer structure"]
impl crate::Writable for DTEMPLIM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTEMPLIM to value 0"]
impl crate::Resettable for DTEMPLIM_SPEC {
    const RESET_VALUE: u32 = 0;
}
