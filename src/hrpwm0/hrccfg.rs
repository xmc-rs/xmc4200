#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HRCCFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `HRCPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRCPMR {
    #[doc = "High resolution generation logic is OFF. It is not possible to generate high resolution signals throughout any of the high resolution channels, HRCy."]
    VALUE1,
    #[doc = "High resolution generation logic is ON. In this mode it is possible to generate a high resolution signal placement with the HRCy subunits."]
    VALUE2,
}
impl HRCPMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HRCPMR::VALUE1 => false,
            HRCPMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRCPMR {
        match value {
            false => HRCPMR::VALUE1,
            true => HRCPMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HRCPMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HRCPMR::VALUE2
    }
}
#[doc = "Possible values of the field `HRC0E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRC0ER {
    #[doc = "HRC0 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC0 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC0 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2,
}
impl HRC0ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HRC0ER::VALUE1 => false,
            HRC0ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRC0ER {
        match value {
            false => HRC0ER::VALUE1,
            true => HRC0ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HRC0ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HRC0ER::VALUE2
    }
}
#[doc = "Possible values of the field `HRC1E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRC1ER {
    #[doc = "HRC1 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC1 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC1 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2,
}
impl HRC1ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HRC1ER::VALUE1 => false,
            HRC1ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRC1ER {
        match value {
            false => HRC1ER::VALUE1,
            true => HRC1ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HRC1ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HRC1ER::VALUE2
    }
}
#[doc = "Possible values of the field `HRC2E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRC2ER {
    #[doc = "HRC2 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC2 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC2 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2,
}
impl HRC2ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HRC2ER::VALUE1 => false,
            HRC2ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRC2ER {
        match value {
            false => HRC2ER::VALUE1,
            true => HRC2ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HRC2ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HRC2ER::VALUE2
    }
}
#[doc = "Possible values of the field `HRC3E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRC3ER {
    #[doc = "HRC3 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC3 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC3 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2,
}
impl HRC3ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HRC3ER::VALUE1 => false,
            HRC3ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRC3ER {
        match value {
            false => HRC3ER::VALUE1,
            true => HRC3ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HRC3ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HRC3ER::VALUE2
    }
}
#[doc = "Possible values of the field `CLKC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKCR {
    #[doc = "No clock frequency is selected"]
    VALUE1,
    #[doc = "Module clock frequency is 180 MHz"]
    VALUE2,
    #[doc = "Module clock frequency is 120 MHz"]
    VALUE3,
    #[doc = "Module clock frequency is 80 MHz"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKCR::VALUE1 => 0,
            CLKCR::VALUE2 => 1,
            CLKCR::VALUE3 => 2,
            CLKCR::VALUE4 => 3,
            CLKCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKCR {
        match value {
            0 => CLKCR::VALUE1,
            1 => CLKCR::VALUE2,
            2 => CLKCR::VALUE3,
            3 => CLKCR::VALUE4,
            i => CLKCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLKCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLKCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CLKCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CLKCR::VALUE4
    }
}
#[doc = "Possible values of the field `LRC0E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRC0ER {
    #[doc = "HRC0 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC0 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC0 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2,
}
impl LRC0ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LRC0ER::VALUE1 => false,
            LRC0ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRC0ER {
        match value {
            false => LRC0ER::VALUE1,
            true => LRC0ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LRC0ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LRC0ER::VALUE2
    }
}
#[doc = "Possible values of the field `LRC1E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRC1ER {
    #[doc = "HRC1 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC1 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC1 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2,
}
impl LRC1ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LRC1ER::VALUE1 => false,
            LRC1ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRC1ER {
        match value {
            false => LRC1ER::VALUE1,
            true => LRC1ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LRC1ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LRC1ER::VALUE2
    }
}
#[doc = "Possible values of the field `LRC2E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRC2ER {
    #[doc = "HRC2 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC2 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC2 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2,
}
impl LRC2ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LRC2ER::VALUE1 => false,
            LRC2ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRC2ER {
        match value {
            false => LRC2ER::VALUE1,
            true => LRC2ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LRC2ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LRC2ER::VALUE2
    }
}
#[doc = "Possible values of the field `LRC3E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRC3ER {
    #[doc = "HRC3 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC3 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC3 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2,
}
impl LRC3ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LRC3ER::VALUE1 => false,
            LRC3ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRC3ER {
        match value {
            false => LRC3ER::VALUE1,
            true => LRC3ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LRC3ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LRC3ER::VALUE2
    }
}
#[doc = "Values that can be written to the field `HRCPM`"]
pub enum HRCPMW {
    #[doc = "High resolution generation logic is OFF. It is not possible to generate high resolution signals throughout any of the high resolution channels, HRCy."]
    VALUE1,
    #[doc = "High resolution generation logic is ON. In this mode it is possible to generate a high resolution signal placement with the HRCy subunits."]
    VALUE2,
}
impl HRCPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRCPMW::VALUE1 => false,
            HRCPMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRCPMW<'a> {
    w: &'a mut W,
}
impl<'a> _HRCPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRCPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High resolution generation logic is OFF. It is not possible to generate high resolution signals throughout any of the high resolution channels, HRCy."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRCPMW::VALUE1)
    }
    #[doc = "High resolution generation logic is ON. In this mode it is possible to generate a high resolution signal placement with the HRCy subunits."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRCPMW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HRC0E`"]
