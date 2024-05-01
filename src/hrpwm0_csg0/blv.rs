#[doc = "Register `BLV` reader"]
pub type R = crate::R<BlvSpec>;
#[doc = "Register `BLV` writer"]
pub type W = crate::W<BlvSpec>;
#[doc = "Field `BLV` reader - Blanking value"]
pub type BlvR = crate::FieldReader;
#[doc = "Field `BLV` writer - Blanking value"]
pub type BlvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Blanking value"]
    #[inline(always)]
    pub fn blv(&self) -> BlvR {
        BlvR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blanking value"]
    #[inline(always)]
    #[must_use]
    pub fn blv(&mut self) -> BlvW<BlvSpec> {
        BlvW::new(self, 0)
    }
}
#[doc = "Comparator blanking value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlvSpec;
impl crate::RegisterSpec for BlvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blv::R`](R) reader structure"]
impl crate::Readable for BlvSpec {}
#[doc = "`write(|w| ..)` method takes [`blv::W`](W) writer structure"]
impl crate::Writable for BlvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLV to value 0"]
impl crate::Resettable for BlvSpec {
    const RESET_VALUE: u32 = 0;
}
