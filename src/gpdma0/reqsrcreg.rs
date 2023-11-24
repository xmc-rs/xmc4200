#[doc = "Register `REQSRCREG` reader"]
pub type R = crate::R<REQSRCREG_SPEC>;
#[doc = "Register `REQSRCREG` writer"]
pub type W = crate::W<REQSRCREG_SPEC>;
#[doc = "Field `CH0` reader - Source request for channel 0"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - Source request for channel 0"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Source request for channel 1"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - Source request for channel 1"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Source request for channel 2"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - Source request for channel 2"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Source request for channel 3"]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH3` writer - Source request for channel 3"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` reader - Source request for channel 4"]
pub type CH4_R = crate::BitReader;
#[doc = "Field `CH4` writer - Source request for channel 4"]
pub type CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` reader - Source request for channel 5"]
pub type CH5_R = crate::BitReader;
#[doc = "Field `CH5` writer - Source request for channel 5"]
pub type CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` reader - Source request for channel 6"]
pub type CH6_R = crate::BitReader;
#[doc = "Field `CH6` writer - Source request for channel 6"]
pub type CH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` reader - Source request for channel 7"]
pub type CH7_R = crate::BitReader;
#[doc = "Field `CH7` writer - Source request for channel 7"]
pub type CH7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Source request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH0_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Source request write enable for channel 0"]
pub type WE_CH0_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH0_AW>;
impl<'a, REG> WE_CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH0_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH0_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH1_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Source request write enable for channel 1"]
pub type WE_CH1_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH1_AW>;
impl<'a, REG> WE_CH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH1_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH1_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH2_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Source request write enable for channel 2"]
pub type WE_CH2_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH2_AW>;
impl<'a, REG> WE_CH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH2_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH2_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH3_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Source request write enable for channel 3"]
pub type WE_CH3_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH3_AW>;
impl<'a, REG> WE_CH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH3_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH3_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH4_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH4_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH4` writer - Source request write enable for channel 4"]
pub type WE_CH4_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH4_AW>;
impl<'a, REG> WE_CH4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH4_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH4_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH5_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH5_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH5` writer - Source request write enable for channel 5"]
pub type WE_CH5_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH5_AW>;
impl<'a, REG> WE_CH5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH5_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH5_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH6_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH6_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH6` writer - Source request write enable for channel 6"]
pub type WE_CH6_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH6_AW>;
impl<'a, REG> WE_CH6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH6_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH6_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH7_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH7_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH7` writer - Source request write enable for channel 7"]
pub type WE_CH7_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH7_AW>;
impl<'a, REG> WE_CH7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH7_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH7_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source request for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Source request for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source request for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source request for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<REQSRCREG_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<REQSRCREG_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<REQSRCREG_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<REQSRCREG_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Source request for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<REQSRCREG_SPEC> {
        CH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Source request for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<REQSRCREG_SPEC> {
        CH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Source request for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<REQSRCREG_SPEC> {
        CH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Source request for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<REQSRCREG_SPEC> {
        CH7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Source request write enable for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch0(&mut self) -> WE_CH0_W<REQSRCREG_SPEC> {
        WE_CH0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Source request write enable for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch1(&mut self) -> WE_CH1_W<REQSRCREG_SPEC> {
        WE_CH1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Source request write enable for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch2(&mut self) -> WE_CH2_W<REQSRCREG_SPEC> {
        WE_CH2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Source request write enable for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch3(&mut self) -> WE_CH3_W<REQSRCREG_SPEC> {
        WE_CH3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Source request write enable for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch4(&mut self) -> WE_CH4_W<REQSRCREG_SPEC> {
        WE_CH4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Source request write enable for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch5(&mut self) -> WE_CH5_W<REQSRCREG_SPEC> {
        WE_CH5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Source request write enable for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch6(&mut self) -> WE_CH6_W<REQSRCREG_SPEC> {
        WE_CH6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Source request write enable for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch7(&mut self) -> WE_CH7_W<REQSRCREG_SPEC> {
        WE_CH7_W::new(self, 15)
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
#[doc = "Source Software Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqsrcreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqsrcreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REQSRCREG_SPEC;
impl crate::RegisterSpec for REQSRCREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqsrcreg::R`](R) reader structure"]
impl crate::Readable for REQSRCREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reqsrcreg::W`](W) writer structure"]
impl crate::Writable for REQSRCREG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REQSRCREG to value 0"]
impl crate::Resettable for REQSRCREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
