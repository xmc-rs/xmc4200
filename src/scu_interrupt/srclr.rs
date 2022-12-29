#[doc = "Register `SRCLR` writer"]
pub struct W(crate::W<SRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDT pre-warning Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRWARN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<PRWARN_AW> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Clear"]
pub type PRWARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, PRWARN_AW, O>;
impl<'a, const O: u8> PRWARN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRWARN_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRWARN_AW::VALUE2)
    }
}
#[doc = "RTC Periodic Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<PI_AW> for bool {
    #[inline(always)]
    fn from(variant: PI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Clear"]
pub type PI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, PI_AW, O>;
impl<'a, const O: u8> PI_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PI_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PI_AW::VALUE2)
    }
}
#[doc = "RTC Alarm Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<AI_AW> for bool {
    #[inline(always)]
    fn from(variant: AI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Clear"]
pub type AI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, AI_AW, O>;
impl<'a, const O: u8> AI_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AI_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AI_AW::VALUE2)
    }
}
#[doc = "DLR Request Overrun Interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLROVR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<DLROVR_AW> for bool {
    #[inline(always)]
    fn from(variant: DLROVR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt clear"]
pub type DLROVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, DLROVR_AW, O>;
impl<'a, const O: u8> DLROVR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLROVR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLROVR_AW::VALUE2)
    }
}
#[doc = "LPACLR Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACCR_AW> for bool {
    #[inline(always)]
    fn from(variant: LPACCR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCR` writer - LPACLR Mirror Register Update Interrupt Clear"]
pub type LPACCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, LPACCR_AW, O>;
impl<'a, const O: u8> LPACCR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACCR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACCR_AW::VALUE2)
    }
}
#[doc = "LPACTH0 Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACTH0_AW> for bool {
    #[inline(always)]
    fn from(variant: LPACTH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH0` writer - LPACTH0 Mirror Register Update Interrupt Clear"]
pub type LPACTH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, LPACTH0_AW, O>;
impl<'a, const O: u8> LPACTH0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACTH0_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACTH0_AW::VALUE2)
    }
}
#[doc = "LPACTH1 Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACTH1_AW> for bool {
    #[inline(always)]
    fn from(variant: LPACTH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH1` writer - LPACTH1 Mirror Register Update Interrupt Clear"]
pub type LPACTH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, LPACTH1_AW, O>;
impl<'a, const O: u8> LPACTH1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACTH1_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACTH1_AW::VALUE2)
    }
}
#[doc = "LPACST Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACST_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACST_AW> for bool {
    #[inline(always)]
    fn from(variant: LPACST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACST` writer - LPACST Mirror Register Update Interrupt Clear"]
pub type LPACST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, LPACST_AW, O>;
impl<'a, const O: u8> LPACST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACST_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACST_AW::VALUE2)
    }
}
#[doc = "LPACCLR Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCLR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: LPACCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCLR` writer - LPACCLR Mirror Register Update Interrupt Clear"]
pub type LPACCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, LPACCLR_AW, O>;
impl<'a, const O: u8> LPACCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACCLR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACCLR_AW::VALUE2)
    }
}
#[doc = "LPACSET Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACSET_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACSET_AW> for bool {
    #[inline(always)]
    fn from(variant: LPACSET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACSET` writer - LPACSET Mirror Register Update Interrupt Clear"]
pub type LPACSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, LPACSET_AW, O>;
impl<'a, const O: u8> LPACSET_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACSET_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACSET_AW::VALUE2)
    }
}
#[doc = "HINTST Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTST_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HINTST_AW> for bool {
    #[inline(always)]
    fn from(variant: HINTST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTST` writer - HINTST Mirror Register Update Interrupt Clear"]
pub type HINTST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, HINTST_AW, O>;
impl<'a, const O: u8> HINTST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HINTST_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HINTST_AW::VALUE2)
    }
}
#[doc = "HINTCLR Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTCLR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HINTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: HINTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTCLR` writer - HINTCLR Mirror Register Update Interrupt Clear"]
pub type HINTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, HINTCLR_AW, O>;
impl<'a, const O: u8> HINTCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HINTCLR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HINTCLR_AW::VALUE2)
    }
}
#[doc = "HINTSET Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTSET_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HINTSET_AW> for bool {
    #[inline(always)]
    fn from(variant: HINTSET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTSET` writer - HINTSET Mirror Register Update Interrupt Clear"]
pub type HINTSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, HINTSET_AW, O>;
impl<'a, const O: u8> HINTSET_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HINTSET_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HINTSET_AW::VALUE2)
    }
}
#[doc = "HDCLR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCLR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HDCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCLR` writer - HDCLR Mirror Register Update Clear"]
pub type HDCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, HDCLR_AW, O>;
impl<'a, const O: u8> HDCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDCLR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDCLR_AW::VALUE2)
    }
}
#[doc = "HDSET Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSET_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HDSET_AW> for bool {
    #[inline(always)]
    fn from(variant: HDSET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSET` writer - HDSET Mirror Register Update Clear"]
pub type HDSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, HDSET_AW, O>;
impl<'a, const O: u8> HDSET_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDSET_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDSET_AW::VALUE2)
    }
}
#[doc = "HDCR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HDCR_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Clear"]
pub type HDCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, HDCR_AW, O>;
impl<'a, const O: u8> HDCR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDCR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDCR_AW::VALUE2)
    }
}
#[doc = "OSCSICTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSICTRL_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<OSCSICTRL_AW> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Clear"]
pub type OSCSICTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, OSCSICTRL_AW, O>;
impl<'a, const O: u8> OSCSICTRL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OSCSICTRL_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OSCSICTRL_AW::VALUE2)
    }
}
#[doc = "OSCULCTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCULCTRL_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<OSCULCTRL_AW> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Clear"]
pub type OSCULCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, OSCULCTRL_AW, O>;
impl<'a, const O: u8> OSCULCTRL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OSCULCTRL_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OSCULCTRL_AW::VALUE2)
    }
}
#[doc = "RTC CTR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CTR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_CTR_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Clear"]
pub type RTC_CTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, RTC_CTR_AW, O>;
impl<'a, const O: u8> RTC_CTR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_CTR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_CTR_AW::VALUE2)
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_ATIM0_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Clear"]
pub type RTC_ATIM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, RTC_ATIM0_AW, O>;
impl<'a, const O: u8> RTC_ATIM0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_ATIM0_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_ATIM0_AW::VALUE2)
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_ATIM1_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Clear"]
pub type RTC_ATIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, RTC_ATIM1_AW, O>;
impl<'a, const O: u8> RTC_ATIM1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_ATIM1_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_ATIM1_AW::VALUE2)
    }
}
#[doc = "RTC TIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_TIM0_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Clear"]
pub type RTC_TIM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, RTC_TIM0_AW, O>;
impl<'a, const O: u8> RTC_TIM0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_TIM0_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_TIM0_AW::VALUE2)
    }
}
#[doc = "RTC TIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_TIM1_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Clear"]
pub type RTC_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, RTC_TIM1_AW, O>;
impl<'a, const O: u8> RTC_TIM1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_TIM1_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_TIM1_AW::VALUE2)
    }
}
#[doc = "Retention Memory Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMX_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RMX_AW> for bool {
    #[inline(always)]
    fn from(variant: RMX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Clear"]
pub type RMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCLR_SPEC, RMX_AW, O>;
impl<'a, const O: u8> RMX_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RMX_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RMX_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PRWARN_W<0> {
        PRWARN_W::new(self)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PI_W<1> {
        PI_W::new(self)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<2> {
        AI_W::new(self)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn dlrovr(&mut self) -> DLROVR_W<3> {
        DLROVR_W::new(self)
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpaccr(&mut self) -> LPACCR_W<6> {
        LPACCR_W::new(self)
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacth0(&mut self) -> LPACTH0_W<7> {
        LPACTH0_W::new(self)
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacth1(&mut self) -> LPACTH1_W<8> {
        LPACTH1_W::new(self)
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacst(&mut self) -> LPACST_W<9> {
        LPACST_W::new(self)
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacclr(&mut self) -> LPACCLR_W<10> {
        LPACCLR_W::new(self)
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacset(&mut self) -> LPACSET_W<11> {
        LPACSET_W::new(self)
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hintst(&mut self) -> HINTST_W<12> {
        HINTST_W::new(self)
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hintclr(&mut self) -> HINTCLR_W<13> {
        HINTCLR_W::new(self)
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hintset(&mut self) -> HINTSET_W<14> {
        HINTSET_W::new(self)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdclr(&mut self) -> HDCLR_W<17> {
        HDCLR_W::new(self)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdset(&mut self) -> HDSET_W<18> {
        HDSET_W::new(self)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdcr(&mut self) -> HDCR_W<19> {
        HDCR_W::new(self)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn oscsictrl(&mut self) -> OSCSICTRL_W<21> {
        OSCSICTRL_W::new(self)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn osculctrl(&mut self) -> OSCULCTRL_W<23> {
        OSCULCTRL_W::new(self)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W<24> {
        RTC_CTR_W::new(self)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W<25> {
        RTC_ATIM0_W::new(self)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W<26> {
        RTC_ATIM1_W::new(self)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W<27> {
        RTC_TIM0_W::new(self)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W<28> {
        RTC_TIM1_W::new(self)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rmx(&mut self) -> RMX_W<29> {
        RMX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCU Service Request Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srclr](index.html) module"]
pub struct SRCLR_SPEC;
impl crate::RegisterSpec for SRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [srclr::W](W) writer structure"]
impl crate::Writable for SRCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCLR to value 0"]
impl crate::Resettable for SRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
