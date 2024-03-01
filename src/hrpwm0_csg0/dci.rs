#[doc = "Register `DCI` reader"]
pub type R = crate::R<DciSpec>;
#[doc = "Register `DCI` writer"]
pub type W = crate::W<DciSpec>;
#[doc = "Value Selector input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Svis {
    #[doc = "0: HRPWMx.SyIA"]
    Value1 = 0,
    #[doc = "1: HRPWMx.SyIB"]
    Value2 = 1,
    #[doc = "2: HRPWMx.SyIC"]
    Value3 = 2,
    #[doc = "3: HRPWMx.SyID"]
    Value4 = 3,
    #[doc = "4: HRPWMx.SyIE"]
    Value5 = 4,
    #[doc = "5: HRPWMx.SyIF"]
    Value6 = 5,
    #[doc = "6: HRPWMx.SyIG"]
    Value7 = 6,
    #[doc = "7: HRPWMx.SyIH"]
    Value8 = 7,
    #[doc = "8: HRPWMx.SyII"]
    Value9 = 8,
    #[doc = "9: HRPWMx.SyIJ"]
    Value10 = 9,
    #[doc = "10: HRPWMx.SyIK"]
    Value11 = 10,
    #[doc = "11: HRPWMx.SyIL"]
    Value12 = 11,
    #[doc = "12: HRPWMx.SyIM"]
    Value13 = 12,
    #[doc = "13: HRPWMx.SyIN"]
    Value14 = 13,
    #[doc = "14: HRPWMx.SyIO"]
    Value15 = 14,
    #[doc = "15: HRPWMx.SyIP"]
    Value16 = 15,
}
impl From<Svis> for u8 {
    #[inline(always)]
    fn from(variant: Svis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Svis {
    type Ux = u8;
}
#[doc = "Field `SVIS` reader - Value Selector input selection"]
pub type SvisR = crate::FieldReader<Svis>;
impl SvisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svis {
        match self.bits {
            0 => Svis::Value1,
            1 => Svis::Value2,
            2 => Svis::Value3,
            3 => Svis::Value4,
            4 => Svis::Value5,
            5 => Svis::Value6,
            6 => Svis::Value7,
            7 => Svis::Value8,
            8 => Svis::Value9,
            9 => Svis::Value10,
            10 => Svis::Value11,
            11 => Svis::Value12,
            12 => Svis::Value13,
            13 => Svis::Value14,
            14 => Svis::Value15,
            15 => Svis::Value16,
            _ => unreachable!(),
        }
    }
    #[doc = "HRPWMx.SyIA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Svis::Value1
    }
    #[doc = "HRPWMx.SyIB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Svis::Value2
    }
    #[doc = "HRPWMx.SyIC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Svis::Value3
    }
    #[doc = "HRPWMx.SyID"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Svis::Value4
    }
    #[doc = "HRPWMx.SyIE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Svis::Value5
    }
    #[doc = "HRPWMx.SyIF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Svis::Value6
    }
    #[doc = "HRPWMx.SyIG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Svis::Value7
    }
    #[doc = "HRPWMx.SyIH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Svis::Value8
    }
    #[doc = "HRPWMx.SyII"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Svis::Value9
    }
    #[doc = "HRPWMx.SyIJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Svis::Value10
    }
    #[doc = "HRPWMx.SyIK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Svis::Value11
    }
    #[doc = "HRPWMx.SyIL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Svis::Value12
    }
    #[doc = "HRPWMx.SyIM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Svis::Value13
    }
    #[doc = "HRPWMx.SyIN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Svis::Value14
    }
    #[doc = "HRPWMx.SyIO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Svis::Value15
    }
    #[doc = "HRPWMx.SyIP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Svis::Value16
    }
}
#[doc = "Field `SVIS` writer - Value Selector input selection"]
pub type SvisW<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, Svis>;
impl<'a, REG> SvisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HRPWMx.SyIA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value1)
    }
    #[doc = "HRPWMx.SyIB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value2)
    }
    #[doc = "HRPWMx.SyIC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value3)
    }
    #[doc = "HRPWMx.SyID"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value4)
    }
    #[doc = "HRPWMx.SyIE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value5)
    }
    #[doc = "HRPWMx.SyIF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value6)
    }
    #[doc = "HRPWMx.SyIG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value7)
    }
    #[doc = "HRPWMx.SyIH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value8)
    }
    #[doc = "HRPWMx.SyII"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value9)
    }
    #[doc = "HRPWMx.SyIJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value10)
    }
    #[doc = "HRPWMx.SyIK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value11)
    }
    #[doc = "HRPWMx.SyIL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value12)
    }
    #[doc = "HRPWMx.SyIM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value13)
    }
    #[doc = "HRPWMx.SyIN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value14)
    }
    #[doc = "HRPWMx.SyIO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value15)
    }
    #[doc = "HRPWMx.SyIP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Svis::Value16)
    }
}
#[doc = "Field `STRIS` reader - Slope generation start control input selection"]
pub type StrisR = crate::FieldReader;
#[doc = "Field `STRIS` writer - Slope generation start control input selection"]
pub type StrisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STPIS` reader - Slope generation stop control input selection"]
pub type StpisR = crate::FieldReader;
#[doc = "Field `STPIS` writer - Slope generation stop control input selection"]
pub type StpisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRGIS` reader - External conversion trigger input selection"]
pub type TrgisR = crate::FieldReader;
#[doc = "Field `TRGIS` writer - External conversion trigger input selection"]
pub type TrgisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STIS` reader - External shadow request enable input selection"]
pub type StisR = crate::FieldReader;
#[doc = "Field `STIS` writer - External shadow request enable input selection"]
pub type StisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Slope generation clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scs {
    #[doc = "0: HRPWMx.MCLK (Module clock is used)"]
    Value1 = 0,
    #[doc = "1: HRPWMx.ECLKA (External clock is used)"]
    Value2 = 1,
    #[doc = "2: HRPWMx.ECLKB (External clock is used)"]
    Value3 = 2,
    #[doc = "3: HRPWMx.ECLKC (External clock is used)"]
    Value4 = 3,
}
impl From<Scs> for u8 {
    #[inline(always)]
    fn from(variant: Scs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scs {
    type Ux = u8;
}
#[doc = "Field `SCS` reader - Slope generation clock selection"]
pub type ScsR = crate::FieldReader<Scs>;
impl ScsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scs {
        match self.bits {
            0 => Scs::Value1,
            1 => Scs::Value2,
            2 => Scs::Value3,
            3 => Scs::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "HRPWMx.MCLK (Module clock is used)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Scs::Value1
    }
    #[doc = "HRPWMx.ECLKA (External clock is used)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Scs::Value2
    }
    #[doc = "HRPWMx.ECLKB (External clock is used)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Scs::Value3
    }
    #[doc = "HRPWMx.ECLKC (External clock is used)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Scs::Value4
    }
}
#[doc = "Field `SCS` writer - Slope generation clock selection"]
pub type ScsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Scs>;
impl<'a, REG> ScsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HRPWMx.MCLK (Module clock is used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Value1)
    }
    #[doc = "HRPWMx.ECLKA (External clock is used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Value2)
    }
    #[doc = "HRPWMx.ECLKB (External clock is used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Value3)
    }
    #[doc = "HRPWMx.ECLKC (External clock is used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Value Selector input selection"]
    #[inline(always)]
    pub fn svis(&self) -> SvisR {
        SvisR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Slope generation start control input selection"]
    #[inline(always)]
    pub fn stris(&self) -> StrisR {
        StrisR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Slope generation stop control input selection"]
    #[inline(always)]
    pub fn stpis(&self) -> StpisR {
        StpisR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External conversion trigger input selection"]
    #[inline(always)]
    pub fn trgis(&self) -> TrgisR {
        TrgisR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External shadow request enable input selection"]
    #[inline(always)]
    pub fn stis(&self) -> StisR {
        StisR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Slope generation clock selection"]
    #[inline(always)]
    pub fn scs(&self) -> ScsR {
        ScsR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Value Selector input selection"]
    #[inline(always)]
    #[must_use]
    pub fn svis(&mut self) -> SvisW<DciSpec> {
        SvisW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Slope generation start control input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stris(&mut self) -> StrisW<DciSpec> {
        StrisW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Slope generation stop control input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stpis(&mut self) -> StpisW<DciSpec> {
        StpisW::new(self, 8)
    }
    #[doc = "Bits 12:15 - External conversion trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgis(&mut self) -> TrgisW<DciSpec> {
        TrgisW::new(self, 12)
    }
    #[doc = "Bits 16:19 - External shadow request enable input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stis(&mut self) -> StisW<DciSpec> {
        StisW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Slope generation clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> ScsW<DciSpec> {
        ScsW::new(self, 20)
    }
}
#[doc = "External input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dci::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dci::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DciSpec;
impl crate::RegisterSpec for DciSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dci::R`](R) reader structure"]
impl crate::Readable for DciSpec {}
#[doc = "`write(|w| ..)` method takes [`dci::W`](W) writer structure"]
impl crate::Writable for DciSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCI to value 0"]
impl crate::Resettable for DciSpec {
    const RESET_VALUE: u32 = 0;
}
