#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLC {
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
#[doc = "Possible values of the field `IPLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPLSR {
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
impl IPLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IPLSR::VALUE1 => 0,
            IPLSR::VALUE2 => 1,
            IPLSR::VALUE3 => 2,
            IPLSR::VALUE4 => 3,
            IPLSR::VALUE5 => 4,
            IPLSR::VALUE6 => 5,
            IPLSR::VALUE7 => 6,
            IPLSR::VALUE8 => 7,
            IPLSR::VALUE9 => 8,
            IPLSR::VALUE10 => 9,
            IPLSR::VALUE11 => 10,
            IPLSR::VALUE12 => 11,
            IPLSR::VALUE13 => 12,
            IPLSR::VALUE14 => 13,
            IPLSR::VALUE15 => 14,
            IPLSR::VALUE16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IPLSR {
        match value {
            0 => IPLSR::VALUE1,
            1 => IPLSR::VALUE2,
            2 => IPLSR::VALUE3,
            3 => IPLSR::VALUE4,
            4 => IPLSR::VALUE5,
            5 => IPLSR::VALUE6,
            6 => IPLSR::VALUE7,
            7 => IPLSR::VALUE8,
            8 => IPLSR::VALUE9,
            9 => IPLSR::VALUE10,
            10 => IPLSR::VALUE11,
            11 => IPLSR::VALUE12,
            12 => IPLSR::VALUE13,
            13 => IPLSR::VALUE14,
            14 => IPLSR::VALUE15,
            15 => IPLSR::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IPLSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IPLSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == IPLSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == IPLSR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == IPLSR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == IPLSR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == IPLSR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == IPLSR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == IPLSR::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == IPLSR::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == IPLSR::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == IPLSR::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == IPLSR::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == IPLSR::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == IPLSR::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == IPLSR::VALUE16
    }
}
#[doc = "Possible values of the field `PLCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLCLR {
    #[doc = "Clamping control disabled"]
    VALUE1,
    #[doc = "Output is set to clamped level when the control signal is HIGH"]
    VALUE2,
    #[doc = "Output is set to clamped level when the control signal is LOW"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLCLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLCLR::VALUE1 => 0,
            PLCLR::VALUE2 => 1,
            PLCLR::VALUE3 => 2,
            PLCLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLCLR {
        match value {
            0 => PLCLR::VALUE1,
            1 => PLCLR::VALUE2,
            2 => PLCLR::VALUE3,
            i => PLCLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLCLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLCLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PLCLR::VALUE3
    }
}
#[doc = "Possible values of the field `PSL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSLR {
    #[doc = "Output clamped level is LOW"]
    VALUE1,
    #[doc = "Output clamped level is HIGH"]
    VALUE2,
}
impl PSLR {
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
            PSLR::VALUE1 => false,
            PSLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSLR {
        match value {
            false => PSLR::VALUE1,
            true => PSLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSLR::VALUE2
    }
}
#[doc = "Possible values of the field `PLSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLSWR {
    #[doc = "External signal and SW can remove the output from the clamped state"]
    VALUE1,
    #[doc = "Only SW can remove the output from the clamped state"]
    VALUE2,
}
impl PLSWR {
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
            PLSWR::VALUE1 => false,
            PLSWR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLSWR {
        match value {
            false => PLSWR::VALUE1,
            true => PLSWR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLSWR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLSWR::VALUE2
    }
}
#[doc = "Possible values of the field `PLEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLECR {
    #[doc = "Passive level is entered immediately"]
    VALUE1,
    #[doc = "Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2,
    #[doc = "Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLECR::VALUE1 => 0,
            PLECR::VALUE2 => 1,
            PLECR::VALUE3 => 2,
            PLECR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLECR {
        match value {
            0 => PLECR::VALUE1,
            1 => PLECR::VALUE2,
            2 => PLECR::VALUE3,
            i => PLECR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLECR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLECR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PLECR::VALUE3
    }
}
#[doc = "Possible values of the field `PLXC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLXCR {
    #[doc = "Passive level is exit immediately"]
    VALUE1,
    #[doc = "Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2,
    #[doc = "Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLXCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLXCR::VALUE1 => 0,
            PLXCR::VALUE2 => 1,
            PLXCR::VALUE3 => 2,
            PLXCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLXCR {
        match value {
            0 => PLXCR::VALUE1,
            1 => PLXCR::VALUE2,
            2 => PLXCR::VALUE3,
            i => PLXCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLXCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLXCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PLXCR::VALUE3
    }
}
#[doc = "Values that can be written to the field `IPLS`"]
pub enum IPLSW {
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
impl IPLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IPLSW::VALUE1 => 0,
            IPLSW::VALUE2 => 1,
            IPLSW::VALUE3 => 2,
            IPLSW::VALUE4 => 3,
            IPLSW::VALUE5 => 4,
            IPLSW::VALUE6 => 5,
            IPLSW::VALUE7 => 6,
            IPLSW::VALUE8 => 7,
            IPLSW::VALUE9 => 8,
            IPLSW::VALUE10 => 9,
            IPLSW::VALUE11 => 10,
            IPLSW::VALUE12 => 11,
            IPLSW::VALUE13 => 12,
            IPLSW::VALUE14 => 13,
            IPLSW::VALUE15 => 14,
            IPLSW::VALUE16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPLSW<'a> {
    w: &'a mut W,
}
impl<'a> _IPLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPLSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "HRPWMx.BLyA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPLSW::VALUE1)
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPLSW::VALUE2)
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(IPLSW::VALUE3)
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(IPLSW::VALUE4)
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(IPLSW::VALUE5)
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(IPLSW::VALUE6)
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(IPLSW::VALUE7)
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(IPLSW::VALUE8)
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(IPLSW::VALUE9)
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(IPLSW::VALUE10)
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(IPLSW::VALUE11)
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(IPLSW::VALUE12)
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(IPLSW::VALUE13)
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(IPLSW::VALUE14)
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(IPLSW::VALUE15)
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(IPLSW::VALUE16)
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
#[doc = "Values that can be written to the field `PLCL`"]
pub enum PLCLW {
    #[doc = "Clamping control disabled"]
    VALUE1,
    #[doc = "Output is set to clamped level when the control signal is HIGH"]
    VALUE2,
    #[doc = "Output is set to clamped level when the control signal is LOW"]
    VALUE3,
}
impl PLCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLCLW::VALUE1 => 0,
            PLCLW::VALUE2 => 1,
            PLCLW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLCLW<'a> {
    w: &'a mut W,
}
impl<'a> _PLCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLCLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clamping control disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLCLW::VALUE1)
    }
    #[doc = "Output is set to clamped level when the control signal is HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLCLW::VALUE2)
    }
    #[doc = "Output is set to clamped level when the control signal is LOW"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLCLW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSL`"]
