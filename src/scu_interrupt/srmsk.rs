#[doc = "Register `SRMSK` reader"]
pub type R = crate::R<SRMSK_SPEC>;
#[doc = "Register `SRMSK` writer"]
pub type W = crate::W<SRMSK_SPEC>;
#[doc = "WDT pre-warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRWARN_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PRWARN_A> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` reader - WDT pre-warning Interrupt Mask"]
pub type PRWARN_R = crate::BitReader<PRWARN_A>;
impl PRWARN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRWARN_A {
        match self.bits {
            false => PRWARN_A::VALUE1,
            true => PRWARN_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRWARN_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRWARN_A::VALUE2
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Mask"]
pub type PRWARN_W<'a, REG> = crate::BitWriter<'a, REG, PRWARN_A>;
impl<'a, REG> PRWARN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_A::VALUE2)
    }
}
#[doc = "RTC Periodic Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PI_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PI_A> for bool {
    #[inline(always)]
    fn from(variant: PI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` reader - RTC Periodic Interrupt Mask"]
pub type PI_R = crate::BitReader<PI_A>;
impl PI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PI_A {
        match self.bits {
            false => PI_A::VALUE1,
            true => PI_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PI_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PI_A::VALUE2
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Mask"]
pub type PI_W<'a, REG> = crate::BitWriter<'a, REG, PI_A>;
impl<'a, REG> PI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PI_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PI_A::VALUE2)
    }
}
#[doc = "RTC Alarm Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<AI_A> for bool {
    #[inline(always)]
    fn from(variant: AI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` reader - RTC Alarm Interrupt Mask"]
pub type AI_R = crate::BitReader<AI_A>;
impl AI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AI_A {
        match self.bits {
            false => AI_A::VALUE1,
            true => AI_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AI_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AI_A::VALUE2
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Mask"]
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG, AI_A>;
impl<'a, REG> AI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AI_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AI_A::VALUE2)
    }
}
#[doc = "DLR Request Overrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLROVR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<DLROVR_A> for bool {
    #[inline(always)]
    fn from(variant: DLROVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLROVR` reader - DLR Request Overrun Interrupt Mask"]
pub type DLROVR_R = crate::BitReader<DLROVR_A>;
impl DLROVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLROVR_A {
        match self.bits {
            false => DLROVR_A::VALUE1,
            true => DLROVR_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLROVR_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLROVR_A::VALUE2
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt Mask"]
pub type DLROVR_W<'a, REG> = crate::BitWriter<'a, REG, DLROVR_A>;
impl<'a, REG> DLROVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_A::VALUE2)
    }
}
#[doc = "LPACLR Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACCR_A> for bool {
    #[inline(always)]
    fn from(variant: LPACCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCR` reader - LPACLR Mirror Register Update Interrupt Mask"]
pub type LPACCR_R = crate::BitReader<LPACCR_A>;
impl LPACCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPACCR_A {
        match self.bits {
            false => LPACCR_A::VALUE1,
            true => LPACCR_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACCR_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACCR_A::VALUE2
    }
}
#[doc = "Field `LPACCR` writer - LPACLR Mirror Register Update Interrupt Mask"]
pub type LPACCR_W<'a, REG> = crate::BitWriter<'a, REG, LPACCR_A>;
impl<'a, REG> LPACCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACCR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACCR_A::VALUE2)
    }
}
#[doc = "LPACTH0 Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACTH0_A> for bool {
    #[inline(always)]
    fn from(variant: LPACTH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH0` reader - LPACTH0 Mirror Register Update Interrupt Mask"]
pub type LPACTH0_R = crate::BitReader<LPACTH0_A>;
impl LPACTH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPACTH0_A {
        match self.bits {
            false => LPACTH0_A::VALUE1,
            true => LPACTH0_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACTH0_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACTH0_A::VALUE2
    }
}
#[doc = "Field `LPACTH0` writer - LPACTH0 Mirror Register Update Interrupt Mask"]
pub type LPACTH0_W<'a, REG> = crate::BitWriter<'a, REG, LPACTH0_A>;
impl<'a, REG> LPACTH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACTH0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACTH0_A::VALUE2)
    }
}
#[doc = "LPACTH1 Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACTH1_A> for bool {
    #[inline(always)]
    fn from(variant: LPACTH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH1` reader - LPACTH1 Mirror Register Update Interrupt Mask"]
pub type LPACTH1_R = crate::BitReader<LPACTH1_A>;
impl LPACTH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPACTH1_A {
        match self.bits {
            false => LPACTH1_A::VALUE1,
            true => LPACTH1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACTH1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACTH1_A::VALUE2
    }
}
#[doc = "Field `LPACTH1` writer - LPACTH1 Mirror Register Update Interrupt Mask"]
pub type LPACTH1_W<'a, REG> = crate::BitWriter<'a, REG, LPACTH1_A>;
impl<'a, REG> LPACTH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACTH1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACTH1_A::VALUE2)
    }
}
#[doc = "LPACST Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACST_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACST_A> for bool {
    #[inline(always)]
    fn from(variant: LPACST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACST` reader - LPACST Mirror Register Update Interrupt Mask"]
pub type LPACST_R = crate::BitReader<LPACST_A>;
impl LPACST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPACST_A {
        match self.bits {
            false => LPACST_A::VALUE1,
            true => LPACST_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACST_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACST_A::VALUE2
    }
}
#[doc = "Field `LPACST` writer - LPACST Mirror Register Update Interrupt Mask"]
pub type LPACST_W<'a, REG> = crate::BitWriter<'a, REG, LPACST_A>;
impl<'a, REG> LPACST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACST_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACST_A::VALUE2)
    }
}
#[doc = "LPACCLR Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCLR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACCLR_A> for bool {
    #[inline(always)]
    fn from(variant: LPACCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCLR` reader - LPACCLR Mirror Register Update Interrupt Mask"]
pub type LPACCLR_R = crate::BitReader<LPACCLR_A>;
impl LPACCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPACCLR_A {
        match self.bits {
            false => LPACCLR_A::VALUE1,
            true => LPACCLR_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACCLR_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACCLR_A::VALUE2
    }
}
#[doc = "Field `LPACCLR` writer - LPACCLR Mirror Register Update Interrupt Mask"]
pub type LPACCLR_W<'a, REG> = crate::BitWriter<'a, REG, LPACCLR_A>;
impl<'a, REG> LPACCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACCLR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACCLR_A::VALUE2)
    }
}
#[doc = "LPACSET Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACSET_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACSET_A> for bool {
    #[inline(always)]
    fn from(variant: LPACSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACSET` reader - LPACSET Mirror Register Update Interrupt Mask"]
pub type LPACSET_R = crate::BitReader<LPACSET_A>;
impl LPACSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPACSET_A {
        match self.bits {
            false => LPACSET_A::VALUE1,
            true => LPACSET_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACSET_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACSET_A::VALUE2
    }
}
#[doc = "Field `LPACSET` writer - LPACSET Mirror Register Update Interrupt Mask"]
pub type LPACSET_W<'a, REG> = crate::BitWriter<'a, REG, LPACSET_A>;
impl<'a, REG> LPACSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LPACSET_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LPACSET_A::VALUE2)
    }
}
#[doc = "HINTST Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTST_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HINTST_A> for bool {
    #[inline(always)]
    fn from(variant: HINTST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTST` reader - HINTST Mirror Register Update Interrupt Mask"]
pub type HINTST_R = crate::BitReader<HINTST_A>;
impl HINTST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HINTST_A {
        match self.bits {
            false => HINTST_A::VALUE1,
            true => HINTST_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTST_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTST_A::VALUE2
    }
}
#[doc = "Field `HINTST` writer - HINTST Mirror Register Update Interrupt Mask"]
pub type HINTST_W<'a, REG> = crate::BitWriter<'a, REG, HINTST_A>;
impl<'a, REG> HINTST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HINTST_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HINTST_A::VALUE2)
    }
}
#[doc = "HINTCLR Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTCLR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTCLR` reader - HINTCLR Mirror Register Update Interrupt Mask"]
pub type HINTCLR_R = crate::BitReader<HINTCLR_A>;
impl HINTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HINTCLR_A {
        match self.bits {
            false => HINTCLR_A::VALUE1,
            true => HINTCLR_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTCLR_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTCLR_A::VALUE2
    }
}
#[doc = "Field `HINTCLR` writer - HINTCLR Mirror Register Update Interrupt Mask"]
pub type HINTCLR_W<'a, REG> = crate::BitWriter<'a, REG, HINTCLR_A>;
impl<'a, REG> HINTCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HINTCLR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HINTCLR_A::VALUE2)
    }
}
#[doc = "HINTSET Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTSET_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HINTSET_A> for bool {
    #[inline(always)]
    fn from(variant: HINTSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTSET` reader - HINTSET Mirror Register Update Interrupt Mask"]
pub type HINTSET_R = crate::BitReader<HINTSET_A>;
impl HINTSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HINTSET_A {
        match self.bits {
            false => HINTSET_A::VALUE1,
            true => HINTSET_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTSET_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTSET_A::VALUE2
    }
}
#[doc = "Field `HINTSET` writer - HINTSET Mirror Register Update Interrupt Mask"]
pub type HINTSET_W<'a, REG> = crate::BitWriter<'a, REG, HINTSET_A>;
impl<'a, REG> HINTSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HINTSET_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HINTSET_A::VALUE2)
    }
}
#[doc = "HDCLR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCLR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HDCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Update Mask"]
pub type HDCLR_R = crate::BitReader<HDCLR_A>;
impl HDCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDCLR_A {
        match self.bits {
            false => HDCLR_A::VALUE1,
            true => HDCLR_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDCLR_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDCLR_A::VALUE2
    }
}
#[doc = "Field `HDCLR` writer - HDCLR Mirror Register Update Mask"]
pub type HDCLR_W<'a, REG> = crate::BitWriter<'a, REG, HDCLR_A>;
impl<'a, REG> HDCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCLR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HDCLR_A::VALUE2)
    }
}
#[doc = "HDSET Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSET_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HDSET_A> for bool {
    #[inline(always)]
    fn from(variant: HDSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSET` reader - HDSET Mirror Register Update Mask"]
pub type HDSET_R = crate::BitReader<HDSET_A>;
impl HDSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDSET_A {
        match self.bits {
            false => HDSET_A::VALUE1,
            true => HDSET_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDSET_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDSET_A::VALUE2
    }
}
#[doc = "Field `HDSET` writer - HDSET Mirror Register Update Mask"]
pub type HDSET_W<'a, REG> = crate::BitWriter<'a, REG, HDSET_A>;
impl<'a, REG> HDSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HDSET_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HDSET_A::VALUE2)
    }
}
#[doc = "HDCR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HDCR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` reader - HDCR Mirror Register Update Mask"]
pub type HDCR_R = crate::BitReader<HDCR_A>;
impl HDCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDCR_A {
        match self.bits {
            false => HDCR_A::VALUE1,
            true => HDCR_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDCR_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDCR_A::VALUE2
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Mask"]
pub type HDCR_W<'a, REG> = crate::BitWriter<'a, REG, HDCR_A>;
impl<'a, REG> HDCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_A::VALUE2)
    }
}
#[doc = "OSCSICTRL Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSICTRL_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<OSCSICTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Update Mask"]
pub type OSCSICTRL_R = crate::BitReader<OSCSICTRL_A>;
impl OSCSICTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCSICTRL_A {
        match self.bits {
            false => OSCSICTRL_A::VALUE1,
            true => OSCSICTRL_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCSICTRL_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCSICTRL_A::VALUE2
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Mask"]
pub type OSCSICTRL_W<'a, REG> = crate::BitWriter<'a, REG, OSCSICTRL_A>;
impl<'a, REG> OSCSICTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_A::VALUE2)
    }
}
#[doc = "OSCULCTRL Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCULCTRL_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<OSCULCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Update Mask"]
pub type OSCULCTRL_R = crate::BitReader<OSCULCTRL_A>;
impl OSCULCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCULCTRL_A {
        match self.bits {
            false => OSCULCTRL_A::VALUE1,
            true => OSCULCTRL_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCULCTRL_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCULCTRL_A::VALUE2
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Mask"]
pub type OSCULCTRL_W<'a, REG> = crate::BitWriter<'a, REG, OSCULCTRL_A>;
impl<'a, REG> OSCULCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_A::VALUE2)
    }
}
#[doc = "RTC CTR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CTR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_CTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Update Mask"]
pub type RTC_CTR_R = crate::BitReader<RTC_CTR_A>;
impl RTC_CTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_CTR_A {
        match self.bits {
            false => RTC_CTR_A::VALUE1,
            true => RTC_CTR_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_CTR_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_CTR_A::VALUE2
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Mask"]
pub type RTC_CTR_W<'a, REG> = crate::BitWriter<'a, REG, RTC_CTR_A>;
impl<'a, REG> RTC_CTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_A::VALUE2)
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_ATIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Update Mask"]
pub type RTC_ATIM0_R = crate::BitReader<RTC_ATIM0_A>;
impl RTC_ATIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_ATIM0_A {
        match self.bits {
            false => RTC_ATIM0_A::VALUE1,
            true => RTC_ATIM0_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM0_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM0_A::VALUE2
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Mask"]
pub type RTC_ATIM0_W<'a, REG> = crate::BitWriter<'a, REG, RTC_ATIM0_A>;
impl<'a, REG> RTC_ATIM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_A::VALUE2)
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_ATIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Update Mask"]
pub type RTC_ATIM1_R = crate::BitReader<RTC_ATIM1_A>;
impl RTC_ATIM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_ATIM1_A {
        match self.bits {
            false => RTC_ATIM1_A::VALUE1,
            true => RTC_ATIM1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM1_A::VALUE2
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Mask"]
pub type RTC_ATIM1_W<'a, REG> = crate::BitWriter<'a, REG, RTC_ATIM1_A>;
impl<'a, REG> RTC_ATIM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_A::VALUE2)
    }
}
#[doc = "RTC TIM0 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_TIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Update Mask"]
pub type RTC_TIM0_R = crate::BitReader<RTC_TIM0_A>;
impl RTC_TIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_TIM0_A {
        match self.bits {
            false => RTC_TIM0_A::VALUE1,
            true => RTC_TIM0_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM0_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM0_A::VALUE2
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Mask"]
pub type RTC_TIM0_W<'a, REG> = crate::BitWriter<'a, REG, RTC_TIM0_A>;
impl<'a, REG> RTC_TIM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_A::VALUE2)
    }
}
#[doc = "RTC TIM1 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_TIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Update Mask"]
pub type RTC_TIM1_R = crate::BitReader<RTC_TIM1_A>;
impl RTC_TIM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTC_TIM1_A {
        match self.bits {
            false => RTC_TIM1_A::VALUE1,
            true => RTC_TIM1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM1_A::VALUE2
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Mask"]
pub type RTC_TIM1_W<'a, REG> = crate::BitWriter<'a, REG, RTC_TIM1_A>;
impl<'a, REG> RTC_TIM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_A::VALUE2)
    }
}
#[doc = "Retention Memory Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMX_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RMX_A> for bool {
    #[inline(always)]
    fn from(variant: RMX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` reader - Retention Memory Mirror Register Update Mask"]
pub type RMX_R = crate::BitReader<RMX_A>;
impl RMX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RMX_A {
        match self.bits {
            false => RMX_A::VALUE1,
            true => RMX_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RMX_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RMX_A::VALUE2
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Mask"]
pub type RMX_W<'a, REG> = crate::BitWriter<'a, REG, RMX_A>;
impl<'a, REG> RMX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RMX_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RMX_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Mask"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn dlrovr(&self) -> DLROVR_R {
        DLROVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpaccr(&self) -> LPACCR_R {
        LPACCR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacth0(&self) -> LPACTH0_R {
        LPACTH0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacth1(&self) -> LPACTH1_R {
        LPACTH1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacst(&self) -> LPACST_R {
        LPACST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacclr(&self) -> LPACCLR_R {
        LPACCLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacset(&self) -> LPACSET_R {
        LPACSET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintst(&self) -> HINTST_R {
        HINTST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintclr(&self) -> HINTCLR_R {
        HINTCLR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintset(&self) -> HINTSET_R {
        HINTSET_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PRWARN_W<SRMSK_SPEC> {
        PRWARN_W::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PI_W<SRMSK_SPEC> {
        PI_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<SRMSK_SPEC> {
        AI_W::new(self, 2)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dlrovr(&mut self) -> DLROVR_W<SRMSK_SPEC> {
        DLROVR_W::new(self, 3)
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpaccr(&mut self) -> LPACCR_W<SRMSK_SPEC> {
        LPACCR_W::new(self, 6)
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacth0(&mut self) -> LPACTH0_W<SRMSK_SPEC> {
        LPACTH0_W::new(self, 7)
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacth1(&mut self) -> LPACTH1_W<SRMSK_SPEC> {
        LPACTH1_W::new(self, 8)
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacst(&mut self) -> LPACST_W<SRMSK_SPEC> {
        LPACST_W::new(self, 9)
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacclr(&mut self) -> LPACCLR_W<SRMSK_SPEC> {
        LPACCLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacset(&mut self) -> LPACSET_W<SRMSK_SPEC> {
        LPACSET_W::new(self, 11)
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hintst(&mut self) -> HINTST_W<SRMSK_SPEC> {
        HINTST_W::new(self, 12)
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hintclr(&mut self) -> HINTCLR_W<SRMSK_SPEC> {
        HINTCLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hintset(&mut self) -> HINTSET_W<SRMSK_SPEC> {
        HINTSET_W::new(self, 14)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hdclr(&mut self) -> HDCLR_W<SRMSK_SPEC> {
        HDCLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hdset(&mut self) -> HDSET_W<SRMSK_SPEC> {
        HDSET_W::new(self, 18)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hdcr(&mut self) -> HDCR_W<SRMSK_SPEC> {
        HDCR_W::new(self, 19)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn oscsictrl(&mut self) -> OSCSICTRL_W<SRMSK_SPEC> {
        OSCSICTRL_W::new(self, 21)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn osculctrl(&mut self) -> OSCULCTRL_W<SRMSK_SPEC> {
        OSCULCTRL_W::new(self, 23)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W<SRMSK_SPEC> {
        RTC_CTR_W::new(self, 24)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W<SRMSK_SPEC> {
        RTC_ATIM0_W::new(self, 25)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W<SRMSK_SPEC> {
        RTC_ATIM1_W::new(self, 26)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W<SRMSK_SPEC> {
        RTC_TIM0_W::new(self, 27)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W<SRMSK_SPEC> {
        RTC_TIM1_W::new(self, 28)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rmx(&mut self) -> RMX_W<SRMSK_SPEC> {
        RMX_W::new(self, 29)
    }
}
#[doc = "SCU Service Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRMSK_SPEC;
impl crate::RegisterSpec for SRMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srmsk::R`](R) reader structure"]
impl crate::Readable for SRMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srmsk::W`](W) writer structure"]
impl crate::Writable for SRMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRMSK to value 0"]
impl crate::Resettable for SRMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
