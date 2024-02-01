#[doc = "Register `SSC` reader"]
pub type R = crate::R<SSC_SPEC>;
#[doc = "Register `SSC` writer"]
pub type W = crate::W<SSC_SPEC>;
#[doc = "Field `SST` reader - Source selector for the shadow transfer"]
pub type SST_R = crate::BitReader<SST_A>;
#[doc = "Source selector for the shadow transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SST_A {
    #[doc = "0: Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    VALUE1 = 0,
    #[doc = "1: Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    VALUE2 = 1,
}
impl From<SST_A> for bool {
    #[inline(always)]
    fn from(variant: SST_A) -> Self {
        variant as u8 != 0
    }
}
impl SST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SST_A {
        match self.bits {
            false => SST_A::VALUE1,
            true => SST_A::VALUE2,
        }
    }
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SST_A::VALUE1
    }
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SST_A::VALUE2
    }
}
#[doc = "Field `SST` writer - Source selector for the shadow transfer"]
pub type SST_W<'a, REG> = crate::BitWriter<'a, REG, SST_A>;
impl<'a, REG> SST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SST_A::VALUE1)
    }
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SST_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SST_W<SSC_SPEC> {
        SST_W::new(self, 0)
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
#[doc = "HRC next source for shadow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSC_SPEC;
impl crate::RegisterSpec for SSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssc::R`](R) reader structure"]
impl crate::Readable for SSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssc::W`](W) writer structure"]
impl crate::Writable for SSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSC to value 0"]
impl crate::Resettable for SSC_SPEC {
    const RESET_VALUE: u32 = 0;
}
