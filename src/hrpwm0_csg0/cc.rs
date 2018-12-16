#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CC {
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
#[doc = "Possible values of the field `IBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBSR {
    #[doc = "HRPWMx.BLyA"]
    VALUE1,
    #[doc = "HRPWMx.BLyB"]
    VALUE2,
    #[doc = "HRPWMx.BLyC"]
    VALUE3,
    #[doc = "HRPWMx.BLyD"]
    VALUE4,
    #[doc = "HRPWMx.BLyE"]
    VALUE5,
    #[doc = "HRPWMx.BLyF"]
    VALUE6,
    #[doc = "HRPWMx.BLyG"]
    VALUE7,
    #[doc = "HRPWMx.BLyH"]
    VALUE8,
    #[doc = "HRPWMx.BLyI"]
    VALUE9,
    #[doc = "HRPWMx.BLyJ"]
    VALUE10,
    #[doc = "HRPWMx.BLyK"]
    VALUE11,
    #[doc = "HRPWMx.BLyL"]
    VALUE12,
    #[doc = "HRPWMx.BLyM"]
    VALUE13,
    #[doc = "HRPWMx.BLyN"]
    VALUE14,
    #[doc = "HRPWMx.BLyO"]
    VALUE15,
    #[doc = "HRPWMx.BLyP"]
    VALUE16,
}
impl IBSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IBSR::VALUE1 => 0,
            IBSR::VALUE2 => 1,
            IBSR::VALUE3 => 2,
            IBSR::VALUE4 => 3,
            IBSR::VALUE5 => 4,
            IBSR::VALUE6 => 5,
            IBSR::VALUE7 => 6,
            IBSR::VALUE8 => 7,
            IBSR::VALUE9 => 8,
            IBSR::VALUE10 => 9,
            IBSR::VALUE11 => 10,
            IBSR::VALUE12 => 11,
            IBSR::VALUE13 => 12,
            IBSR::VALUE14 => 13,
            IBSR::VALUE15 => 14,
            IBSR::VALUE16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IBSR {
        match value {
            0 => IBSR::VALUE1,
            1 => IBSR::VALUE2,
            2 => IBSR::VALUE3,
            3 => IBSR::VALUE4,
            4 => IBSR::VALUE5,
            5 => IBSR::VALUE6,
            6 => IBSR::VALUE7,
            7 => IBSR::VALUE8,
            8 => IBSR::VALUE9,
            9 => IBSR::VALUE10,
            10 => IBSR::VALUE11,
            11 => IBSR::VALUE12,
            12 => IBSR::VALUE13,
            13 => IBSR::VALUE14,
            14 => IBSR::VALUE15,
            15 => IBSR::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IBSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IBSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == IBSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == IBSR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == IBSR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == IBSR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == IBSR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == IBSR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == IBSR::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == IBSR::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == IBSR::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == IBSR::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == IBSR::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == IBSR::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == IBSR::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == IBSR::VALUE16
    }
}
#[doc = "Possible values of the field `IMCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMCSR {
    #[doc = "HRPWMx.CyINA"]
    VALUE1,
    #[doc = "HRPWMx.CyINB"]
    VALUE2,
}
impl IMCSR {
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
            IMCSR::VALUE1 => false,
            IMCSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMCSR {
        match value {
            false => IMCSR::VALUE1,
            true => IMCSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IMCSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IMCSR::VALUE2
    }
}
#[doc = "Possible values of the field `IMCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMCCR {
    #[doc = "Dynamic switch disabled"]
    VALUE1,
    #[doc = "Comparator input is connected to HRPWMx.CyINB when the control signal is HIGH"]
    VALUE2,
    #[doc = "Comparator input is connected to HRPWMx.CyINA when the control signal is HIGH"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IMCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IMCCR::VALUE1 => 0,
            IMCCR::VALUE2 => 1,
            IMCCR::VALUE3 => 2,
            IMCCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IMCCR {
        match value {
            0 => IMCCR::VALUE1,
            1 => IMCCR::VALUE2,
            2 => IMCCR::VALUE3,
            i => IMCCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IMCCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IMCCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == IMCCR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct ESER {
    bits: bool,
}
impl ESER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct OIER {
    bits: bool,
}
impl OIER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct OSER {
    bits: bool,
}
impl OSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `BLMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLMCR {
    #[doc = "Blanking disabled"]
    VALUE1,
    #[doc = "Blanking on a LOW to HIGH transition"]
    VALUE2,
    #[doc = "Blanking on a HIGH to LOW transition"]
    VALUE3,
    #[doc = "Blanking on both transitions"]
    VALUE4,
}
impl BLMCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BLMCR::VALUE1 => 0,
            BLMCR::VALUE2 => 1,
            BLMCR::VALUE3 => 2,
            BLMCR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BLMCR {
        match value {
            0 => BLMCR::VALUE1,
            1 => BLMCR::VALUE2,
            2 => BLMCR::VALUE3,
            3 => BLMCR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BLMCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BLMCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BLMCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BLMCR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct EBER {
    bits: bool,
}
impl EBER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `COFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COFER {
    #[doc = "Filtering stage disabled"]
    VALUE1,
    #[doc = "Filtering stage enabled"]
    VALUE2,
}
impl COFER {
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
            COFER::VALUE1 => false,
            COFER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COFER {
        match value {
            false => COFER::VALUE1,
            true => COFER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COFER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COFER::VALUE2
    }
}
#[doc = "Possible values of the field `COFM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COFMR {
    #[doc = "Comparator Output needs to be stable for 2 clock cycles"]
    VALUE1,
    #[doc = "Comparator Output needs to be stable for 3 clock cycles"]
    VALUE2,
    #[doc = "Comparator Output needs to be stable for 4 clock cycles"]
    VALUE3,
    #[doc = "Comparator Output needs to be stable for 5 clock cycles"]
    VALUE4,
    #[doc = "Comparator Output needs to be stable for 14 clock cycles"]
    VALUE5,
    #[doc = "Comparator Output needs to be stable for 15 clock cycles"]
    VALUE6,
    #[doc = "Comparator Output needs to be stable for 16 clock cycles"]
    VALUE7,
    #[doc = "Comparator Output needs to be stable for 32 clock cycles"]
    VALUE8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl COFMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COFMR::VALUE1 => 0,
            COFMR::VALUE2 => 1,
            COFMR::VALUE3 => 2,
            COFMR::VALUE4 => 3,
            COFMR::VALUE5 => 12,
            COFMR::VALUE6 => 13,
            COFMR::VALUE7 => 14,
            COFMR::VALUE8 => 15,
            COFMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COFMR {
        match value {
            0 => COFMR::VALUE1,
            1 => COFMR::VALUE2,
            2 => COFMR::VALUE3,
            3 => COFMR::VALUE4,
            12 => COFMR::VALUE5,
            13 => COFMR::VALUE6,
            14 => COFMR::VALUE7,
            15 => COFMR::VALUE8,
            i => COFMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COFMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COFMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == COFMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == COFMR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == COFMR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == COFMR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == COFMR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == COFMR::VALUE8
    }
}
#[doc = "Possible values of the field `COFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COFCR {
    #[doc = "Filtering is always done if enabled"]
    VALUE1,
    #[doc = "Filtering is only done when CSGyDSV1 value is currently fed to the DAC"]
    VALUE2,
    #[doc = "Filtering is only done when the CSGyDSV2 value is currently fed to the DAC"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl COFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COFCR::VALUE1 => 0,
            COFCR::VALUE2 => 1,
            COFCR::VALUE3 => 2,
            COFCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COFCR {
        match value {
            0 => COFCR::VALUE1,
            1 => COFCR::VALUE2,
            2 => COFCR::VALUE3,
            i => COFCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COFCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COFCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == COFCR::VALUE3
    }
}
#[doc = "Values that can be written to the field `IBS`"]
pub enum IBSW {
    #[doc = "HRPWMx.BLyA"]
    VALUE1,
    #[doc = "HRPWMx.BLyB"]
    VALUE2,
    #[doc = "HRPWMx.BLyC"]
    VALUE3,
    #[doc = "HRPWMx.BLyD"]
    VALUE4,
    #[doc = "HRPWMx.BLyE"]
    VALUE5,
    #[doc = "HRPWMx.BLyF"]
    VALUE6,
    #[doc = "HRPWMx.BLyG"]
    VALUE7,
    #[doc = "HRPWMx.BLyH"]
    VALUE8,
    #[doc = "HRPWMx.BLyI"]
    VALUE9,
    #[doc = "HRPWMx.BLyJ"]
    VALUE10,
    #[doc = "HRPWMx.BLyK"]
    VALUE11,
    #[doc = "HRPWMx.BLyL"]
    VALUE12,
    #[doc = "HRPWMx.BLyM"]
    VALUE13,
    #[doc = "HRPWMx.BLyN"]
    VALUE14,
    #[doc = "HRPWMx.BLyO"]
    VALUE15,
    #[doc = "HRPWMx.BLyP"]
    VALUE16,
}
impl IBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IBSW::VALUE1 => 0,
            IBSW::VALUE2 => 1,
            IBSW::VALUE3 => 2,
            IBSW::VALUE4 => 3,
            IBSW::VALUE5 => 4,
            IBSW::VALUE6 => 5,
            IBSW::VALUE7 => 6,
            IBSW::VALUE8 => 7,
            IBSW::VALUE9 => 8,
            IBSW::VALUE10 => 9,
            IBSW::VALUE11 => 10,
            IBSW::VALUE12 => 11,
            IBSW::VALUE13 => 12,
            IBSW::VALUE14 => 13,
            IBSW::VALUE15 => 14,
            IBSW::VALUE16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBSW<'a> {
    w: &'a mut W,
}
impl<'a> _IBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "HRPWMx.BLyA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IBSW::VALUE1)
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IBSW::VALUE2)
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(IBSW::VALUE3)
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(IBSW::VALUE4)
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(IBSW::VALUE5)
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(IBSW::VALUE6)
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(IBSW::VALUE7)
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(IBSW::VALUE8)
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(IBSW::VALUE9)
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(IBSW::VALUE10)
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(IBSW::VALUE11)
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(IBSW::VALUE12)
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(IBSW::VALUE13)
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(IBSW::VALUE14)
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(IBSW::VALUE15)
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(IBSW::VALUE16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IMCS`"]
pub enum IMCSW {
    #[doc = "HRPWMx.CyINA"]
    VALUE1,
    #[doc = "HRPWMx.CyINB"]
    VALUE2,
}
impl IMCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IMCSW::VALUE1 => false,
            IMCSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMCSW<'a> {
    w: &'a mut W,
}
impl<'a> _IMCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRPWMx.CyINA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IMCSW::VALUE1)
    }
    #[doc = "HRPWMx.CyINB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IMCSW::VALUE2)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IMCC`"]
pub enum IMCCW {
    #[doc = "Dynamic switch disabled"]
    VALUE1,
    #[doc = "Comparator input is connected to HRPWMx.CyINB when the control signal is HIGH"]
    VALUE2,
    #[doc = "Comparator input is connected to HRPWMx.CyINA when the control signal is HIGH"]
    VALUE3,
}
impl IMCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IMCCW::VALUE1 => 0,
            IMCCW::VALUE2 => 1,
            IMCCW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMCCW<'a> {
    w: &'a mut W,
}
impl<'a> _IMCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMCCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Dynamic switch disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IMCCW::VALUE1)
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINB when the control signal is HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IMCCW::VALUE2)
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINA when the control signal is HIGH"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(IMCCW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ESEW<'a> {
    w: &'a mut W,
}
impl<'a> _ESEW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OIEW<'a> {
    w: &'a mut W,
}
impl<'a> _OIEW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSEW<'a> {
    w: &'a mut W,
}
impl<'a> _OSEW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLMC`"]
pub enum BLMCW {
    #[doc = "Blanking disabled"]
    VALUE1,
    #[doc = "Blanking on a LOW to HIGH transition"]
    VALUE2,
    #[doc = "Blanking on a HIGH to LOW transition"]
    VALUE3,
    #[doc = "Blanking on both transitions"]
    VALUE4,
}
impl BLMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BLMCW::VALUE1 => 0,
            BLMCW::VALUE2 => 1,
            BLMCW::VALUE3 => 2,
            BLMCW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLMCW<'a> {
    w: &'a mut W,
}
impl<'a> _BLMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLMCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Blanking disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BLMCW::VALUE1)
    }
    #[doc = "Blanking on a LOW to HIGH transition"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BLMCW::VALUE2)
    }
    #[doc = "Blanking on a HIGH to LOW transition"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BLMCW::VALUE3)
    }
    #[doc = "Blanking on both transitions"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BLMCW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EBEW<'a> {
    w: &'a mut W,
}
impl<'a> _EBEW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COFE`"]
pub enum COFEW {
    #[doc = "Filtering stage disabled"]
    VALUE1,
    #[doc = "Filtering stage enabled"]
    VALUE2,
}
impl COFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COFEW::VALUE1 => false,
            COFEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COFEW<'a> {
    w: &'a mut W,
}
impl<'a> _COFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filtering stage disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(COFEW::VALUE1)
    }
    #[doc = "Filtering stage enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(COFEW::VALUE2)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COFM`"]
