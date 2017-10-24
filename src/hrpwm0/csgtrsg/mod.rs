#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CSGTRSG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `D0STE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D0STER {
    #[doc = "Shadow transfer has been performed."]
    VALUE1,
    #[doc = "Shadow transfer has been requested but is still pending completion."]
    VALUE2,
}
impl D0STER {
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
            D0STER::VALUE1 => false,
            D0STER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D0STER {
        match value {
            false => D0STER::VALUE1,
            true => D0STER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == D0STER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == D0STER::VALUE2
    }
}
#[doc = "Possible values of the field `SW0ST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW0STR {
    #[doc = "Inverting input connected to HRPWMx.C0I[A]"]
    VALUE1,
    #[doc = "Inverting input connected to HRPWMx.C0I[B]"]
    VALUE2,
}
impl SW0STR {
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
            SW0STR::VALUE1 => false,
            SW0STR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SW0STR {
        match value {
            false => SW0STR::VALUE1,
            true => SW0STR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SW0STR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SW0STR::VALUE2
    }
}
#[doc = "Possible values of the field `D1STE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D1STER {
    #[doc = "Shadow transfer has been performed."]
    VALUE1,
    #[doc = "Shadow transfer has been requested but is still pending completion."]
    VALUE2,
}
impl D1STER {
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
            D1STER::VALUE1 => false,
            D1STER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D1STER {
        match value {
            false => D1STER::VALUE1,
            true => D1STER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == D1STER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == D1STER::VALUE2
    }
}
#[doc = "Possible values of the field `SW1ST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW1STR {
    #[doc = "Inverting input connected to HRPWMx.C1I[A]"]
    VALUE1,
    #[doc = "Inverting input connected to HRPWMx.C1I[B]"]
    VALUE2,
}
impl SW1STR {
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
            SW1STR::VALUE1 => false,
            SW1STR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SW1STR {
        match value {
            false => SW1STR::VALUE1,
            true => SW1STR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SW1STR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SW1STR::VALUE2
    }
}
#[doc = "Possible values of the field `D2STE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D2STER {
    #[doc = "Shadow transfer has been performed."]
    VALUE1,
    #[doc = "Shadow transfer has been requested but is still pending completion."]
    VALUE2,
}
impl D2STER {
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
            D2STER::VALUE1 => false,
            D2STER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D2STER {
        match value {
            false => D2STER::VALUE1,
            true => D2STER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == D2STER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == D2STER::VALUE2
    }
}
#[doc = "Possible values of the field `SW2ST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW2STR {
    #[doc = "Inverting input connected to HRPWMx.C2I[A]"]
    VALUE1,
    #[doc = "Inverting input connected to HRPWMx.C2I[B]"]
    VALUE2,
}
impl SW2STR {
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
            SW2STR::VALUE1 => false,
            SW2STR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SW2STR {
        match value {
            false => SW2STR::VALUE1,
            true => SW2STR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SW2STR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SW2STR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DAC0 shadow transfer enable"]
    #[inline]
    pub fn d0ste(&self) -> D0STER {
        D0STER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CMP0 inverting input connection status"]
    #[inline]
    pub fn sw0st(&self) -> SW0STR {
        SW0STR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable"]
    #[inline]
    pub fn d1ste(&self) -> D1STER {
        D1STER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - CMP1 inverting input connection status"]
    #[inline]
    pub fn sw1st(&self) -> SW1STR {
        SW1STR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable"]
    #[inline]
    pub fn d2ste(&self) -> D2STER {
        D2STER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CMP2 inverting input connection status"]
    #[inline]
    pub fn sw2st(&self) -> SW2STR {
        SW2STR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
