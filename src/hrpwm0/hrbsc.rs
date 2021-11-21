#[doc = "Register `HRBSC` reader"]
pub struct R(crate::R<HRBSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRBSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRBSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRBSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRBSC` writer"]
pub struct W(crate::W<HRBSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRBSC_SPEC>;
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
impl From<crate::W<HRBSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRBSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Suspend configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUSCFG_A {
    #[doc = "0: Suspend is ignored."]
    VALUE1 = 0,
    #[doc = "1: CSGy and HRCy units are halted."]
    VALUE2 = 1,
    #[doc = "2: Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE3 = 2,
    #[doc = "3: CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE4 = 3,
}
impl From<SUSCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SUSCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SUSCFG` reader - Suspend configuration"]
pub struct SUSCFG_R(crate::FieldReader<u8, SUSCFG_A>);
impl SUSCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUSCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUSCFG_A> {
        match self.bits {
            0 => Some(SUSCFG_A::VALUE1),
            1 => Some(SUSCFG_A::VALUE2),
            2 => Some(SUSCFG_A::VALUE3),
            3 => Some(SUSCFG_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SUSCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SUSCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SUSCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SUSCFG_A::VALUE4
    }
}
impl core::ops::Deref for SUSCFG_R {
    type Target = crate::FieldReader<u8, SUSCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSCFG` writer - Suspend configuration"]
pub struct SUSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Suspend is ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE1)
    }
    #[doc = "CSGy and HRCy units are halted."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE2)
    }
    #[doc = "Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE3)
    }
    #[doc = "CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `HRBE` reader - HRPWM bias enable"]
pub struct HRBE_R(crate::FieldReader<bool, bool>);
impl HRBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRBE` writer - HRPWM bias enable"]
pub struct HRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> HRBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SUSCFG_R {
        SUSCFG_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline(always)]
    pub fn hrbe(&self) -> HRBE_R {
        HRBE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline(always)]
    pub fn suscfg(&mut self) -> SUSCFG_W {
        SUSCFG_W { w: self }
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline(always)]
    pub fn hrbe(&mut self) -> HRBE_W {
        HRBE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bias and suspend configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrbsc](index.html) module"]
pub struct HRBSC_SPEC;
impl crate::RegisterSpec for HRBSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrbsc::R](R) reader structure"]
impl crate::Readable for HRBSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrbsc::W](W) writer structure"]
impl crate::Writable for HRBSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HRBSC to value 0"]
impl crate::Resettable for HRBSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
