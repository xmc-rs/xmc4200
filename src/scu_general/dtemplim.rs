#[doc = "Register `DTEMPLIM` reader"]
pub type R = crate::R<DtemplimSpec>;
#[doc = "Register `DTEMPLIM` writer"]
pub type W = crate::W<DtemplimSpec>;
#[doc = "Field `LOWER` reader - Lower Limit"]
pub type LowerR = crate::FieldReader<u16>;
#[doc = "Field `LOWER` writer - Lower Limit"]
pub type LowerW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `UPPER` reader - Upper Limit"]
pub type UpperR = crate::FieldReader<u16>;
#[doc = "Field `UPPER` writer - Upper Limit"]
pub type UpperW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Lower Limit"]
    #[inline(always)]
    pub fn lower(&self) -> LowerR {
        LowerR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Upper Limit"]
    #[inline(always)]
    pub fn upper(&self) -> UpperR {
        UpperR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lower Limit"]
    #[inline(always)]
    #[must_use]
    pub fn lower(&mut self) -> LowerW<DtemplimSpec> {
        LowerW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Upper Limit"]
    #[inline(always)]
    #[must_use]
    pub fn upper(&mut self) -> UpperW<DtemplimSpec> {
        UpperW::new(self, 16)
    }
}
#[doc = "Die Temperature Sensor Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtemplim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtemplim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtemplimSpec;
impl crate::RegisterSpec for DtemplimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtemplim::R`](R) reader structure"]
impl crate::Readable for DtemplimSpec {}
#[doc = "`write(|w| ..)` method takes [`dtemplim::W`](W) writer structure"]
impl crate::Writable for DtemplimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTEMPLIM to value 0"]
impl crate::Resettable for DtemplimSpec {
    const RESET_VALUE: u32 = 0;
}