pub enum PSLW {
    #[doc = "Output clamped level is LOW"]
    VALUE1,
    #[doc = "Output clamped level is HIGH"]
    VALUE2,
}
impl PSLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSLW::VALUE1 => false,
            PSLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSLW<'a> {
    w: &'a mut W,
}
impl<'a> _PSLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output clamped level is LOW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSLW::VALUE1)
    }
    #[doc = "Output clamped level is HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSLW::VALUE2)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLSW`"]
pub enum PLSWW {
    #[doc = "External signal and SW can remove the output from the clamped state"]
    VALUE1,
    #[doc = "Only SW can remove the output from the clamped state"]
    VALUE2,
}
impl PLSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLSWW::VALUE1 => false,
            PLSWW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLSWW<'a> {
    w: &'a mut W,
}
impl<'a> _PLSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External signal and SW can remove the output from the clamped state"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLSWW::VALUE1)
    }
    #[doc = "Only SW can remove the output from the clamped state"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLSWW::VALUE2)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLEC`"]
pub enum PLECW {
    #[doc = "Passive level is entered immediately"]
    VALUE1,
    #[doc = "Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2,
    #[doc = "Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3,
}
impl PLECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLECW::VALUE1 => 0,
            PLECW::VALUE2 => 1,
            PLECW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLECW<'a> {
    w: &'a mut W,
}
impl<'a> _PLECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLECW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Passive level is entered immediately"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLECW::VALUE1)
    }
    #[doc = "Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLECW::VALUE2)
    }
    #[doc = "Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLECW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLXC`"]
pub enum PLXCW {
    #[doc = "Passive level is exit immediately"]
    VALUE1,
    #[doc = "Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2,
    #[doc = "Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3,
}
impl PLXCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLXCW::VALUE1 => 0,
            PLXCW::VALUE2 => 1,
            PLXCW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLXCW<'a> {
    w: &'a mut W,
}
impl<'a> _PLXCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLXCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Passive level is exit immediately"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLXCW::VALUE1)
    }
    #[doc = "Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLXCW::VALUE2)
    }
    #[doc = "Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLXCW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:3 - Clamping control signal selector"]
    #[inline]
    pub fn ipls(&self) -> IPLSR {
        IPLSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Clamping control signal level selection"]
    #[inline]
    pub fn plcl(&self) -> PLCLR {
        PLCLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Output passive level value"]
    #[inline]
    pub fn psl(&self) -> PSLR {
        PSLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Clamped state exit SW configuration"]
    #[inline]
    pub fn plsw(&self) -> PLSWR {
        PLSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Passive level enter configuration"]
    #[inline]
    pub fn plec(&self) -> PLECR {
        PLECR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Passive level exit configuration"]
    #[inline]
    pub fn plxc(&self) -> PLXCR {
        PLXCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:3 - Clamping control signal selector"]
    #[inline]
    pub fn ipls(&mut self) -> _IPLSW {
        _IPLSW { w: self }
    }
    #[doc = "Bits 8:9 - Clamping control signal level selection"]
    #[inline]
    pub fn plcl(&mut self) -> _PLCLW {
        _PLCLW { w: self }
    }
    #[doc = "Bit 10 - Output passive level value"]
    #[inline]
    pub fn psl(&mut self) -> _PSLW {
        _PSLW { w: self }
    }
    #[doc = "Bit 11 - Clamped state exit SW configuration"]
    #[inline]
    pub fn plsw(&mut self) -> _PLSWW {
        _PLSWW { w: self }
    }
    #[doc = "Bits 12:13 - Passive level enter configuration"]
    #[inline]
    pub fn plec(&mut self) -> _PLECW {
        _PLECW { w: self }
    }
    #[doc = "Bits 14:15 - Passive level exit configuration"]
    #[inline]
    pub fn plxc(&mut self) -> _PLXCW {
        _PLXCW { w: self }
    }
}
