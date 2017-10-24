#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HINTCLR {
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
}
#[doc = "Values that can be written to the field `HIBNINT`"]
pub enum HIBNINTW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Hibernate bit clear"]
    VALUE2,
}
impl HIBNINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBNINTW::VALUE1 => false,
            HIBNINTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBNINTW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBNINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBNINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBNINTW::VALUE1)
    }
    #[doc = "Hibernate bit clear"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBNINTW::VALUE2)
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
#[doc = "Values that can be written to the field `FLASHOFF`"]
pub enum FLASHOFFW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Switch on VDDP supply of Flash"]
    VALUE2,
}
impl FLASHOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHOFFW::VALUE1 => false,
            FLASHOFFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHOFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLASHOFFW::VALUE1)
    }
    #[doc = "Switch on VDDP supply of Flash"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLASHOFFW::VALUE2)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLASHPD`"]
pub enum FLASHPDW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Flash power down mode leave request"]
    VALUE2,
}
impl FLASHPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHPDW::VALUE1 => false,
            FLASHPDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHPDW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHPDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLASHPDW::VALUE1)
    }
    #[doc = "Flash power down mode leave request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLASHPDW::VALUE2)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POFFD`"]
pub enum POFFDW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Pull-up on"]
    VALUE2,
}
impl POFFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POFFDW::VALUE1 => false,
            POFFDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POFFDW<'a> {
    w: &'a mut W,
}
impl<'a> _POFFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POFFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(POFFDW::VALUE1)
    }
    #[doc = "Pull-up on"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(POFFDW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _PPODELW<'a> {
    w: &'a mut W,
}
impl<'a> _PPODELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POFFH`"]
pub enum POFFHW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Pull-up on"]
    VALUE2,
}
impl POFFHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POFFHW::VALUE1 => false,
            POFFHW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POFFHW<'a> {
    w: &'a mut W,
}
impl<'a> _POFFHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POFFHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(POFFHW::VALUE1)
    }
    #[doc = "Pull-up on"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(POFFHW::VALUE2)
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
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request Clear"]
    #[inline]
    pub fn hibnint(&mut self) -> _HIBNINTW {
        _HIBNINTW { w: self }
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash Clear"]
    #[inline]
    pub fn flashoff(&mut self) -> _FLASHOFFW {
        _FLASHOFFW { w: self }
    }
    #[doc = "Bit 3 - Flash Power Down Clear"]
    #[inline]
    pub fn flashpd(&mut self) -> _FLASHPDW {
        _FLASHPDW { w: self }
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control Clear"]
    #[inline]
    pub fn poffd(&mut self) -> _POFFDW {
        _POFFDW { w: self }
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request Clear"]
    #[inline]
    pub fn ppodel(&mut self) -> _PPODELW {
        _PPODELW { w: self }
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode Clear"]
    #[inline]
    pub fn poffh(&mut self) -> _POFFHW {
        _POFFHW { w: self }
    }
}
