#[doc = "Register `SRCLR` writer"]
pub type W = crate::W<SrclrSpec>;
#[doc = "WDT pre-warning Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prwarn {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Prwarn> for bool {
    #[inline(always)]
    fn from(variant: Prwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Clear"]
pub type PrwarnW<'a, REG> = crate::BitWriter<'a, REG, Prwarn>;
impl<'a, REG> PrwarnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Prwarn::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Prwarn::Value2)
    }
}
#[doc = "RTC Periodic Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pi {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Pi> for bool {
    #[inline(always)]
    fn from(variant: Pi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Clear"]
pub type PiW<'a, REG> = crate::BitWriter<'a, REG, Pi>;
impl<'a, REG> PiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pi::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pi::Value2)
    }
}
#[doc = "RTC Alarm Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ai {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Ai> for bool {
    #[inline(always)]
    fn from(variant: Ai) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Clear"]
pub type AiW<'a, REG> = crate::BitWriter<'a, REG, Ai>;
impl<'a, REG> AiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ai::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ai::Value2)
    }
}
#[doc = "DLR Request Overrun Interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlrovr {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Dlrovr> for bool {
    #[inline(always)]
    fn from(variant: Dlrovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt clear"]
pub type DlrovrW<'a, REG> = crate::BitWriter<'a, REG, Dlrovr>;
impl<'a, REG> DlrovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlrovr::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dlrovr::Value2)
    }
}
#[doc = "LPACLR Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpaccr {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Lpaccr> for bool {
    #[inline(always)]
    fn from(variant: Lpaccr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCR` writer - LPACLR Mirror Register Update Interrupt Clear"]
pub type LpaccrW<'a, REG> = crate::BitWriter<'a, REG, Lpaccr>;
impl<'a, REG> LpaccrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpaccr::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpaccr::Value2)
    }
}
#[doc = "LPACTH0 Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacth0 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Lpacth0> for bool {
    #[inline(always)]
    fn from(variant: Lpacth0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH0` writer - LPACTH0 Mirror Register Update Interrupt Clear"]
pub type Lpacth0W<'a, REG> = crate::BitWriter<'a, REG, Lpacth0>;
impl<'a, REG> Lpacth0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacth0::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacth0::Value2)
    }
}
#[doc = "LPACTH1 Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacth1 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Lpacth1> for bool {
    #[inline(always)]
    fn from(variant: Lpacth1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH1` writer - LPACTH1 Mirror Register Update Interrupt Clear"]
pub type Lpacth1W<'a, REG> = crate::BitWriter<'a, REG, Lpacth1>;
impl<'a, REG> Lpacth1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacth1::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacth1::Value2)
    }
}
#[doc = "LPACST Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacst {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Lpacst> for bool {
    #[inline(always)]
    fn from(variant: Lpacst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACST` writer - LPACST Mirror Register Update Interrupt Clear"]
pub type LpacstW<'a, REG> = crate::BitWriter<'a, REG, Lpacst>;
impl<'a, REG> LpacstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacst::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacst::Value2)
    }
}
#[doc = "LPACCLR Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacclr {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Lpacclr> for bool {
    #[inline(always)]
    fn from(variant: Lpacclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCLR` writer - LPACCLR Mirror Register Update Interrupt Clear"]
pub type LpacclrW<'a, REG> = crate::BitWriter<'a, REG, Lpacclr>;
impl<'a, REG> LpacclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacclr::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacclr::Value2)
    }
}
#[doc = "LPACSET Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacset {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Lpacset> for bool {
    #[inline(always)]
    fn from(variant: Lpacset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACSET` writer - LPACSET Mirror Register Update Interrupt Clear"]
pub type LpacsetW<'a, REG> = crate::BitWriter<'a, REG, Lpacset>;
impl<'a, REG> LpacsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacset::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacset::Value2)
    }
}
#[doc = "HINTST Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hintst {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Hintst> for bool {
    #[inline(always)]
    fn from(variant: Hintst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTST` writer - HINTST Mirror Register Update Interrupt Clear"]
pub type HintstW<'a, REG> = crate::BitWriter<'a, REG, Hintst>;
impl<'a, REG> HintstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hintst::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hintst::Value2)
    }
}
#[doc = "HINTCLR Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hintclr {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Hintclr> for bool {
    #[inline(always)]
    fn from(variant: Hintclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTCLR` writer - HINTCLR Mirror Register Update Interrupt Clear"]
pub type HintclrW<'a, REG> = crate::BitWriter<'a, REG, Hintclr>;
impl<'a, REG> HintclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hintclr::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hintclr::Value2)
    }
}
#[doc = "HINTSET Mirror Register Update Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hintset {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Hintset> for bool {
    #[inline(always)]
    fn from(variant: Hintset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTSET` writer - HINTSET Mirror Register Update Interrupt Clear"]
pub type HintsetW<'a, REG> = crate::BitWriter<'a, REG, Hintset>;
impl<'a, REG> HintsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hintset::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hintset::Value2)
    }
}
#[doc = "HDCLR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdclr {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Hdclr> for bool {
    #[inline(always)]
    fn from(variant: Hdclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCLR` writer - HDCLR Mirror Register Update Clear"]
pub type HdclrW<'a, REG> = crate::BitWriter<'a, REG, Hdclr>;
impl<'a, REG> HdclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdclr::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hdclr::Value2)
    }
}
#[doc = "HDSET Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdset {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Hdset> for bool {
    #[inline(always)]
    fn from(variant: Hdset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSET` writer - HDSET Mirror Register Update Clear"]
pub type HdsetW<'a, REG> = crate::BitWriter<'a, REG, Hdset>;
impl<'a, REG> HdsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdset::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hdset::Value2)
    }
}
#[doc = "HDCR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcr {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Hdcr> for bool {
    #[inline(always)]
    fn from(variant: Hdcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Clear"]
pub type HdcrW<'a, REG> = crate::BitWriter<'a, REG, Hdcr>;
impl<'a, REG> HdcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcr::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcr::Value2)
    }
}
#[doc = "OSCSICTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscsictrl {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Oscsictrl> for bool {
    #[inline(always)]
    fn from(variant: Oscsictrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Clear"]
pub type OscsictrlW<'a, REG> = crate::BitWriter<'a, REG, Oscsictrl>;
impl<'a, REG> OscsictrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsictrl::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsictrl::Value2)
    }
}
#[doc = "OSCULCTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osculctrl {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Osculctrl> for bool {
    #[inline(always)]
    fn from(variant: Osculctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Clear"]
pub type OsculctrlW<'a, REG> = crate::BitWriter<'a, REG, Osculctrl>;
impl<'a, REG> OsculctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Osculctrl::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Osculctrl::Value2)
    }
}
#[doc = "RTC CTR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcCtr {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<RtcCtr> for bool {
    #[inline(always)]
    fn from(variant: RtcCtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Clear"]
pub type RtcCtrW<'a, REG> = crate::BitWriter<'a, REG, RtcCtr>;
impl<'a, REG> RtcCtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCtr::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCtr::Value2)
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim0 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<RtcAtim0> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Clear"]
pub type RtcAtim0W<'a, REG> = crate::BitWriter<'a, REG, RtcAtim0>;
impl<'a, REG> RtcAtim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim0::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim0::Value2)
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim1 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<RtcAtim1> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Clear"]
pub type RtcAtim1W<'a, REG> = crate::BitWriter<'a, REG, RtcAtim1>;
impl<'a, REG> RtcAtim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim1::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim1::Value2)
    }
}
#[doc = "RTC TIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim0 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<RtcTim0> for bool {
    #[inline(always)]
    fn from(variant: RtcTim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Clear"]
pub type RtcTim0W<'a, REG> = crate::BitWriter<'a, REG, RtcTim0>;
impl<'a, REG> RtcTim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim0::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim0::Value2)
    }
}
#[doc = "RTC TIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim1 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<RtcTim1> for bool {
    #[inline(always)]
    fn from(variant: RtcTim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Clear"]
pub type RtcTim1W<'a, REG> = crate::BitWriter<'a, REG, RtcTim1>;
impl<'a, REG> RtcTim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim1::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim1::Value2)
    }
}
#[doc = "Retention Memory Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmx {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the status bit"]
    Value2 = 1,
}
impl From<Rmx> for bool {
    #[inline(always)]
    fn from(variant: Rmx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Clear"]
pub type RmxW<'a, REG> = crate::BitWriter<'a, REG, Rmx>;
impl<'a, REG> RmxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmx::Value1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rmx::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PrwarnW<SrclrSpec> {
        PrwarnW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PiW<SrclrSpec> {
        PiW::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AiW<SrclrSpec> {
        AiW::new(self, 2)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn dlrovr(&mut self) -> DlrovrW<SrclrSpec> {
        DlrovrW::new(self, 3)
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpaccr(&mut self) -> LpaccrW<SrclrSpec> {
        LpaccrW::new(self, 6)
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacth0(&mut self) -> Lpacth0W<SrclrSpec> {
        Lpacth0W::new(self, 7)
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacth1(&mut self) -> Lpacth1W<SrclrSpec> {
        Lpacth1W::new(self, 8)
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacst(&mut self) -> LpacstW<SrclrSpec> {
        LpacstW::new(self, 9)
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacclr(&mut self) -> LpacclrW<SrclrSpec> {
        LpacclrW::new(self, 10)
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpacset(&mut self) -> LpacsetW<SrclrSpec> {
        LpacsetW::new(self, 11)
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hintst(&mut self) -> HintstW<SrclrSpec> {
        HintstW::new(self, 12)
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hintclr(&mut self) -> HintclrW<SrclrSpec> {
        HintclrW::new(self, 13)
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hintset(&mut self) -> HintsetW<SrclrSpec> {
        HintsetW::new(self, 14)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdclr(&mut self) -> HdclrW<SrclrSpec> {
        HdclrW::new(self, 17)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdset(&mut self) -> HdsetW<SrclrSpec> {
        HdsetW::new(self, 18)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdcr(&mut self) -> HdcrW<SrclrSpec> {
        HdcrW::new(self, 19)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn oscsictrl(&mut self) -> OscsictrlW<SrclrSpec> {
        OscsictrlW::new(self, 21)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn osculctrl(&mut self) -> OsculctrlW<SrclrSpec> {
        OsculctrlW::new(self, 23)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctr(&mut self) -> RtcCtrW<SrclrSpec> {
        RtcCtrW::new(self, 24)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim0(&mut self) -> RtcAtim0W<SrclrSpec> {
        RtcAtim0W::new(self, 25)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim1(&mut self) -> RtcAtim1W<SrclrSpec> {
        RtcAtim1W::new(self, 26)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim0(&mut self) -> RtcTim0W<SrclrSpec> {
        RtcTim0W::new(self, 27)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim1(&mut self) -> RtcTim1W<SrclrSpec> {
        RtcTim1W::new(self, 28)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rmx(&mut self) -> RmxW<SrclrSpec> {
        RmxW::new(self, 29)
    }
}
#[doc = "SCU Service Request Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrclrSpec;
impl crate::RegisterSpec for SrclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srclr::W`](W) writer structure"]
impl crate::Writable for SrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCLR to value 0"]
impl crate::Resettable for SrclrSpec {
    const RESET_VALUE: u32 = 0;
}
