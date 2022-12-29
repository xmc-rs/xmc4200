#[doc = "Register `HINTCLR` writer"]
pub struct W(crate::W<HINTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HINTCLR_SPEC>;
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
impl From<crate::W<HINTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HINTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internally Controlled Hibernate Sequence Request Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBNINT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Hibernate bit clear"]
    VALUE2 = 1,
}
impl From<HIBNINT_AW> for bool {
    #[inline(always)]
    fn from(variant: HIBNINT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNINT` writer - Internally Controlled Hibernate Sequence Request Clear"]
pub type HIBNINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HINTCLR_SPEC, HIBNINT_AW, O>;
impl<'a, const O: u8> HIBNINT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBNINT_AW::VALUE1)
    }
    #[doc = "Hibernate bit clear"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBNINT_AW::VALUE2)
    }
}
#[doc = "VDDP Supply Switch of Flash Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHOFF_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Switch on VDDP supply of Flash"]
    VALUE2 = 1,
}
impl From<FLASHOFF_AW> for bool {
    #[inline(always)]
    fn from(variant: FLASHOFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHOFF` writer - VDDP Supply Switch of Flash Clear"]
pub type FLASHOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HINTCLR_SPEC, FLASHOFF_AW, O>;
impl<'a, const O: u8> FLASHOFF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLASHOFF_AW::VALUE1)
    }
    #[doc = "Switch on VDDP supply of Flash"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLASHOFF_AW::VALUE2)
    }
}
#[doc = "Flash Power Down Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHPD_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Flash power down mode leave request"]
    VALUE2 = 1,
}
impl From<FLASHPD_AW> for bool {
    #[inline(always)]
    fn from(variant: FLASHPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHPD` writer - Flash Power Down Clear"]
pub type FLASHPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HINTCLR_SPEC, FLASHPD_AW, O>;
impl<'a, const O: u8> FLASHPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLASHPD_AW::VALUE1)
    }
    #[doc = "Flash power down mode leave request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLASHPD_AW::VALUE2)
    }
}
#[doc = "PORST Pull-up OFF Direct Control Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POFFD_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up on"]
    VALUE2 = 1,
}
impl From<POFFD_AW> for bool {
    #[inline(always)]
    fn from(variant: POFFD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFD` writer - PORST Pull-up OFF Direct Control Clear"]
pub type POFFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HINTCLR_SPEC, POFFD_AW, O>;
impl<'a, const O: u8> POFFD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POFFD_AW::VALUE1)
    }
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POFFD_AW::VALUE2)
    }
}
#[doc = "Field `PPODEL` writer - Delay on PORTS Pull-up Switching OFF on Hibernate Request Clear"]
pub type PPODEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HINTCLR_SPEC, u8, u8, 2, O>;
#[doc = "PORST Pull-up OFF in Hibernate Mode Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POFFH_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up on"]
    VALUE2 = 1,
}
impl From<POFFH_AW> for bool {
    #[inline(always)]
    fn from(variant: POFFH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFH` writer - PORST Pull-up OFF in Hibernate Mode Clear"]
pub type POFFH_W<'a, const O: u8> = crate::BitWriter<'a, u32, HINTCLR_SPEC, POFFH_AW, O>;
impl<'a, const O: u8> POFFH_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POFFH_AW::VALUE1)
    }
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POFFH_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hibnint(&mut self) -> HIBNINT_W<0> {
        HIBNINT_W::new(self)
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flashoff(&mut self) -> FLASHOFF_W<2> {
        FLASHOFF_W::new(self)
    }
    #[doc = "Bit 3 - Flash Power Down Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flashpd(&mut self) -> FLASHPD_W<3> {
        FLASHPD_W::new(self)
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn poffd(&mut self) -> POFFD_W<4> {
        POFFD_W::new(self)
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ppodel(&mut self) -> PPODEL_W<16> {
        PPODEL_W::new(self)
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode Clear"]
    #[inline(always)]
    #[must_use]
    pub fn poffh(&mut self) -> POFFH_W<20> {
        POFFH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernate Internal Control Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hintclr](index.html) module"]
pub struct HINTCLR_SPEC;
impl crate::RegisterSpec for HINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hintclr::W](W) writer structure"]
impl crate::Writable for HINTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HINTCLR to value 0"]
impl crate::Resettable for HINTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
