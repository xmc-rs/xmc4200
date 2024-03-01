#[doc = "Register `HRCCFG` reader"]
pub type R = crate::R<HrccfgSpec>;
#[doc = "Register `HRCCFG` writer"]
pub type W = crate::W<HrccfgSpec>;
#[doc = "High resolution channels power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrcpm {
    #[doc = "0: High resolution generation logic is OFF. It is not possible to generate high resolution signals throughout any of the high resolution channels, HRCy."]
    Value1 = 0,
    #[doc = "1: High resolution generation logic is ON. In this mode it is possible to generate a high resolution signal placement with the HRCy subunits."]
    Value2 = 1,
}
impl From<Hrcpm> for bool {
    #[inline(always)]
    fn from(variant: Hrcpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRCPM` reader - High resolution channels power mode"]
pub type HrcpmR = crate::BitReader<Hrcpm>;
impl HrcpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrcpm {
        match self.bits {
            false => Hrcpm::Value1,
            true => Hrcpm::Value2,
        }
    }
    #[doc = "High resolution generation logic is OFF. It is not possible to generate high resolution signals throughout any of the high resolution channels, HRCy."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hrcpm::Value1
    }
    #[doc = "High resolution generation logic is ON. In this mode it is possible to generate a high resolution signal placement with the HRCy subunits."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hrcpm::Value2
    }
}
#[doc = "Field `HRCPM` writer - High resolution channels power mode"]
pub type HrcpmW<'a, REG> = crate::BitWriter<'a, REG, Hrcpm>;
impl<'a, REG> HrcpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High resolution generation logic is OFF. It is not possible to generate high resolution signals throughout any of the high resolution channels, HRCy."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrcpm::Value1)
    }
    #[doc = "High resolution generation logic is ON. In this mode it is possible to generate a high resolution signal placement with the HRCy subunits."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hrcpm::Value2)
    }
}
#[doc = "HRC0 high resolution enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrc0e {
    #[doc = "0: HRC0 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC0 to generate an output PWM signal."]
    Value1 = 0,
    #[doc = "1: HRC0 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    Value2 = 1,
}
impl From<Hrc0e> for bool {
    #[inline(always)]
    fn from(variant: Hrc0e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRC0E` reader - HRC0 high resolution enable"]
pub type Hrc0eR = crate::BitReader<Hrc0e>;
impl Hrc0eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrc0e {
        match self.bits {
            false => Hrc0e::Value1,
            true => Hrc0e::Value2,
        }
    }
    #[doc = "HRC0 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC0 to generate an output PWM signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hrc0e::Value1
    }
    #[doc = "HRC0 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hrc0e::Value2
    }
}
#[doc = "Field `HRC0E` writer - HRC0 high resolution enable"]
pub type Hrc0eW<'a, REG> = crate::BitWriter<'a, REG, Hrc0e>;
impl<'a, REG> Hrc0eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRC0 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC0 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrc0e::Value1)
    }
    #[doc = "HRC0 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hrc0e::Value2)
    }
}
#[doc = "HRC1 high resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrc1e {
    #[doc = "0: HRC1 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC1 to generate an output PWM signal."]
    Value1 = 0,
    #[doc = "1: HRC1 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    Value2 = 1,
}
impl From<Hrc1e> for bool {
    #[inline(always)]
    fn from(variant: Hrc1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRC1E` reader - HRC1 high resolution channel enable"]
pub type Hrc1eR = crate::BitReader<Hrc1e>;
impl Hrc1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrc1e {
        match self.bits {
            false => Hrc1e::Value1,
            true => Hrc1e::Value2,
        }
    }
    #[doc = "HRC1 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC1 to generate an output PWM signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hrc1e::Value1
    }
    #[doc = "HRC1 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hrc1e::Value2
    }
}
#[doc = "Field `HRC1E` writer - HRC1 high resolution channel enable"]
pub type Hrc1eW<'a, REG> = crate::BitWriter<'a, REG, Hrc1e>;
impl<'a, REG> Hrc1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRC1 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC1 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrc1e::Value1)
    }
    #[doc = "HRC1 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hrc1e::Value2)
    }
}
#[doc = "HRC2 high resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrc2e {
    #[doc = "0: HRC2 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC2 to generate an output PWM signal."]
    Value1 = 0,
    #[doc = "1: HRC2 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    Value2 = 1,
}
impl From<Hrc2e> for bool {
    #[inline(always)]
    fn from(variant: Hrc2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRC2E` reader - HRC2 high resolution channel enable"]
pub type Hrc2eR = crate::BitReader<Hrc2e>;
impl Hrc2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrc2e {
        match self.bits {
            false => Hrc2e::Value1,
            true => Hrc2e::Value2,
        }
    }
    #[doc = "HRC2 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC2 to generate an output PWM signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hrc2e::Value1
    }
    #[doc = "HRC2 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hrc2e::Value2
    }
}
#[doc = "Field `HRC2E` writer - HRC2 high resolution channel enable"]
pub type Hrc2eW<'a, REG> = crate::BitWriter<'a, REG, Hrc2e>;
impl<'a, REG> Hrc2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRC2 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC2 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrc2e::Value1)
    }
    #[doc = "HRC2 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hrc2e::Value2)
    }
}
#[doc = "HRC3 high resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrc3e {
    #[doc = "0: HRC3 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC3 to generate an output PWM signal."]
    Value1 = 0,
    #[doc = "1: HRC3 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    Value2 = 1,
}
impl From<Hrc3e> for bool {
    #[inline(always)]
    fn from(variant: Hrc3e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRC3E` reader - HRC3 high resolution channel enable"]
pub type Hrc3eR = crate::BitReader<Hrc3e>;
impl Hrc3eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrc3e {
        match self.bits {
            false => Hrc3e::Value1,
            true => Hrc3e::Value2,
        }
    }
    #[doc = "HRC3 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC3 to generate an output PWM signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hrc3e::Value1
    }
    #[doc = "HRC3 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hrc3e::Value2
    }
}
#[doc = "Field `HRC3E` writer - HRC3 high resolution channel enable"]
pub type Hrc3eW<'a, REG> = crate::BitWriter<'a, REG, Hrc3e>;
impl<'a, REG> Hrc3eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRC3 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC3 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrc3e::Value1)
    }
    #[doc = "HRC3 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hrc3e::Value2)
    }
}
#[doc = "Clock information control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkc {
    #[doc = "0: No clock frequency is selected"]
    Value1 = 0,
    #[doc = "1: Module clock frequency is 180 MHz"]
    Value2 = 1,
    #[doc = "2: Module clock frequency is 120 MHz"]
    Value3 = 2,
    #[doc = "3: Module clock frequency is 80 MHz"]
    Value4 = 3,
}
impl From<Clkc> for u8 {
    #[inline(always)]
    fn from(variant: Clkc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkc {
    type Ux = u8;
}
#[doc = "Field `CLKC` reader - Clock information control"]
pub type ClkcR = crate::FieldReader<Clkc>;
impl ClkcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkc> {
        match self.bits {
            0 => Some(Clkc::Value1),
            1 => Some(Clkc::Value2),
            2 => Some(Clkc::Value3),
            3 => Some(Clkc::Value4),
            _ => None,
        }
    }
    #[doc = "No clock frequency is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Clkc::Value1
    }
    #[doc = "Module clock frequency is 180 MHz"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Clkc::Value2
    }
    #[doc = "Module clock frequency is 120 MHz"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Clkc::Value3
    }
    #[doc = "Module clock frequency is 80 MHz"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Clkc::Value4
    }
}
#[doc = "Field `CLKC` writer - Clock information control"]
pub type ClkcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clkc>;
impl<'a, REG> ClkcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock frequency is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkc::Value1)
    }
    #[doc = "Module clock frequency is 180 MHz"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkc::Value2)
    }
    #[doc = "Module clock frequency is 120 MHz"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Clkc::Value3)
    }
    #[doc = "Module clock frequency is 80 MHz"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkc::Value4)
    }
}
#[doc = "HRC0 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrc0e {
    #[doc = "0: HRC0 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC0 to generate an output PWM signal."]
    Value1 = 0,
    #[doc = "1: HRC0 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    Value2 = 1,
}
impl From<Lrc0e> for bool {
    #[inline(always)]
    fn from(variant: Lrc0e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRC0E` reader - HRC0 low resolution channel enable"]
pub type Lrc0eR = crate::BitReader<Lrc0e>;
impl Lrc0eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrc0e {
        match self.bits {
            false => Lrc0e::Value1,
            true => Lrc0e::Value2,
        }
    }
    #[doc = "HRC0 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC0 to generate an output PWM signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lrc0e::Value1
    }
    #[doc = "HRC0 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lrc0e::Value2
    }
}
#[doc = "Field `LRC0E` writer - HRC0 low resolution channel enable"]
pub type Lrc0eW<'a, REG> = crate::BitWriter<'a, REG, Lrc0e>;
impl<'a, REG> Lrc0eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRC0 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC0 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrc0e::Value1)
    }
    #[doc = "HRC0 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lrc0e::Value2)
    }
}
#[doc = "HRC1 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrc1e {
    #[doc = "0: HRC1 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC1 to generate an output PWM signal."]
    Value1 = 0,
    #[doc = "1: HRC1 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    Value2 = 1,
}
impl From<Lrc1e> for bool {
    #[inline(always)]
    fn from(variant: Lrc1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRC1E` reader - HRC1 low resolution channel enable"]
pub type Lrc1eR = crate::BitReader<Lrc1e>;
impl Lrc1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrc1e {
        match self.bits {
            false => Lrc1e::Value1,
            true => Lrc1e::Value2,
        }
    }
    #[doc = "HRC1 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC1 to generate an output PWM signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lrc1e::Value1
    }
    #[doc = "HRC1 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lrc1e::Value2
    }
}
#[doc = "Field `LRC1E` writer - HRC1 low resolution channel enable"]
pub type Lrc1eW<'a, REG> = crate::BitWriter<'a, REG, Lrc1e>;
impl<'a, REG> Lrc1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRC1 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC1 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrc1e::Value1)
    }
    #[doc = "HRC1 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lrc1e::Value2)
    }
}
#[doc = "HRC2 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrc2e {
    #[doc = "0: HRC2 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC2 to generate an output PWM signal."]
    Value1 = 0,
    #[doc = "1: HRC2 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    Value2 = 1,
}
impl From<Lrc2e> for bool {
    #[inline(always)]
    fn from(variant: Lrc2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRC2E` reader - HRC2 low resolution channel enable"]
pub type Lrc2eR = crate::BitReader<Lrc2e>;
impl Lrc2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrc2e {
        match self.bits {
            false => Lrc2e::Value1,
            true => Lrc2e::Value2,
        }
    }
    #[doc = "HRC2 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC2 to generate an output PWM signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lrc2e::Value1
    }
    #[doc = "HRC2 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lrc2e::Value2
    }
}
#[doc = "Field `LRC2E` writer - HRC2 low resolution channel enable"]
pub type Lrc2eW<'a, REG> = crate::BitWriter<'a, REG, Lrc2e>;
impl<'a, REG> Lrc2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRC2 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC2 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrc2e::Value1)
    }
    #[doc = "HRC2 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lrc2e::Value2)
    }
}
#[doc = "HRC3 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrc3e {
    #[doc = "0: HRC3 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC3 to generate an output PWM signal."]
    Value1 = 0,
    #[doc = "1: HRC3 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    Value2 = 1,
}
impl From<Lrc3e> for bool {
    #[inline(always)]
    fn from(variant: Lrc3e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRC3E` reader - HRC3 low resolution channel enable"]
pub type Lrc3eR = crate::BitReader<Lrc3e>;
impl Lrc3eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrc3e {
        match self.bits {
            false => Lrc3e::Value1,
            true => Lrc3e::Value2,
        }
    }
    #[doc = "HRC3 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC3 to generate an output PWM signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lrc3e::Value1
    }
    #[doc = "HRC3 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lrc3e::Value2
    }
}
#[doc = "Field `LRC3E` writer - HRC3 low resolution channel enable"]
pub type Lrc3eW<'a, REG> = crate::BitWriter<'a, REG, Lrc3e>;
impl<'a, REG> Lrc3eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRC3 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC3 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrc3e::Value1)
    }
    #[doc = "HRC3 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lrc3e::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - High resolution channels power mode"]
    #[inline(always)]
    pub fn hrcpm(&self) -> HrcpmR {
        HrcpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - HRC0 high resolution enable"]
    #[inline(always)]
    pub fn hrc0e(&self) -> Hrc0eR {
        Hrc0eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRC1 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc1e(&self) -> Hrc1eR {
        Hrc1eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HRC2 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc2e(&self) -> Hrc2eR {
        Hrc2eR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HRC3 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc3e(&self) -> Hrc3eR {
        Hrc3eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Clock information control"]
    #[inline(always)]
    pub fn clkc(&self) -> ClkcR {
        ClkcR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - HRC0 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc0e(&self) -> Lrc0eR {
        Lrc0eR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HRC1 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc1e(&self) -> Lrc1eR {
        Lrc1eR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HRC2 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc2e(&self) -> Lrc2eR {
        Lrc2eR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - HRC3 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc3e(&self) -> Lrc3eR {
        Lrc3eR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High resolution channels power mode"]
    #[inline(always)]
    #[must_use]
    pub fn hrcpm(&mut self) -> HrcpmW<HrccfgSpec> {
        HrcpmW::new(self, 0)
    }
    #[doc = "Bit 4 - HRC0 high resolution enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrc0e(&mut self) -> Hrc0eW<HrccfgSpec> {
        Hrc0eW::new(self, 4)
    }
    #[doc = "Bit 5 - HRC1 high resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrc1e(&mut self) -> Hrc1eW<HrccfgSpec> {
        Hrc1eW::new(self, 5)
    }
    #[doc = "Bit 6 - HRC2 high resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrc2e(&mut self) -> Hrc2eW<HrccfgSpec> {
        Hrc2eW::new(self, 6)
    }
    #[doc = "Bit 7 - HRC3 high resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrc3e(&mut self) -> Hrc3eW<HrccfgSpec> {
        Hrc3eW::new(self, 7)
    }
    #[doc = "Bits 16:18 - Clock information control"]
    #[inline(always)]
    #[must_use]
    pub fn clkc(&mut self) -> ClkcW<HrccfgSpec> {
        ClkcW::new(self, 16)
    }
    #[doc = "Bit 20 - HRC0 low resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn lrc0e(&mut self) -> Lrc0eW<HrccfgSpec> {
        Lrc0eW::new(self, 20)
    }
    #[doc = "Bit 21 - HRC1 low resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn lrc1e(&mut self) -> Lrc1eW<HrccfgSpec> {
        Lrc1eW::new(self, 21)
    }
    #[doc = "Bit 22 - HRC2 low resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn lrc2e(&mut self) -> Lrc2eW<HrccfgSpec> {
        Lrc2eW::new(self, 22)
    }
    #[doc = "Bit 23 - HRC3 low resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn lrc3e(&mut self) -> Lrc3eW<HrccfgSpec> {
        Lrc3eW::new(self, 23)
    }
}
#[doc = "Global HRC configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrccfgSpec;
impl crate::RegisterSpec for HrccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrccfg::R`](R) reader structure"]
impl crate::Readable for HrccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hrccfg::W`](W) writer structure"]
impl crate::Writable for HrccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRCCFG to value 0"]
impl crate::Resettable for HrccfgSpec {
    const RESET_VALUE: u32 = 0;
}
