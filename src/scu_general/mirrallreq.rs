#[doc = "Register `MIRRALLREQ` writer"]
pub struct W(crate::W<MIRRALLREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIRRALLREQ_SPEC>;
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
impl From<crate::W<MIRRALLREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIRRALLREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mirror All Execution Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQ_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Start mirror update"]
    VALUE2 = 1,
}
impl From<REQ_AW> for bool {
    #[inline(always)]
    fn from(variant: REQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQ` writer - Mirror All Execution Request"]
pub struct REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQ_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REQ_AW::VALUE1)
    }
    #[doc = "Start mirror update"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REQ_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Mirror All Execution Request"]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mirror All Request\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mirrallreq](index.html) module"]
pub struct MIRRALLREQ_SPEC;
impl crate::RegisterSpec for MIRRALLREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mirrallreq::W](W) writer structure"]
impl crate::Writable for MIRRALLREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIRRALLREQ to value 0"]
impl crate::Resettable for MIRRALLREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