pub enum COFMW {
    #[doc = "Comparator Output needs to be stable for 2 clock cycles"]
    VALUE1,
    #[doc = "Comparator Output needs to be stable for 3 clock cycles"]
    VALUE2,
    #[doc = "Comparator Output needs to be stable for 4 clock cycles"]
    VALUE3,
    #[doc = "Comparator Output needs to be stable for 5 clock cycles"]
    VALUE4,
    #[doc = "Comparator Output needs to be stable for 14 clock cycles"]
    VALUE5,
    #[doc = "Comparator Output needs to be stable for 15 clock cycles"]
    VALUE6,
    #[doc = "Comparator Output needs to be stable for 16 clock cycles"]
    VALUE7,
    #[doc = "Comparator Output needs to be stable for 32 clock cycles"]
    VALUE8,
}
impl COFMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COFMW::VALUE1 => 0,
            COFMW::VALUE2 => 1,
            COFMW::VALUE3 => 2,
            COFMW::VALUE4 => 3,
            COFMW::VALUE5 => 12,
            COFMW::VALUE6 => 13,
            COFMW::VALUE7 => 14,
            COFMW::VALUE8 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COFMW<'a> {
    w: &'a mut W,
}
impl<'a> _COFMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COFMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Comparator Output needs to be stable for 2 clock cycles"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(COFMW::VALUE1)
    }
    #[doc = "Comparator Output needs to be stable for 3 clock cycles"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(COFMW::VALUE2)
    }
    #[doc = "Comparator Output needs to be stable for 4 clock cycles"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(COFMW::VALUE3)
    }
    #[doc = "Comparator Output needs to be stable for 5 clock cycles"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(COFMW::VALUE4)
    }
    #[doc = "Comparator Output needs to be stable for 14 clock cycles"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(COFMW::VALUE5)
    }
    #[doc = "Comparator Output needs to be stable for 15 clock cycles"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(COFMW::VALUE6)
    }
    #[doc = "Comparator Output needs to be stable for 16 clock cycles"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(COFMW::VALUE7)
    }
    #[doc = "Comparator Output needs to be stable for 32 clock cycles"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(COFMW::VALUE8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COFC`"]
