#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GUSBCFG {
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
#[doc = r" Value of the field"]
pub struct TOUTCALR {
    bits: u8,
}
impl TOUTCALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PHYSel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYSELR {
    #[doc = "USB 1.1 full-speed serial transceiver"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PHYSELR {
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
            PHYSELR::VALUE2 => true,
            PHYSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHYSELR {
        match value {
            true => PHYSELR::VALUE2,
            i => PHYSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PHYSELR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct USBTRDTIMR {
    bits: u8,
}
impl USBTRDTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TxEndDelay`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENDDELAYR {
    #[doc = "Normal mode"]
    VALUE1,
    #[doc = "Introduce Tx end delay timers"]
    VALUE2,
}
impl TXENDDELAYR {
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
            TXENDDELAYR::VALUE1 => false,
            TXENDDELAYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXENDDELAYR {
        match value {
            false => TXENDDELAYR::VALUE1,
            true => TXENDDELAYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXENDDELAYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXENDDELAYR::VALUE2
    }
}
#[doc = "Possible values of the field `ForceDevMode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEDEVMODER {
    #[doc = "Normal Mode"]
    VALUE1,
    #[doc = "Force Device Mode"]
    VALUE2,
}
impl FORCEDEVMODER {
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
            FORCEDEVMODER::VALUE1 => false,
            FORCEDEVMODER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCEDEVMODER {
        match value {
            false => FORCEDEVMODER::VALUE1,
            true => FORCEDEVMODER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FORCEDEVMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FORCEDEVMODER::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CTPR {
    bits: bool,
}
impl CTPR {
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
#[doc = r" Proxy"]
pub struct _TOUTCALW<'a> {
    w: &'a mut W,
}
impl<'a> _TOUTCALW<'a> {
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
#[doc = r" Proxy"]
pub struct _USBTRDTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _USBTRDTIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TxEndDelay`"]
pub enum TXENDDELAYW {
    #[doc = "Normal mode"]
    VALUE1,
    #[doc = "Introduce Tx end delay timers"]
    VALUE2,
}
impl TXENDDELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXENDDELAYW::VALUE1 => false,
            TXENDDELAYW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXENDDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXENDDELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXENDDELAYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXENDDELAYW::VALUE1)
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXENDDELAYW::VALUE2)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ForceDevMode`"]
pub enum FORCEDEVMODEW {
    #[doc = "Normal Mode"]
    VALUE1,
    #[doc = "Force Device Mode"]
    VALUE2,
}
impl FORCEDEVMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCEDEVMODEW::VALUE1 => false,
            FORCEDEVMODEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCEDEVMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEDEVMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCEDEVMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FORCEDEVMODEW::VALUE1)
    }
    #[doc = "Force Device Mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FORCEDEVMODEW::VALUE2)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTPW<'a> {
    w: &'a mut W,
}
impl<'a> _CTPW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline]
    pub fn tout_cal(&self) -> TOUTCALR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TOUTCALR { bits }
    }
    #[doc = "Bit 6 - USB 1.1 Full-Speed Serial Transceiver Select"]
    #[inline]
    pub fn physel(&self) -> PHYSELR {
        PHYSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline]
    pub fn usbtrd_tim(&self) -> USBTRDTIMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USBTRDTIMR { bits }
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline]
    pub fn tx_end_delay(&self) -> TXENDDELAYR {
        TXENDDELAYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline]
    pub fn force_dev_mode(&self) -> FORCEDEVMODER {
        FORCEDEVMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline]
    pub fn ctp(&self) -> CTPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 5184 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline]
    pub fn tout_cal(&mut self) -> _TOUTCALW {
        _TOUTCALW { w: self }
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline]
    pub fn usbtrd_tim(&mut self) -> _USBTRDTIMW {
        _USBTRDTIMW { w: self }
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline]
    pub fn tx_end_delay(&mut self) -> _TXENDDELAYW {
        _TXENDDELAYW { w: self }
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline]
    pub fn force_dev_mode(&mut self) -> _FORCEDEVMODEW {
        _FORCEDEVMODEW { w: self }
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline]
    pub fn ctp(&mut self) -> _CTPW {
        _CTPW { w: self }
    }
}
