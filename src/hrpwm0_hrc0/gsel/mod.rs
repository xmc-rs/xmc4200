#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GSEL {
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
#[doc = "Possible values of the field `C0SS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0SSR {
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    VALUE1,
    #[doc = "CMP output of CSG1 unit can be used as set source for the output latch"]
    VALUE2,
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl C0SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C0SSR::VALUE1 => 0,
            C0SSR::VALUE2 => 1,
            C0SSR::VALUE3 => 2,
            C0SSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C0SSR {
        match value {
            0 => C0SSR::VALUE1,
            1 => C0SSR::VALUE2,
            2 => C0SSR::VALUE3,
            i => C0SSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C0SSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C0SSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == C0SSR::VALUE3
    }
}
#[doc = "Possible values of the field `C0CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0CSR {
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    VALUE1,
    #[doc = "CMP output of CSG1 unit can be used as clear source for the output latch"]
    VALUE2,
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl C0CSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C0CSR::VALUE1 => 0,
            C0CSR::VALUE2 => 1,
            C0CSR::VALUE3 => 2,
            C0CSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C0CSR {
        match value {
            0 => C0CSR::VALUE1,
            1 => C0CSR::VALUE2,
            2 => C0CSR::VALUE3,
            i => C0CSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C0CSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C0CSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == C0CSR::VALUE3
    }
}
#[doc = "Possible values of the field `S0M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0MR {
    #[doc = "Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1,
    #[doc = "Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl S0MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S0MR::VALUE1 => 0,
            S0MR::VALUE2 => 1,
            S0MR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S0MR {
        match value {
            0 => S0MR::VALUE1,
            1 => S0MR::VALUE2,
            i => S0MR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S0MR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S0MR::VALUE2
    }
}
#[doc = "Possible values of the field `C0M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0MR {
    #[doc = "Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1,
    #[doc = "Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl C0MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C0MR::VALUE1 => 0,
            C0MR::VALUE2 => 1,
            C0MR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C0MR {
        match value {
            0 => C0MR::VALUE1,
            1 => C0MR::VALUE2,
            i => C0MR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C0MR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C0MR::VALUE2
    }
}
#[doc = "Possible values of the field `S0ES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0ESR {
    #[doc = "Generation of the set signal is disabled"]
    VALUE1,
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2,
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3,
    #[doc = "Set signal is generated on both transitions of the selected input"]
    VALUE4,
}
impl S0ESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S0ESR::VALUE1 => 0,
            S0ESR::VALUE2 => 1,
            S0ESR::VALUE3 => 2,
            S0ESR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S0ESR {
        match value {
            0 => S0ESR::VALUE1,
            1 => S0ESR::VALUE2,
            2 => S0ESR::VALUE3,
            3 => S0ESR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S0ESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S0ESR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == S0ESR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == S0ESR::VALUE4
    }
}
#[doc = "Possible values of the field `C0ES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0ESR {
    #[doc = "Generation of the clear signal is disabled"]
    VALUE1,
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2,
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3,
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    VALUE4,
}
impl C0ESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C0ESR::VALUE1 => 0,
            C0ESR::VALUE2 => 1,
            C0ESR::VALUE3 => 2,
            C0ESR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C0ESR {
        match value {
            0 => C0ESR::VALUE1,
            1 => C0ESR::VALUE2,
            2 => C0ESR::VALUE3,
            3 => C0ESR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C0ESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C0ESR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == C0ESR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == C0ESR::VALUE4
    }
}
#[doc = "Possible values of the field `C1SS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1SSR {
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    VALUE1,
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE2,
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl C1SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C1SSR::VALUE1 => 0,
            C1SSR::VALUE2 => 1,
            C1SSR::VALUE3 => 2,
            C1SSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C1SSR {
        match value {
            0 => C1SSR::VALUE1,
            1 => C1SSR::VALUE2,
            2 => C1SSR::VALUE3,
            i => C1SSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C1SSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C1SSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == C1SSR::VALUE3
    }
}
#[doc = "Possible values of the field `C1CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1CSR {
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    VALUE1,
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE2,
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl C1CSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C1CSR::VALUE1 => 0,
            C1CSR::VALUE2 => 1,
            C1CSR::VALUE3 => 2,
            C1CSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C1CSR {
        match value {
            0 => C1CSR::VALUE1,
            1 => C1CSR::VALUE2,
            2 => C1CSR::VALUE3,
            i => C1CSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C1CSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C1CSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == C1CSR::VALUE3
    }
}
#[doc = "Possible values of the field `S1M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1MR {
    #[doc = "Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1,
    #[doc = "Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl S1MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S1MR::VALUE1 => 0,
            S1MR::VALUE2 => 1,
            S1MR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S1MR {
        match value {
            0 => S1MR::VALUE1,
            1 => S1MR::VALUE2,
            i => S1MR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S1MR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S1MR::VALUE2
    }
}
#[doc = "Possible values of the field `C1M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1MR {
    #[doc = "Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1,
    #[doc = "Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl C1MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C1MR::VALUE1 => 0,
            C1MR::VALUE2 => 1,
            C1MR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C1MR {
        match value {
            0 => C1MR::VALUE1,
            1 => C1MR::VALUE2,
            i => C1MR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C1MR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C1MR::VALUE2
    }
}
#[doc = "Possible values of the field `S1ES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1ESR {
    #[doc = "Generation of the set signal is disabled"]
    VALUE1,
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2,
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3,
    #[doc = "Set signal is generated on both transitions of the selected input"]
    VALUE4,
}
impl S1ESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S1ESR::VALUE1 => 0,
            S1ESR::VALUE2 => 1,
            S1ESR::VALUE3 => 2,
            S1ESR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S1ESR {
        match value {
            0 => S1ESR::VALUE1,
            1 => S1ESR::VALUE2,
            2 => S1ESR::VALUE3,
            3 => S1ESR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S1ESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S1ESR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == S1ESR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == S1ESR::VALUE4
    }
}
#[doc = "Possible values of the field `C1ES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1ESR {
    #[doc = "Generation of the clear signal is disabled"]
    VALUE1,
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2,
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3,
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    VALUE4,
}
impl C1ESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C1ESR::VALUE1 => 0,
            C1ESR::VALUE2 => 1,
            C1ESR::VALUE3 => 2,
            C1ESR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C1ESR {
        match value {
            0 => C1ESR::VALUE1,
            1 => C1ESR::VALUE2,
            2 => C1ESR::VALUE3,
            3 => C1ESR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == C1ESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == C1ESR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == C1ESR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == C1ESR::VALUE4
    }
}
#[doc = "Values that can be written to the field `C0SS`"]
pub enum C0SSW {
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    VALUE1,
    #[doc = "CMP output of CSG1 unit can be used as set source for the output latch"]
    VALUE2,
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE3,
}
impl C0SSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C0SSW::VALUE1 => 0,
            C0SSW::VALUE2 => 1,
            C0SSW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C0SSW<'a> {
    w: &'a mut W,
}
impl<'a> _C0SSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C0SSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0SSW::VALUE1)
    }
    #[doc = "CMP output of CSG1 unit can be used as set source for the output latch"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0SSW::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(C0SSW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C0CS`"]
