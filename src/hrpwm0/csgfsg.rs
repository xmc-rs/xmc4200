#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CSGFSG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `S0RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RBR {
    #[doc = "Slope generation is stopped."]
    VALUE1,
    #[doc = "Slope generation is running."]
    VALUE2,
}
impl S0RBR {
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
            S0RBR::VALUE1 => false,
            S0RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0RBR {
        match value {
            false => S0RBR::VALUE1,
            true => S0RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S0RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S0RBR::VALUE2
    }
}
#[doc = "Possible values of the field `P0RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0RBR {
    #[doc = "Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    VALUE1,
    #[doc = "Prescaler is running."]
    VALUE2,
}
impl P0RBR {
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
            P0RBR::VALUE1 => false,
            P0RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P0RBR {
        match value {
            false => P0RBR::VALUE1,
            true => P0RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P0RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P0RBR::VALUE2
    }
}
#[doc = "Possible values of the field `S1RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1RBR {
    #[doc = "Slope generation is stopped."]
    VALUE1,
    #[doc = "Slope generation is running."]
    VALUE2,
}
impl S1RBR {
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
            S1RBR::VALUE1 => false,
            S1RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1RBR {
        match value {
            false => S1RBR::VALUE1,
            true => S1RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S1RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S1RBR::VALUE2
    }
}
#[doc = "Possible values of the field `P1RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1RBR {
    #[doc = "Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    VALUE1,
    #[doc = "Prescaler is running."]
    VALUE2,
}
impl P1RBR {
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
            P1RBR::VALUE1 => false,
            P1RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1RBR {
        match value {
            false => P1RBR::VALUE1,
            true => P1RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P1RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P1RBR::VALUE2
    }
}
#[doc = "Possible values of the field `S2RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2RBR {
    #[doc = "Slope generation is stopped."]
    VALUE1,
    #[doc = "Slope generation is running."]
    VALUE2,
}
impl S2RBR {
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
            S2RBR::VALUE1 => false,
            S2RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2RBR {
        match value {
            false => S2RBR::VALUE1,
            true => S2RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S2RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S2RBR::VALUE2
    }
}
#[doc = "Possible values of the field `P2RB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2RBR {
    #[doc = "Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    VALUE1,
    #[doc = "Prescaler is running."]
    VALUE2,
}
impl P2RBR {
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
            P2RBR::VALUE1 => false,
            P2RBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2RBR {
        match value {
            false => P2RBR::VALUE1,
            true => P2RBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == P2RBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == P2RBR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DAC0 slope generation status"]
    #[inline]
    pub fn s0rb(&self) -> S0RBR {
        S0RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CSG0 prescaler status"]
    #[inline]
    pub fn p0rb(&self) -> P0RBR {
        P0RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - DAC1 slope generation status"]
    #[inline]
    pub fn s1rb(&self) -> S1RBR {
        S1RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CSG1 prescaler status"]
    #[inline]
    pub fn p1rb(&self) -> P1RBR {
        P1RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - DAC2 slope generation status"]
    #[inline]
    pub fn s2rb(&self) -> S2RBR {
        S2RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - CSG2 prescaler status"]
    #[inline]
    pub fn p2rb(&self) -> P2RBR {
        P2RBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