pub enum HRC0EW {
    #[doc = "HRC0 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC0 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC0 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2,
}
impl HRC0EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRC0EW::VALUE1 => false,
            HRC0EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRC0EW<'a> {
    w: &'a mut W,
}
impl<'a> _HRC0EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRC0EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRC0 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC0 to generate an output PWM signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRC0EW::VALUE1)
    }
    #[doc = "HRC0 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRC0EW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HRC1E`"]
pub enum HRC1EW {
    #[doc = "HRC1 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC1 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC1 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2,
}
impl HRC1EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRC1EW::VALUE1 => false,
            HRC1EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRC1EW<'a> {
    w: &'a mut W,
}
impl<'a> _HRC1EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRC1EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRC1 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC1 to generate an output PWM signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRC1EW::VALUE1)
    }
    #[doc = "HRC1 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRC1EW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HRC2E`"]
pub enum HRC2EW {
    #[doc = "HRC2 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC2 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC2 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2,
}
impl HRC2EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRC2EW::VALUE1 => false,
            HRC2EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRC2EW<'a> {
    w: &'a mut W,
}
impl<'a> _HRC2EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRC2EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRC2 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC2 to generate an output PWM signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRC2EW::VALUE1)
    }
    #[doc = "HRC2 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRC2EW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HRC3E`"]
pub enum HRC3EW {
    #[doc = "HRC3 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC3 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC3 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2,
}
impl HRC3EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRC3EW::VALUE1 => false,
            HRC3EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRC3EW<'a> {
    w: &'a mut W,
}
impl<'a> _HRC3EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRC3EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRC3 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC3 to generate an output PWM signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRC3EW::VALUE1)
    }
    #[doc = "HRC3 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRC3EW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKC`"]
pub enum CLKCW {
    #[doc = "No clock frequency is selected"]
    VALUE1,
    #[doc = "Module clock frequency is 180 MHz"]
    VALUE2,
    #[doc = "Module clock frequency is 120 MHz"]
    VALUE3,
    #[doc = "Module clock frequency is 80 MHz"]
    VALUE4,
}
impl CLKCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKCW::VALUE1 => 0,
            CLKCW::VALUE2 => 1,
            CLKCW::VALUE3 => 2,
            CLKCW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No clock frequency is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKCW::VALUE1)
    }
    #[doc = "Module clock frequency is 180 MHz"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLKCW::VALUE2)
    }
    #[doc = "Module clock frequency is 120 MHz"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLKCW::VALUE3)
    }
    #[doc = "Module clock frequency is 80 MHz"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLKCW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LRC0E`"]
pub enum LRC0EW {
    #[doc = "HRC0 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC0 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC0 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2,
}
impl LRC0EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRC0EW::VALUE1 => false,
            LRC0EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRC0EW<'a> {
    w: &'a mut W,
}
impl<'a> _LRC0EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRC0EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRC0 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC0 to generate an output PWM signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LRC0EW::VALUE1)
    }
    #[doc = "HRC0 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LRC0EW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LRC1E`"]
pub enum LRC1EW {
    #[doc = "HRC1 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC1 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC1 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2,
}
impl LRC1EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRC1EW::VALUE1 => false,
            LRC1EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRC1EW<'a> {
    w: &'a mut W,
}
impl<'a> _LRC1EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRC1EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRC1 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC1 to generate an output PWM signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LRC1EW::VALUE1)
    }
    #[doc = "HRC1 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LRC1EW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LRC2E`"]
