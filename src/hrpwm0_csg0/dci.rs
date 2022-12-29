#[doc = "Register `DCI` reader"]
pub struct R(crate::R<DCI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCI` writer"]
pub struct W(crate::W<DCI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCI_SPEC>;
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
impl From<crate::W<DCI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVIS` reader - Value Selector input selection"]
pub type SVIS_R = crate::FieldReader<u8, SVIS_A>;
#[doc = "Value Selector input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SVIS_A {
    #[doc = "0: HRPWMx.SyIA"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.SyIB"]
    VALUE2 = 1,
    #[doc = "2: HRPWMx.SyIC"]
    VALUE3 = 2,
    #[doc = "3: HRPWMx.SyID"]
    VALUE4 = 3,
    #[doc = "4: HRPWMx.SyIE"]
    VALUE5 = 4,
    #[doc = "5: HRPWMx.SyIF"]
    VALUE6 = 5,
    #[doc = "6: HRPWMx.SyIG"]
    VALUE7 = 6,
    #[doc = "7: HRPWMx.SyIH"]
    VALUE8 = 7,
    #[doc = "8: HRPWMx.SyII"]
    VALUE9 = 8,
    #[doc = "9: HRPWMx.SyIJ"]
    VALUE10 = 9,
    #[doc = "10: HRPWMx.SyIK"]
    VALUE11 = 10,
    #[doc = "11: HRPWMx.SyIL"]
    VALUE12 = 11,
    #[doc = "12: HRPWMx.SyIM"]
    VALUE13 = 12,
    #[doc = "13: HRPWMx.SyIN"]
    VALUE14 = 13,
    #[doc = "14: HRPWMx.SyIO"]
    VALUE15 = 14,
    #[doc = "15: HRPWMx.SyIP"]
    VALUE16 = 15,
}
impl From<SVIS_A> for u8 {
    #[inline(always)]
    fn from(variant: SVIS_A) -> Self {
        variant as _
    }
}
impl SVIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVIS_A {
        match self.bits {
            0 => SVIS_A::VALUE1,
            1 => SVIS_A::VALUE2,
            2 => SVIS_A::VALUE3,
            3 => SVIS_A::VALUE4,
            4 => SVIS_A::VALUE5,
            5 => SVIS_A::VALUE6,
            6 => SVIS_A::VALUE7,
            7 => SVIS_A::VALUE8,
            8 => SVIS_A::VALUE9,
            9 => SVIS_A::VALUE10,
            10 => SVIS_A::VALUE11,
            11 => SVIS_A::VALUE12,
            12 => SVIS_A::VALUE13,
            13 => SVIS_A::VALUE14,
            14 => SVIS_A::VALUE15,
            15 => SVIS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SVIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SVIS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SVIS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SVIS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SVIS_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SVIS_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SVIS_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == SVIS_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == SVIS_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == SVIS_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == SVIS_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == SVIS_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == SVIS_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == SVIS_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == SVIS_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == SVIS_A::VALUE16
    }
}
#[doc = "Field `SVIS` writer - Value Selector input selection"]
pub type SVIS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCI_SPEC, u8, SVIS_A, 4, O>;
impl<'a, const O: u8> SVIS_W<'a, O> {
    #[doc = "HRPWMx.SyIA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE1)
    }
    #[doc = "HRPWMx.SyIB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE2)
    }
    #[doc = "HRPWMx.SyIC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE3)
    }
    #[doc = "HRPWMx.SyID"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE4)
    }
    #[doc = "HRPWMx.SyIE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE5)
    }
    #[doc = "HRPWMx.SyIF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE6)
    }
    #[doc = "HRPWMx.SyIG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE7)
    }
    #[doc = "HRPWMx.SyIH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE8)
    }
    #[doc = "HRPWMx.SyII"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE9)
    }
    #[doc = "HRPWMx.SyIJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE10)
    }
    #[doc = "HRPWMx.SyIK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE11)
    }
    #[doc = "HRPWMx.SyIL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE12)
    }
    #[doc = "HRPWMx.SyIM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE13)
    }
    #[doc = "HRPWMx.SyIN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE14)
    }
    #[doc = "HRPWMx.SyIO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE15)
    }
    #[doc = "HRPWMx.SyIP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(SVIS_A::VALUE16)
    }
}
#[doc = "Field `STRIS` reader - Slope generation start control input selection"]
pub type STRIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STRIS` writer - Slope generation start control input selection"]
pub type STRIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCI_SPEC, u8, u8, 4, O>;
#[doc = "Field `STPIS` reader - Slope generation stop control input selection"]
pub type STPIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STPIS` writer - Slope generation stop control input selection"]
pub type STPIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCI_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRGIS` reader - External conversion trigger input selection"]
pub type TRGIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGIS` writer - External conversion trigger input selection"]
pub type TRGIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCI_SPEC, u8, u8, 4, O>;
#[doc = "Field `STIS` reader - External shadow request enable input selection"]
pub type STIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STIS` writer - External shadow request enable input selection"]
pub type STIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCI_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCS` reader - Slope generation clock selection"]
pub type SCS_R = crate::FieldReader<u8, SCS_A>;
#[doc = "Slope generation clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCS_A {
    #[doc = "0: HRPWMx.MCLK (Module clock is used)"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.ECLKA (External clock is used)"]
    VALUE2 = 1,
    #[doc = "2: HRPWMx.ECLKB (External clock is used)"]
    VALUE3 = 2,
    #[doc = "3: HRPWMx.ECLKC (External clock is used)"]
    VALUE4 = 3,
}
impl From<SCS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as _
    }
}
impl SCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCS_A {
        match self.bits {
            0 => SCS_A::VALUE1,
            1 => SCS_A::VALUE2,
            2 => SCS_A::VALUE3,
            3 => SCS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SCS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SCS_A::VALUE4
    }
}
#[doc = "Field `SCS` writer - Slope generation clock selection"]
pub type SCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCI_SPEC, u8, SCS_A, 2, O>;
impl<'a, const O: u8> SCS_W<'a, O> {
    #[doc = "HRPWMx.MCLK (Module clock is used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCS_A::VALUE1)
    }
    #[doc = "HRPWMx.ECLKA (External clock is used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCS_A::VALUE2)
    }
    #[doc = "HRPWMx.ECLKB (External clock is used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCS_A::VALUE3)
    }
    #[doc = "HRPWMx.ECLKC (External clock is used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCS_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Value Selector input selection"]
    #[inline(always)]
    pub fn svis(&self) -> SVIS_R {
        SVIS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Slope generation start control input selection"]
    #[inline(always)]
    pub fn stris(&self) -> STRIS_R {
        STRIS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Slope generation stop control input selection"]
    #[inline(always)]
    pub fn stpis(&self) -> STPIS_R {
        STPIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External conversion trigger input selection"]
    #[inline(always)]
    pub fn trgis(&self) -> TRGIS_R {
        TRGIS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External shadow request enable input selection"]
    #[inline(always)]
    pub fn stis(&self) -> STIS_R {
        STIS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Slope generation clock selection"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Value Selector input selection"]
    #[inline(always)]
    #[must_use]
    pub fn svis(&mut self) -> SVIS_W<0> {
        SVIS_W::new(self)
    }
    #[doc = "Bits 4:7 - Slope generation start control input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stris(&mut self) -> STRIS_W<4> {
        STRIS_W::new(self)
    }
    #[doc = "Bits 8:11 - Slope generation stop control input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stpis(&mut self) -> STPIS_W<8> {
        STPIS_W::new(self)
    }
    #[doc = "Bits 12:15 - External conversion trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgis(&mut self) -> TRGIS_W<12> {
        TRGIS_W::new(self)
    }
    #[doc = "Bits 16:19 - External shadow request enable input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stis(&mut self) -> STIS_W<16> {
        STIS_W::new(self)
    }
    #[doc = "Bits 20:21 - Slope generation clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> SCS_W<20> {
        SCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dci](index.html) module"]
pub struct DCI_SPEC;
impl crate::RegisterSpec for DCI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dci::R](R) reader structure"]
impl crate::Readable for DCI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dci::W](W) writer structure"]
impl crate::Writable for DCI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCI to value 0"]
impl crate::Resettable for DCI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