pub enum COFCW {
    #[doc = "Filtering is always done if enabled"]
    VALUE1,
    #[doc = "Filtering is only done when CSGyDSV1 value is currently fed to the DAC"]
    VALUE2,
    #[doc = "Filtering is only done when the CSGyDSV2 value is currently fed to the DAC"]
    VALUE3,
}
impl COFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COFCW::VALUE1 => 0,
            COFCW::VALUE2 => 1,
            COFCW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COFCW<'a> {
    w: &'a mut W,
}
impl<'a> _COFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COFCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Filtering is always done if enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(COFCW::VALUE1)
    }
    #[doc = "Filtering is only done when CSGyDSV1 value is currently fed to the DAC"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(COFCW::VALUE2)
    }
    #[doc = "Filtering is only done when the CSGyDSV2 value is currently fed to the DAC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(COFCW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:3 - External blanking trigger selector"]
    #[inline]
    pub fn ibs(&self) -> IBSR {
        IBSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Inverting comparator input selector"]
    #[inline]
    pub fn imcs(&self) -> IMCSR {
        IMCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - Comparator input switching configuration"]
    #[inline]
    pub fn imcc(&self) -> IMCCR {
        IMCCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - External triggered switch enable"]
    #[inline]
    pub fn ese(&self) -> ESER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ESER { bits }
    }
    #[doc = "Bit 12 - Comparator output inversion enable"]
    #[inline]
    pub fn oie(&self) -> OIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OIER { bits }
    }
    #[doc = "Bit 13 - Comparator output synchronization enable"]
    #[inline]
    pub fn ose(&self) -> OSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSER { bits }
    }
    #[doc = "Bits 14:15 - Blanking mode"]
    #[inline]
    pub fn blmc(&self) -> BLMCR {
        BLMCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - External blanking trigger enabled"]
    #[inline]
    pub fn ebe(&self) -> EBER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EBER { bits }
    }
    #[doc = "Bit 17 - Comparator output filter enable"]
    #[inline]
    pub fn cofe(&self) -> COFER {
        COFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:21 - Comparator output filter window"]
    #[inline]
    pub fn cofm(&self) -> COFMR {
        COFMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Comparator output filter control"]
    #[inline]
    pub fn cofc(&self) -> COFCR {
        COFCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:3 - External blanking trigger selector"]
    #[inline]
    pub fn ibs(&mut self) -> _IBSW {
        _IBSW { w: self }
    }
    #[doc = "Bit 8 - Inverting comparator input selector"]
    #[inline]
    pub fn imcs(&mut self) -> _IMCSW {
        _IMCSW { w: self }
    }
    #[doc = "Bits 9:10 - Comparator input switching configuration"]
    #[inline]
    pub fn imcc(&mut self) -> _IMCCW {
        _IMCCW { w: self }
    }
    #[doc = "Bit 11 - External triggered switch enable"]
    #[inline]
    pub fn ese(&mut self) -> _ESEW {
        _ESEW { w: self }
    }
    #[doc = "Bit 12 - Comparator output inversion enable"]
    #[inline]
    pub fn oie(&mut self) -> _OIEW {
        _OIEW { w: self }
    }
    #[doc = "Bit 13 - Comparator output synchronization enable"]
    #[inline]
    pub fn ose(&mut self) -> _OSEW {
        _OSEW { w: self }
    }
    #[doc = "Bits 14:15 - Blanking mode"]
    #[inline]
    pub fn blmc(&mut self) -> _BLMCW {
        _BLMCW { w: self }
    }
    #[doc = "Bit 16 - External blanking trigger enabled"]
    #[inline]
    pub fn ebe(&mut self) -> _EBEW {
        _EBEW { w: self }
    }
    #[doc = "Bit 17 - Comparator output filter enable"]
    #[inline]
    pub fn cofe(&mut self) -> _COFEW {
        _COFEW { w: self }
    }
    #[doc = "Bits 18:21 - Comparator output filter window"]
    #[inline]
    pub fn cofm(&mut self) -> _COFMW {
        _COFMW { w: self }
    }
    #[doc = "Bits 24:25 - Comparator output filter control"]
    #[inline]
    pub fn cofc(&mut self) -> _COFCW {
        _COFCW { w: self }
    }
}
