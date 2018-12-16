#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMTSR {
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
#[doc = "Possible values of the field `MTENPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENPSR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENPSR {
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
            MTENPSR::VALUE1 => false,
            MTENPSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTENPSR {
        match value {
            false => MTENPSR::VALUE1,
            true => MTENPSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTENPSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTENPSR::VALUE2
    }
}
#[doc = "Possible values of the field `MTENDS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENDS1R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENDS1R {
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
            MTENDS1R::VALUE1 => false,
            MTENDS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTENDS1R {
        match value {
            false => MTENDS1R::VALUE1,
            true => MTENDS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTENDS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTENDS1R::VALUE2
    }
}
#[doc = "Possible values of the field `MTEU0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU0R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU0R {
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
            MTEU0R::VALUE1 => false,
            MTEU0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTEU0R {
        match value {
            false => MTEU0R::VALUE1,
            true => MTEU0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTEU0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTEU0R::VALUE2
    }
}
#[doc = "Possible values of the field `MTEU1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU1R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU1R {
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
            MTEU1R::VALUE1 => false,
            MTEU1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTEU1R {
        match value {
            false => MTEU1R::VALUE1,
            true => MTEU1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTEU1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTEU1R::VALUE2
    }
}
#[doc = "Possible values of the field `MTEMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEMCR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEMCR {
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
            MTEMCR::VALUE1 => false,
            MTEMCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTEMCR {
        match value {
            false => MTEMCR::VALUE1,
            true => MTEMCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTEMCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTEMCR::VALUE2
    }
}
#[doc = "Possible values of the field `MTEPPRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEPPRFR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEPPRFR {
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
            MTEPPRFR::VALUE1 => false,
            MTEPPRFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTEPPRFR {
        match value {
            false => MTEPPRFR::VALUE1,
            true => MTEPPRFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTEPPRFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTEPPRFR::VALUE2
    }
}
#[doc = "Possible values of the field `MTUSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTUSBR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTUSBR {
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
            MTUSBR::VALUE1 => false,
            MTUSBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTUSBR {
        match value {
            false => MTUSBR::VALUE1,
            true => MTUSBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTUSBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTUSBR::VALUE2
    }
}
#[doc = "Values that can be written to the field `MTENPS`"]
pub enum MTENPSW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTENPSW::VALUE1 => false,
            MTENPSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTENPSW<'a> {
    w: &'a mut W,
}
impl<'a> _MTENPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTENPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTENPSW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTENPSW::VALUE2)
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
#[doc = "Values that can be written to the field `MTENDS1`"]
pub enum MTENDS1W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENDS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTENDS1W::VALUE1 => false,
            MTENDS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTENDS1W<'a> {
    w: &'a mut W,
}
impl<'a> _MTENDS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTENDS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTENDS1W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTENDS1W::VALUE2)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTEU0`"]
pub enum MTEU0W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTEU0W::VALUE1 => false,
            MTEU0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTEU0W<'a> {
    w: &'a mut W,
}
impl<'a> _MTEU0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTEU0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEU0W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEU0W::VALUE2)
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
#[doc = "Values that can be written to the field `MTEU1`"]
pub enum MTEU1W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTEU1W::VALUE1 => false,
            MTEU1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTEU1W<'a> {
    w: &'a mut W,
}
impl<'a> _MTEU1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTEU1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEU1W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEU1W::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTEMC`"]
pub enum MTEMCW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTEMCW::VALUE1 => false,
            MTEMCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTEMCW<'a> {
    w: &'a mut W,
}
impl<'a> _MTEMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTEMCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEMCW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEMCW::VALUE2)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTEPPRF`"]
pub enum MTEPPRFW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEPPRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTEPPRFW::VALUE1 => false,
            MTEPPRFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTEPPRFW<'a> {
    w: &'a mut W,
}
impl<'a> _MTEPPRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTEPPRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEPPRFW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEPPRFW::VALUE2)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTUSB`"]
pub enum MTUSBW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTUSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTUSBW::VALUE1 => false,
            MTUSBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTUSBW<'a> {
    w: &'a mut W,
}
impl<'a> _MTUSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTUSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTUSBW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTUSBW::VALUE2)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline]
    pub fn mtenps(&self) -> MTENPSR {
        MTENPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline]
    pub fn mtends1(&self) -> MTENDS1R {
        MTENDS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline]
    pub fn mteu0(&self) -> MTEU0R {
        MTEU0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline]
    pub fn mteu1(&self) -> MTEU1R {
        MTEU1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline]
    pub fn mtemc(&self) -> MTEMCR {
        MTEMCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline]
    pub fn mtepprf(&self) -> MTEPPRFR {
        MTEPPRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline]
    pub fn mtusb(&self) -> MTUSBR {
        MTUSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline]
    pub fn mtenps(&mut self) -> _MTENPSW {
        _MTENPSW { w: self }
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline]
    pub fn mtends1(&mut self) -> _MTENDS1W {
        _MTENDS1W { w: self }
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline]
    pub fn mteu0(&mut self) -> _MTEU0W {
        _MTEU0W { w: self }
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline]
    pub fn mteu1(&mut self) -> _MTEU1W {
        _MTEU1W { w: self }
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline]
    pub fn mtemc(&mut self) -> _MTEMCW {
        _MTEMCW { w: self }
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline]
    pub fn mtepprf(&mut self) -> _MTEPPRFW {
        _MTEPPRFW { w: self }
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline]
    pub fn mtusb(&mut self) -> _MTUSBW {
        _MTUSBW { w: self }
    }
}