pub enum LRC2EW {
    #[doc = "HRC2 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC2 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC2 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2,
}
impl LRC2EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRC2EW::VALUE1 => false,
            LRC2EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRC2EW<'a> {
    w: &'a mut W,
}
impl<'a> _LRC2EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRC2EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRC2 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC2 to generate an output PWM signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LRC2EW::VALUE1)
    }
    #[doc = "HRC2 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LRC2EW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LRC3E`"]
pub enum LRC3EW {
    #[doc = "HRC3 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC3 to generate an output PWM signal."]
    VALUE1,
    #[doc = "HRC3 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2,
}
impl LRC3EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRC3EW::VALUE1 => false,
            LRC3EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRC3EW<'a> {
    w: &'a mut W,
}
impl<'a> _LRC3EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRC3EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRC3 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC3 to generate an output PWM signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LRC3EW::VALUE1)
    }
    #[doc = "HRC3 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LRC3EW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - High resolution channels power mode"]
    #[inline]
    pub fn hrcpm(&self) -> HRCPMR {
        HRCPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - HRC0 high resolution enable"]
    #[inline]
    pub fn hrc0e(&self) -> HRC0ER {
        HRC0ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - HRC1 high resolution channel enable"]
    #[inline]
    pub fn hrc1e(&self) -> HRC1ER {
        HRC1ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - HRC2 high resolution channel enable"]
    #[inline]
    pub fn hrc2e(&self) -> HRC2ER {
        HRC2ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - HRC3 high resolution channel enable"]
    #[inline]
    pub fn hrc3e(&self) -> HRC3ER {
        HRC3ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Clock information control"]
    #[inline]
    pub fn clkc(&self) -> CLKCR {
        CLKCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - HRC0 low resolution channel enable"]
    #[inline]
    pub fn lrc0e(&self) -> LRC0ER {
        LRC0ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - HRC1 low resolution channel enable"]
    #[inline]
    pub fn lrc1e(&self) -> LRC1ER {
        LRC1ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - HRC2 low resolution channel enable"]
    #[inline]
    pub fn lrc2e(&self) -> LRC2ER {
        LRC2ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - HRC3 low resolution channel enable"]
    #[inline]
    pub fn lrc3e(&self) -> LRC3ER {
        LRC3ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - High resolution channels power mode"]
    #[inline]
    pub fn hrcpm(&mut self) -> _HRCPMW {
        _HRCPMW { w: self }
    }
    #[doc = "Bit 4 - HRC0 high resolution enable"]
    #[inline]
    pub fn hrc0e(&mut self) -> _HRC0EW {
        _HRC0EW { w: self }
    }
    #[doc = "Bit 5 - HRC1 high resolution channel enable"]
    #[inline]
    pub fn hrc1e(&mut self) -> _HRC1EW {
        _HRC1EW { w: self }
    }
    #[doc = "Bit 6 - HRC2 high resolution channel enable"]
    #[inline]
    pub fn hrc2e(&mut self) -> _HRC2EW {
        _HRC2EW { w: self }
    }
    #[doc = "Bit 7 - HRC3 high resolution channel enable"]
    #[inline]
    pub fn hrc3e(&mut self) -> _HRC3EW {
        _HRC3EW { w: self }
    }
    #[doc = "Bits 16:18 - Clock information control"]
    #[inline]
    pub fn clkc(&mut self) -> _CLKCW {
        _CLKCW { w: self }
    }
    #[doc = "Bit 20 - HRC0 low resolution channel enable"]
    #[inline]
    pub fn lrc0e(&mut self) -> _LRC0EW {
        _LRC0EW { w: self }
    }
    #[doc = "Bit 21 - HRC1 low resolution channel enable"]
    #[inline]
    pub fn lrc1e(&mut self) -> _LRC1EW {
        _LRC1EW { w: self }
    }
    #[doc = "Bit 22 - HRC2 low resolution channel enable"]
    #[inline]
    pub fn lrc2e(&mut self) -> _LRC2EW {
        _LRC2EW { w: self }
    }
    #[doc = "Bit 23 - HRC3 low resolution channel enable"]
    #[inline]
    pub fn lrc3e(&mut self) -> _LRC3EW {
        _LRC3EW { w: self }
    }
}
