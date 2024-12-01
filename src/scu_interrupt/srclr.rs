#[doc = "Register `SRCLR` writer"]
pub type W = crate::W<SRCLR_SPEC>;
#[doc = "WDT pre-warning Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRWARN_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<PRWARN_A> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Clear"]
pub type PRWARN_W<'a, REG> = crate::BitWriter<'a, REG, PRWARN_A>;
impl<'a, REG> PRWARN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_A::VALUE2)
    }
}
#[doc = "RTC Periodic Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PI_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<PI_A> for bool {
    #[inline(always)]
    fn from(variant: PI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Clear"]
pub type PI_W<'a, REG> = crate::BitWriter<'a, REG, PI_A>;
impl<'a, REG> PI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PI_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PI_A::VALUE2)
    }
}
#[doc = "RTC Alarm Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<AI_A> for bool {
    #[inline(always)]
    fn from(variant: AI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Clear"]
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG, AI_A>;
impl<'a, REG> AI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AI_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AI_A::VALUE2)
    }
}
#[doc = "DLR Request Overrun Interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLROVR_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<DLROVR_A> for bool {
    #[inline(always)]
    fn from(variant: DLROVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt clear"]
pub type DLROVR_W<'a, REG> = crate::BitWriter<'a, REG, DLROVR_A>;
impl<'a, REG> DLROVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_A::VALUE2)
    }
}
#[doc = "LPACLR Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCR_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACCR_A> for bool {
    #[inline(always)]
    fn from(variant: LPACCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCR` writer - LPACLR Mirror Register Update Interrupt Clear"]
pub type LPACCR_W<'a, REG> = crate::BitWriter<'a, REG, LPACCR_A>;
impl<'a, REG> LPACCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACCR_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACCR_A::VALUE2)
    }
}
#[doc = "LPACTH0 Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH0_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACTH0_A> for bool {
    #[inline(always)]
    fn from(variant: LPACTH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH0` writer - LPACTH0 Mirror Register Update Interrupt Clear"]
pub type LPACTH0_W<'a, REG> = crate::BitWriter<'a, REG, LPACTH0_A>;
impl<'a, REG> LPACTH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACTH0_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACTH0_A::VALUE2)
    }
}
#[doc = "LPACTH1 Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH1_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACTH1_A> for bool {
    #[inline(always)]
    fn from(variant: LPACTH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH1` writer - LPACTH1 Mirror Register Update Interrupt Clear"]
pub type LPACTH1_W<'a, REG> = crate::BitWriter<'a, REG, LPACTH1_A>;
impl<'a, REG> LPACTH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACTH1_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACTH1_A::VALUE2)
    }
}
#[doc = "LPACST Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACST_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACST_A> for bool {
    #[inline(always)]
    fn from(variant: LPACST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACST` writer - LPACST Mirror Register Update Interrupt Clear"]
pub type LPACST_W<'a, REG> = crate::BitWriter<'a, REG, LPACST_A>;
impl<'a, REG> LPACST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACST_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACST_A::VALUE2)
    }
}
#[doc = "LPACCLR Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCLR_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACCLR_A> for bool {
    #[inline(always)]
    fn from(variant: LPACCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCLR` writer - LPACCLR Mirror Register Update Interrupt Clear"]
pub type LPACCLR_W<'a, REG> = crate::BitWriter<'a, REG, LPACCLR_A>;
impl<'a, REG> LPACCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACCLR_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACCLR_A::VALUE2)
    }
}
#[doc = "LPACSET Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACSET_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<LPACSET_A> for bool {
    #[inline(always)]
    fn from(variant: LPACSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACSET` writer - LPACSET Mirror Register Update Interrupt Clear"]
pub type LPACSET_W<'a, REG> = crate::BitWriter<'a, REG, LPACSET_A>;
impl<'a, REG> LPACSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACSET_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACSET_A::VALUE2)
    }
}
#[doc = "HINTST Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTST_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HINTST_A> for bool {
    #[inline(always)]
    fn from(variant: HINTST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTST` writer - HINTST Mirror Register Update Interrupt Clear"]
pub type HINTST_W<'a, REG> = crate::BitWriter<'a, REG, HINTST_A>;
impl<'a, REG> HINTST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HINTST_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HINTST_A::VALUE2)
    }
}
#[doc = "HINTCLR Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTCLR_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTCLR` writer - HINTCLR Mirror Register Update Interrupt Clear"]
pub type HINTCLR_W<'a, REG> = crate::BitWriter<'a, REG, HINTCLR_A>;
impl<'a, REG> HINTCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HINTCLR_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HINTCLR_A::VALUE2)
    }
}
#[doc = "HINTSET Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTSET_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HINTSET_A> for bool {
    #[inline(always)]
    fn from(variant: HINTSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTSET` writer - HINTSET Mirror Register Update Interrupt Clear"]
pub type HINTSET_W<'a, REG> = crate::BitWriter<'a, REG, HINTSET_A>;
impl<'a, REG> HINTSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HINTSET_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HINTSET_A::VALUE2)
    }
}
#[doc = "HDCLR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCLR_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HDCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCLR` writer - HDCLR Mirror Register Update Clear"]
pub type HDCLR_W<'a, REG> = crate::BitWriter<'a, REG, HDCLR_A>;
impl<'a, REG> HDCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCLR_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HDCLR_A::VALUE2)
    }
}
#[doc = "HDSET Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSET_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HDSET_A> for bool {
    #[inline(always)]
    fn from(variant: HDSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSET` writer - HDSET Mirror Register Update Clear"]
pub type HDSET_W<'a, REG> = crate::BitWriter<'a, REG, HDSET_A>;
impl<'a, REG> HDSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HDSET_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HDSET_A::VALUE2)
    }
}
#[doc = "HDCR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCR_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HDCR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Clear"]
pub type HDCR_W<'a, REG> = crate::BitWriter<'a, REG, HDCR_A>;
impl<'a, REG> HDCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_A::VALUE2)
    }
}
#[doc = "OSCSICTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSICTRL_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<OSCSICTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Clear"]
pub type OSCSICTRL_W<'a, REG> = crate::BitWriter<'a, REG, OSCSICTRL_A>;
impl<'a, REG> OSCSICTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_A::VALUE2)
    }
}
#[doc = "OSCULCTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCULCTRL_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<OSCULCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Clear"]
pub type OSCULCTRL_W<'a, REG> = crate::BitWriter<'a, REG, OSCULCTRL_A>;
impl<'a, REG> OSCULCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_A::VALUE2)
    }
}
#[doc = "RTC CTR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CTR_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_CTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Clear"]
pub type RTC_CTR_W<'a, REG> = crate::BitWriter<'a, REG, RTC_CTR_A>;
impl<'a, REG> RTC_CTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_A::VALUE2)
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM0_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_ATIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Clear"]
pub type RTC_ATIM0_W<'a, REG> = crate::BitWriter<'a, REG, RTC_ATIM0_A>;
impl<'a, REG> RTC_ATIM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_A::VALUE2)
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM1_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_ATIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Clear"]
pub type RTC_ATIM1_W<'a, REG> = crate::BitWriter<'a, REG, RTC_ATIM1_A>;
impl<'a, REG> RTC_ATIM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_A::VALUE2)
    }
}
#[doc = "RTC TIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM0_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_TIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Clear"]
pub type RTC_TIM0_W<'a, REG> = crate::BitWriter<'a, REG, RTC_TIM0_A>;
impl<'a, REG> RTC_TIM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_A::VALUE2)
    }
}
#[doc = "RTC TIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM1_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_TIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Clear"]
pub type RTC_TIM1_W<'a, REG> = crate::BitWriter<'a, REG, RTC_TIM1_A>;
impl<'a, REG> RTC_TIM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_A::VALUE2)
    }
}
#[doc = "Retention Memory Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMX_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RMX_A> for bool {
    #[inline(always)]
    fn from(variant: RMX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Clear"]
pub type RMX_W<'a, REG> = crate::BitWriter<'a, REG, RMX_A>;
impl<'a, REG> RMX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RMX_A::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RMX_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Clear"]
    #[inline(always)]
    pub fn prwarn(&mut self) -> PRWARN_W<SRCLR_SPEC> {
        PRWARN_W::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Clear"]
    #[inline(always)]
    pub fn pi(&mut self) -> PI_W<SRCLR_SPEC> {
        PI_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Clear"]
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W<SRCLR_SPEC> {
        AI_W::new(self, 2)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt clear"]
    #[inline(always)]
    pub fn dlrovr(&mut self) -> DLROVR_W<SRCLR_SPEC> {
        DLROVR_W::new(self, 3)
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    pub fn lpaccr(&mut self) -> LPACCR_W<SRCLR_SPEC> {
        LPACCR_W::new(self, 6)
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    pub fn lpacth0(&mut self) -> LPACTH0_W<SRCLR_SPEC> {
        LPACTH0_W::new(self, 7)
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    pub fn lpacth1(&mut self) -> LPACTH1_W<SRCLR_SPEC> {
        LPACTH1_W::new(self, 8)
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    pub fn lpacst(&mut self) -> LPACST_W<SRCLR_SPEC> {
        LPACST_W::new(self, 9)
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    pub fn lpacclr(&mut self) -> LPACCLR_W<SRCLR_SPEC> {
        LPACCLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    pub fn lpacset(&mut self) -> LPACSET_W<SRCLR_SPEC> {
        LPACSET_W::new(self, 11)
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    pub fn hintst(&mut self) -> HINTST_W<SRCLR_SPEC> {
        HINTST_W::new(self, 12)
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    pub fn hintclr(&mut self) -> HINTCLR_W<SRCLR_SPEC> {
        HINTCLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    pub fn hintset(&mut self) -> HINTSET_W<SRCLR_SPEC> {
        HINTSET_W::new(self, 14)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Clear"]
    #[inline(always)]
    pub fn hdclr(&mut self) -> HDCLR_W<SRCLR_SPEC> {
        HDCLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Clear"]
    #[inline(always)]
    pub fn hdset(&mut self) -> HDSET_W<SRCLR_SPEC> {
        HDSET_W::new(self, 18)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Clear"]
    #[inline(always)]
    pub fn hdcr(&mut self) -> HDCR_W<SRCLR_SPEC> {
        HDCR_W::new(self, 19)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Clear"]
    #[inline(always)]
    pub fn oscsictrl(&mut self) -> OSCSICTRL_W<SRCLR_SPEC> {
        OSCSICTRL_W::new(self, 21)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Clear"]
    #[inline(always)]
    pub fn osculctrl(&mut self) -> OSCULCTRL_W<SRCLR_SPEC> {
        OSCULCTRL_W::new(self, 23)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Clear"]
    #[inline(always)]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W<SRCLR_SPEC> {
        RTC_CTR_W::new(self, 24)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Clear"]
    #[inline(always)]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W<SRCLR_SPEC> {
        RTC_ATIM0_W::new(self, 25)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Clear"]
    #[inline(always)]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W<SRCLR_SPEC> {
        RTC_ATIM1_W::new(self, 26)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Clear"]
    #[inline(always)]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W<SRCLR_SPEC> {
        RTC_TIM0_W::new(self, 27)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Clear"]
    #[inline(always)]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W<SRCLR_SPEC> {
        RTC_TIM1_W::new(self, 28)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Clear"]
    #[inline(always)]
    pub fn rmx(&mut self) -> RMX_W<SRCLR_SPEC> {
        RMX_W::new(self, 29)
    }
}
#[doc = "SCU Service Request Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCLR_SPEC;
impl crate::RegisterSpec for SRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srclr::W`](W) writer structure"]
impl crate::Writable for SRCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCLR to value 0"]
impl crate::Resettable for SRCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
