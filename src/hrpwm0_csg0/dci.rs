#[doc = "Reader of register DCI"]
pub type R = crate::R<u32, super::DCI>;
#[doc = "Writer for register DCI"]
pub type W = crate::W<u32, super::DCI>;
#[doc = "Register DCI `reset()`'s with value 0"]
impl crate::ResetValue for super::DCI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Value Selector input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVIS_A {
    #[doc = "0: HRPWMx.SyIA"]
    VALUE1,
    #[doc = "1: HRPWMx.SyIB"]
    VALUE2,
    #[doc = "2: HRPWMx.SyIC"]
    VALUE3,
    #[doc = "3: HRPWMx.SyID"]
    VALUE4,
    #[doc = "4: HRPWMx.SyIE"]
    VALUE5,
    #[doc = "5: HRPWMx.SyIF"]
    VALUE6,
    #[doc = "6: HRPWMx.SyIG"]
    VALUE7,
    #[doc = "7: HRPWMx.SyIH"]
    VALUE8,
    #[doc = "8: HRPWMx.SyII"]
    VALUE9,
    #[doc = "9: HRPWMx.SyIJ"]
    VALUE10,
    #[doc = "10: HRPWMx.SyIK"]
    VALUE11,
    #[doc = "11: HRPWMx.SyIL"]
    VALUE12,
    #[doc = "12: HRPWMx.SyIM"]
    VALUE13,
    #[doc = "13: HRPWMx.SyIN"]
    VALUE14,
    #[doc = "14: HRPWMx.SyIO"]
    VALUE15,
    #[doc = "15: HRPWMx.SyIP"]
    VALUE16,
}
impl From<SVIS_A> for u8 {
    #[inline(always)]
    fn from(variant: SVIS_A) -> Self {
        match variant {
            SVIS_A::VALUE1 => 0,
            SVIS_A::VALUE2 => 1,
            SVIS_A::VALUE3 => 2,
            SVIS_A::VALUE4 => 3,
            SVIS_A::VALUE5 => 4,
            SVIS_A::VALUE6 => 5,
            SVIS_A::VALUE7 => 6,
            SVIS_A::VALUE8 => 7,
            SVIS_A::VALUE9 => 8,
            SVIS_A::VALUE10 => 9,
            SVIS_A::VALUE11 => 10,
            SVIS_A::VALUE12 => 11,
            SVIS_A::VALUE13 => 12,
            SVIS_A::VALUE14 => 13,
            SVIS_A::VALUE15 => 14,
            SVIS_A::VALUE16 => 15,
        }
    }
}
#[doc = "Reader of field `SVIS`"]
pub type SVIS_R = crate::R<u8, SVIS_A>;
impl SVIS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `SVIS`"]
pub struct SVIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SVIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVIS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `STRIS`"]
pub type STRIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STRIS`"]
pub struct STRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STRIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `STPIS`"]
pub type STPIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STPIS`"]
pub struct STPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STPIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRGIS`"]
pub type TRGIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGIS`"]
pub struct TRGIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `STIS`"]
pub type STIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STIS`"]
pub struct STIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Slope generation clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCS_A {
    #[doc = "0: HRPWMx.MCLK (Module clock is used)"]
    VALUE1,
    #[doc = "1: HRPWMx.ECLKA (External clock is used)"]
    VALUE2,
    #[doc = "2: HRPWMx.ECLKB (External clock is used)"]
    VALUE3,
    #[doc = "3: HRPWMx.ECLKC (External clock is used)"]
    VALUE4,
}
impl From<SCS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        match variant {
            SCS_A::VALUE1 => 0,
            SCS_A::VALUE2 => 1,
            SCS_A::VALUE3 => 2,
            SCS_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<u8, SCS_A>;
impl SCS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `SCS`"]
pub struct SCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
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
        SCS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Value Selector input selection"]
    #[inline(always)]
    pub fn svis(&mut self) -> SVIS_W {
        SVIS_W { w: self }
    }
    #[doc = "Bits 4:7 - Slope generation start control input selection"]
    #[inline(always)]
    pub fn stris(&mut self) -> STRIS_W {
        STRIS_W { w: self }
    }
    #[doc = "Bits 8:11 - Slope generation stop control input selection"]
    #[inline(always)]
    pub fn stpis(&mut self) -> STPIS_W {
        STPIS_W { w: self }
    }
    #[doc = "Bits 12:15 - External conversion trigger input selection"]
    #[inline(always)]
    pub fn trgis(&mut self) -> TRGIS_W {
        TRGIS_W { w: self }
    }
    #[doc = "Bits 16:19 - External shadow request enable input selection"]
    #[inline(always)]
    pub fn stis(&mut self) -> STIS_W {
        STIS_W { w: self }
    }
    #[doc = "Bits 20:21 - Slope generation clock selection"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W { w: self }
    }
}
