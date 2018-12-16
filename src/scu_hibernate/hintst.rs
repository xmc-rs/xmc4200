#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HINTST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `HIBNINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBNINTR {
    #[doc = "Hibernate not entered"]
    VALUE1,
    #[doc = "Hibernate entered"]
    VALUE2,
}
impl HIBNINTR {
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
            HIBNINTR::VALUE1 => false,
            HIBNINTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBNINTR {
        match value {
            false => HIBNINTR::VALUE1,
            true => HIBNINTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBNINTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBNINTR::VALUE2
    }
}
#[doc = "Possible values of the field `FLASHOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHOFFR {
    #[doc = "VDDP supply of Flash switched on"]
    VALUE1,
    #[doc = "VDDP supply of Flash switched off"]
    VALUE2,
}
impl FLASHOFFR {
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
            FLASHOFFR::VALUE1 => false,
            FLASHOFFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHOFFR {
        match value {
            false => FLASHOFFR::VALUE1,
            true => FLASHOFFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FLASHOFFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FLASHOFFR::VALUE2
    }
}
#[doc = "Possible values of the field `FLASHPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHPDR {
    #[doc = "Normal mode"]
    VALUE1,
    #[doc = "Power down mode effectively entered"]
    VALUE2,
}
impl FLASHPDR {
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
            FLASHPDR::VALUE1 => false,
            FLASHPDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHPDR {
        match value {
            false => FLASHPDR::VALUE1,
            true => FLASHPDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FLASHPDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FLASHPDR::VALUE2
    }
}
#[doc = "Possible values of the field `POFFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFFDR {
    #[doc = "Pull-up on"]
    VALUE1,
    #[doc = "Pull-up off"]
    VALUE2,
}
impl POFFDR {
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
            POFFDR::VALUE1 => false,
            POFFDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POFFDR {
        match value {
            false => POFFDR::VALUE1,
            true => POFFDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == POFFDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == POFFDR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PPODELR {
    bits: u8,
}
impl PPODELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `POFFH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFFHR {
    #[doc = "Pull-up on"]
    VALUE1,
    #[doc = "Pull-up off"]
    VALUE2,
}
impl POFFHR {
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
            POFFHR::VALUE1 => false,
            POFFHR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POFFHR {
        match value {
            false => POFFHR::VALUE1,
            true => POFFHR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == POFFHR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == POFFHR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request State"]
    #[inline]
    pub fn hibnint(&self) -> HIBNINTR {
        HIBNINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash State"]
    #[inline]
    pub fn flashoff(&self) -> FLASHOFFR {
        FLASHOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Flash Power Down State"]
    #[inline]
    pub fn flashpd(&self) -> FLASHPDR {
        FLASHPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control State"]
    #[inline]
    pub fn poffd(&self) -> POFFDR {
        POFFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request"]
    #[inline]
    pub fn ppodel(&self) -> PPODELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PPODELR { bits }
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode State"]
    #[inline]
    pub fn poffh(&self) -> POFFHR {
        POFFHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