pub enum C0CSW {
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    VALUE1,
    #[doc = "CMP output of CSG1 unit can be used as clear source for the output latch"]
    VALUE2,
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE3,
}
impl C0CSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C0CSW::VALUE1 => 0,
            C0CSW::VALUE2 => 1,
            C0CSW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C0CSW<'a> {
    w: &'a mut W,
}
impl<'a> _C0CSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C0CSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0CSW::VALUE1)
    }
    #[doc = "CMP output of CSG1 unit can be used as clear source for the output latch"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0CSW::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(C0CSW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S0M`"]
pub enum S0MW {
    #[doc = "Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1,
    #[doc = "Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    VALUE2,
}
impl S0MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S0MW::VALUE1 => 0,
            S0MW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0MW<'a> {
    w: &'a mut W,
}
impl<'a> _S0MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0MW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(S0MW::VALUE1)
    }
    #[doc = "Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(S0MW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C0M`"]
pub enum C0MW {
    #[doc = "Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1,
    #[doc = "Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    VALUE2,
}
impl C0MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C0MW::VALUE1 => 0,
            C0MW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C0MW<'a> {
    w: &'a mut W,
}
impl<'a> _C0MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C0MW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0MW::VALUE1)
    }
    #[doc = "Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0MW::VALUE2)
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
#[doc = "Values that can be written to the field `S0ES`"]
pub enum S0ESW {
    #[doc = "Generation of the set signal is disabled"]
    VALUE1,
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2,
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3,
    #[doc = "Set signal is generated on both transitions of the selected input"]
    VALUE4,
}
impl S0ESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S0ESW::VALUE1 => 0,
            S0ESW::VALUE2 => 1,
            S0ESW::VALUE3 => 2,
            S0ESW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0ESW<'a> {
    w: &'a mut W,
}
impl<'a> _S0ESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0ESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generation of the set signal is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(S0ESW::VALUE1)
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(S0ESW::VALUE2)
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(S0ESW::VALUE3)
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(S0ESW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C0ES`"]
pub enum C0ESW {
    #[doc = "Generation of the clear signal is disabled"]
    VALUE1,
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2,
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3,
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    VALUE4,
}
impl C0ESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C0ESW::VALUE1 => 0,
            C0ESW::VALUE2 => 1,
            C0ESW::VALUE3 => 2,
            C0ESW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C0ESW<'a> {
    w: &'a mut W,
}
impl<'a> _C0ESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C0ESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generation of the clear signal is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0ESW::VALUE1)
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0ESW::VALUE2)
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(C0ESW::VALUE3)
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(C0ESW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C1SS`"]
pub enum C1SSW {
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    VALUE1,
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE2,
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE3,
}
impl C1SSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C1SSW::VALUE1 => 0,
            C1SSW::VALUE2 => 1,
            C1SSW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C1SSW<'a> {
    w: &'a mut W,
}
impl<'a> _C1SSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C1SSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1SSW::VALUE1)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1SSW::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(C1SSW::VALUE3)
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
#[doc = "Values that can be written to the field `C1CS`"]
pub enum C1CSW {
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    VALUE1,
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE2,
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE3,
}
impl C1CSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C1CSW::VALUE1 => 0,
            C1CSW::VALUE2 => 1,
            C1CSW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C1CSW<'a> {
    w: &'a mut W,
}
impl<'a> _C1CSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C1CSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1CSW::VALUE1)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1CSW::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(C1CSW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S1M`"]
pub enum S1MW {
    #[doc = "Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1,
    #[doc = "Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    VALUE2,
}
impl S1MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S1MW::VALUE1 => 0,
            S1MW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1MW<'a> {
    w: &'a mut W,
}
impl<'a> _S1MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1MW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(S1MW::VALUE1)
    }
    #[doc = "Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(S1MW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C1M`"]
