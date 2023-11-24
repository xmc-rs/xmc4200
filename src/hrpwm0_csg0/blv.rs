#[doc = "Register `BLV` reader"]
pub type R = crate::R<BLV_SPEC>;
#[doc = "Register `BLV` writer"]
pub type W = crate::W<BLV_SPEC>;
#[doc = "Field `BLV` reader - Blanking value"]
pub type BLV_R = crate::FieldReader;
#[doc = "Field `BLV` writer - Blanking value"]
pub type BLV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Blanking value"]
    #[inline(always)]
    pub fn blv(&self) -> BLV_R {
        BLV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blanking value"]
    #[inline(always)]
    #[must_use]
    pub fn blv(&mut self) -> BLV_W<BLV_SPEC> {
        BLV_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Comparator blanking value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLV_SPEC;
impl crate::RegisterSpec for BLV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blv::R`](R) reader structure"]
impl crate::Readable for BLV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blv::W`](W) writer structure"]
impl crate::Writable for BLV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLV to value 0"]
impl crate::Resettable for BLV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
