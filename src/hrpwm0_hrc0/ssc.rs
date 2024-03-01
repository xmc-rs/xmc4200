#[doc = "Register `SSC` reader"]
pub type R = crate::R<SscSpec>;
#[doc = "Register `SSC` writer"]
pub type W = crate::W<SscSpec>;
#[doc = "Source selector for the shadow transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sst {
    #[doc = "0: Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    Value1 = 0,
    #[doc = "1: Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    Value2 = 1,
}
impl From<Sst> for bool {
    #[inline(always)]
    fn from(variant: Sst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SST` reader - Source selector for the shadow transfer"]
pub type SstR = crate::BitReader<Sst>;
impl SstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sst {
        match self.bits {
            false => Sst::Value1,
            true => Sst::Value2,
        }
    }
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sst::Value1
    }
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sst::Value2
    }
}
#[doc = "Field `SST` writer - Source selector for the shadow transfer"]
pub type SstW<'a, REG> = crate::BitWriter<'a, REG, Sst>;
impl<'a, REG> SstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sst::Value1)
    }
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sst::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    pub fn sst(&self) -> SstR {
        SstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SstW<SscSpec> {
        SstW::new(self, 0)
    }
}
#[doc = "HRC next source for shadow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SscSpec;
impl crate::RegisterSpec for SscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssc::R`](R) reader structure"]
impl crate::Readable for SscSpec {}
#[doc = "`write(|w| ..)` method takes [`ssc::W`](W) writer structure"]
impl crate::Writable for SscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSC to value 0"]
impl crate::Resettable for SscSpec {
    const RESET_VALUE: u32 = 0;
}