pub enum C1MW {
    #[doc = "Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1,
    #[doc = "Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    VALUE2,
}
impl C1MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C1MW::VALUE1 => 0,
            C1MW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C1MW<'a> {
    w: &'a mut W,
}
impl<'a> _C1MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C1MW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1MW::VALUE1)
    }
    #[doc = "Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1MW::VALUE2)
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
#[doc = "Values that can be written to the field `S1ES`"]
pub enum S1ESW {
    #[doc = "Generation of the set signal is disabled"]
    VALUE1,
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2,
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3,
    #[doc = "Set signal is generated on both transitions of the selected input"]
    VALUE4,
}
impl S1ESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S1ESW::VALUE1 => 0,
            S1ESW::VALUE2 => 1,
            S1ESW::VALUE3 => 2,
            S1ESW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1ESW<'a> {
    w: &'a mut W,
}
impl<'a> _S1ESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1ESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generation of the set signal is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(S1ESW::VALUE1)
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(S1ESW::VALUE2)
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(S1ESW::VALUE3)
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(S1ESW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C1ES`"]
pub enum C1ESW {
    #[doc = "Generation of the clear signal is disabled"]
    VALUE1,
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2,
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3,
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    VALUE4,
}
impl C1ESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C1ESW::VALUE1 => 0,
            C1ESW::VALUE2 => 1,
            C1ESW::VALUE3 => 2,
            C1ESW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C1ESW<'a> {
    w: &'a mut W,
}
impl<'a> _C1ESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C1ESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generation of the clear signal is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1ESW::VALUE1)
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1ESW::VALUE2)
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(C1ESW::VALUE3)
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(C1ESW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline]
    pub fn c0ss(&self) -> C0SSR {
        C0SSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline]
    pub fn c0cs(&self) -> C0CSR {
        C0CSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline]
    pub fn s0m(&self) -> S0MR {
        S0MR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline]
    pub fn c0m(&self) -> C0MR {
        C0MR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline]
    pub fn s0es(&self) -> S0ESR {
        S0ESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline]
    pub fn c0es(&self) -> C0ESR {
        C0ESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline]
    pub fn c1ss(&self) -> C1SSR {
        C1SSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline]
    pub fn c1cs(&self) -> C1CSR {
        C1CSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline]
    pub fn s1m(&self) -> S1MR {
        S1MR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline]
    pub fn c1m(&self) -> C1MR {
        C1MR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline]
    pub fn s1es(&self) -> S1ESR {
        S1ESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline]
    pub fn c1es(&self) -> C1ESR {
        C1ESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline]
    pub fn c0ss(&mut self) -> _C0SSW {
        _C0SSW { w: self }
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline]
    pub fn c0cs(&mut self) -> _C0CSW {
        _C0CSW { w: self }
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline]
    pub fn s0m(&mut self) -> _S0MW {
        _S0MW { w: self }
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline]
    pub fn c0m(&mut self) -> _C0MW {
        _C0MW { w: self }
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline]
    pub fn s0es(&mut self) -> _S0ESW {
        _S0ESW { w: self }
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline]
    pub fn c0es(&mut self) -> _C0ESW {
        _C0ESW { w: self }
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline]
    pub fn c1ss(&mut self) -> _C1SSW {
        _C1SSW { w: self }
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline]
    pub fn c1cs(&mut self) -> _C1CSW {
        _C1CSW { w: self }
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline]
    pub fn s1m(&mut self) -> _S1MW {
        _S1MW { w: self }
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline]
    pub fn c1m(&mut self) -> _C1MW {
        _C1MW { w: self }
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline]
    pub fn s1es(&mut self) -> _S1ESW {
        _S1ESW { w: self }
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline]
    pub fn c1es(&mut self) -> _C1ESW {
        _C1ESW { w: self }
    }
}
