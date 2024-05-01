#[doc = "Register `IES` reader"]
pub type R = crate::R<IesSpec>;
#[doc = "Register `IES` writer"]
pub type W = crate::W<IesSpec>;
#[doc = "External value switch function level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Svls {
    #[doc = "0: Function disabled"]
    Value1 = 0,
    #[doc = "1: Active when input is HIGH"]
    Value2 = 1,
    #[doc = "2: Active when input is LOW"]
    Value3 = 2,
}
impl From<Svls> for u8 {
    #[inline(always)]
    fn from(variant: Svls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Svls {
    type Ux = u8;
}
impl crate::IsEnum for Svls {}
#[doc = "Field `SVLS` reader - External value switch function level selection"]
pub type SvlsR = crate::FieldReader<Svls>;
impl SvlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Svls> {
        match self.bits {
            0 => Some(Svls::Value1),
            1 => Some(Svls::Value2),
            2 => Some(Svls::Value3),
            _ => None,
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Svls::Value1
    }
    #[doc = "Active when input is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Svls::Value2
    }
    #[doc = "Active when input is LOW"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Svls::Value3
    }
}
#[doc = "Field `SVLS` writer - External value switch function level selection"]
pub type SvlsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Svls>;
impl<'a, REG> SvlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Svls::Value1)
    }
    #[doc = "Active when input is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Svls::Value2)
    }
    #[doc = "Active when input is LOW"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Svls::Value3)
    }
}
#[doc = "External start function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stres {
    #[doc = "0: Function disabled"]
    Value1 = 0,
    #[doc = "1: Active on rising edge"]
    Value2 = 1,
    #[doc = "2: Active on falling edge"]
    Value3 = 2,
    #[doc = "3: Active on both edges"]
    Value4 = 3,
}
impl From<Stres> for u8 {
    #[inline(always)]
    fn from(variant: Stres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stres {
    type Ux = u8;
}
impl crate::IsEnum for Stres {}
#[doc = "Field `STRES` reader - External start function edge selection"]
pub type StresR = crate::FieldReader<Stres>;
impl StresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stres {
        match self.bits {
            0 => Stres::Value1,
            1 => Stres::Value2,
            2 => Stres::Value3,
            3 => Stres::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stres::Value1
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stres::Value2
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Stres::Value3
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Stres::Value4
    }
}
#[doc = "Field `STRES` writer - External start function edge selection"]
pub type StresW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stres, crate::Safe>;
impl<'a, REG> StresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stres::Value1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stres::Value2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Stres::Value3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Stres::Value4)
    }
}
#[doc = "External stop function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stpes {
    #[doc = "0: Function disabled"]
    Value1 = 0,
    #[doc = "1: Active on rising edge"]
    Value2 = 1,
    #[doc = "2: Active on falling edge"]
    Value3 = 2,
    #[doc = "3: Active on both edges"]
    Value4 = 3,
}
impl From<Stpes> for u8 {
    #[inline(always)]
    fn from(variant: Stpes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stpes {
    type Ux = u8;
}
impl crate::IsEnum for Stpes {}
#[doc = "Field `STPES` reader - External stop function edge selection"]
pub type StpesR = crate::FieldReader<Stpes>;
impl StpesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpes {
        match self.bits {
            0 => Stpes::Value1,
            1 => Stpes::Value2,
            2 => Stpes::Value3,
            3 => Stpes::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stpes::Value1
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stpes::Value2
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Stpes::Value3
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Stpes::Value4
    }
}
#[doc = "Field `STPES` writer - External stop function edge selection"]
pub type StpesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stpes, crate::Safe>;
impl<'a, REG> StpesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stpes::Value1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stpes::Value2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Stpes::Value3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Stpes::Value4)
    }
}
#[doc = "External trigger function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trges {
    #[doc = "0: Function disabled"]
    Value1 = 0,
    #[doc = "1: Active on rising edge"]
    Value2 = 1,
    #[doc = "2: Active on falling edge"]
    Value3 = 2,
    #[doc = "3: Active on both edges"]
    Value4 = 3,
}
impl From<Trges> for u8 {
    #[inline(always)]
    fn from(variant: Trges) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trges {
    type Ux = u8;
}
impl crate::IsEnum for Trges {}
#[doc = "Field `TRGES` reader - External trigger function edge selection"]
pub type TrgesR = crate::FieldReader<Trges>;
impl TrgesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trges {
        match self.bits {
            0 => Trges::Value1,
            1 => Trges::Value2,
            2 => Trges::Value3,
            3 => Trges::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trges::Value1
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trges::Value2
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Trges::Value3
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Trges::Value4
    }
}
#[doc = "Field `TRGES` writer - External trigger function edge selection"]
pub type TrgesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trges, crate::Safe>;
impl<'a, REG> TrgesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trges::Value1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trges::Value2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Trges::Value3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Trges::Value4)
    }
}
#[doc = "External shadow transfer enable edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stes {
    #[doc = "0: Function disabled"]
    Value1 = 0,
    #[doc = "1: Active on rising edge"]
    Value2 = 1,
    #[doc = "2: Active on falling edge"]
    Value3 = 2,
    #[doc = "3: Active on both edges"]
    Value4 = 3,
}
impl From<Stes> for u8 {
    #[inline(always)]
    fn from(variant: Stes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stes {
    type Ux = u8;
}
impl crate::IsEnum for Stes {}
#[doc = "Field `STES` reader - External shadow transfer enable edge selection"]
pub type StesR = crate::FieldReader<Stes>;
impl StesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stes {
        match self.bits {
            0 => Stes::Value1,
            1 => Stes::Value2,
            2 => Stes::Value3,
            3 => Stes::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stes::Value1
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stes::Value2
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Stes::Value3
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Stes::Value4
    }
}
#[doc = "Field `STES` writer - External shadow transfer enable edge selection"]
pub type StesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stes, crate::Safe>;
impl<'a, REG> StesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stes::Value1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stes::Value2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Stes::Value3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Stes::Value4)
    }
}
impl R {
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline(always)]
    pub fn svls(&self) -> SvlsR {
        SvlsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline(always)]
    pub fn stres(&self) -> StresR {
        StresR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline(always)]
    pub fn stpes(&self) -> StpesR {
        StpesR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline(always)]
    pub fn trges(&self) -> TrgesR {
        TrgesR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline(always)]
    pub fn stes(&self) -> StesR {
        StesR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline(always)]
    #[must_use]
    pub fn svls(&mut self) -> SvlsW<IesSpec> {
        SvlsW::new(self, 0)
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn stres(&mut self) -> StresW<IesSpec> {
        StresW::new(self, 2)
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn stpes(&mut self) -> StpesW<IesSpec> {
        StpesW::new(self, 4)
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn trges(&mut self) -> TrgesW<IesSpec> {
        TrgesW::new(self, 6)
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn stes(&mut self) -> StesW<IesSpec> {
        StesW::new(self, 8)
    }
}
#[doc = "External input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ies::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ies::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IesSpec;
impl crate::RegisterSpec for IesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ies::R`](R) reader structure"]
impl crate::Readable for IesSpec {}
#[doc = "`write(|w| ..)` method takes [`ies::W`](W) writer structure"]
impl crate::Writable for IesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IES to value 0"]
impl crate::Resettable for IesSpec {
    const RESET_VALUE: u32 = 0;
}
