#[doc = "Register `PRSET2` writer"]
pub struct W(crate::W<PRSET2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSET2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PRSET2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSET2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDT Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<WDTRS_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` writer - WDT Reset Assert"]
pub struct WDTRS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTRS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTRS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTRS_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "DMA0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<DMA0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` writer - DMA0 Reset Assert"]
pub struct DMA0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0RS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMA0RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMA0RS_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "FCE Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCERS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<FCERS_AW> for bool {
    #[inline(always)]
    fn from(variant: FCERS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` writer - FCE Reset Assert"]
pub struct FCERS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCERS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCERS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCERS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCERS_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "USB Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<USBRS_AW> for bool {
    #[inline(always)]
    fn from(variant: USBRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` writer - USB Reset Assert"]
pub struct USBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBRS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBRS_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 1 - WDT Reset Assert"]
    #[inline(always)]
    pub fn wdtrs(&mut self) -> WDTRS_W {
        WDTRS_W { w: self }
    }
    #[doc = "Bit 4 - DMA0 Reset Assert"]
    #[inline(always)]
    pub fn dma0rs(&mut self) -> DMA0RS_W {
        DMA0RS_W { w: self }
    }
    #[doc = "Bit 6 - FCE Reset Assert"]
    #[inline(always)]
    pub fn fcers(&mut self) -> FCERS_W {
        FCERS_W { w: self }
    }
    #[doc = "Bit 7 - USB Reset Assert"]
    #[inline(always)]
    pub fn usbrs(&mut self) -> USBRS_W {
        USBRS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCU Peripheral 2 Reset Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prset2](index.html) module"]
pub struct PRSET2_SPEC;
impl crate::RegisterSpec for PRSET2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prset2::W](W) writer structure"]
impl crate::Writable for PRSET2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRSET2 to value 0"]
impl crate::Resettable for PRSET2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
