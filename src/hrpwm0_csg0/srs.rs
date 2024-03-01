#[doc = "Register `SRS` reader"]
pub type R = crate::R<SrsSpec>;
#[doc = "Register `SRS` writer"]
pub type W = crate::W<SrsSpec>;
#[doc = "Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vls1s {
    #[doc = "0: CSGySR0"]
    Value1 = 0,
    #[doc = "1: CSGySR1"]
    Value2 = 1,
    #[doc = "2: CSGySR2"]
    Value3 = 2,
    #[doc = "3: CSGySR3"]
    Value4 = 3,
}
impl From<Vls1s> for u8 {
    #[inline(always)]
    fn from(variant: Vls1s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vls1s {
    type Ux = u8;
}
#[doc = "Field `VLS1S` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
pub type Vls1sR = crate::FieldReader<Vls1s>;
impl Vls1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vls1s {
        match self.bits {
            0 => Vls1s::Value1,
            1 => Vls1s::Value2,
            2 => Vls1s::Value3,
            3 => Vls1s::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vls1s::Value1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vls1s::Value2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Vls1s::Value3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Vls1s::Value4
    }
}
#[doc = "Field `VLS1S` writer - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
pub type Vls1sW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Vls1s>;
impl<'a, REG> Vls1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vls1s::Value1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vls1s::Value2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Vls1s::Value3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Vls1s::Value4)
    }
}
#[doc = "Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vls2s {
    #[doc = "0: CSGySR0"]
    Value1 = 0,
    #[doc = "1: CSGySR1"]
    Value2 = 1,
    #[doc = "2: CSGySR2"]
    Value3 = 2,
    #[doc = "3: CSGySR3"]
    Value4 = 3,
}
impl From<Vls2s> for u8 {
    #[inline(always)]
    fn from(variant: Vls2s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vls2s {
    type Ux = u8;
}
#[doc = "Field `VLS2S` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
pub type Vls2sR = crate::FieldReader<Vls2s>;
impl Vls2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vls2s {
        match self.bits {
            0 => Vls2s::Value1,
            1 => Vls2s::Value2,
            2 => Vls2s::Value3,
            3 => Vls2s::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vls2s::Value1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vls2s::Value2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Vls2s::Value3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Vls2s::Value4
    }
}
#[doc = "Field `VLS2S` writer - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
pub type Vls2sW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Vls2s>;
impl<'a, REG> Vls2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vls2s::Value1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vls2s::Value2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Vls2s::Value3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Vls2s::Value4)
    }
}
#[doc = "Conversion trigger interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trls {
    #[doc = "0: CSGySR0"]
    Value1 = 0,
    #[doc = "1: CSGySR1"]
    Value2 = 1,
    #[doc = "2: CSGySR2"]
    Value3 = 2,
    #[doc = "3: CSGySR3"]
    Value4 = 3,
}
impl From<Trls> for u8 {
    #[inline(always)]
    fn from(variant: Trls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trls {
    type Ux = u8;
}
#[doc = "Field `TRLS` reader - Conversion trigger interrupt line selection"]
pub type TrlsR = crate::FieldReader<Trls>;
impl TrlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trls {
        match self.bits {
            0 => Trls::Value1,
            1 => Trls::Value2,
            2 => Trls::Value3,
            3 => Trls::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trls::Value1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trls::Value2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Trls::Value3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Trls::Value4
    }
}
#[doc = "Field `TRLS` writer - Conversion trigger interrupt line selection"]
pub type TrlsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Trls>;
impl<'a, REG> TrlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trls::Value1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trls::Value2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Trls::Value3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Trls::Value4)
    }
}
#[doc = "Start/Stop trigger interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssls {
    #[doc = "0: CSGySR0"]
    Value1 = 0,
    #[doc = "1: CSGySR1"]
    Value2 = 1,
    #[doc = "2: CSGySR2"]
    Value3 = 2,
    #[doc = "3: CSGySR3"]
    Value4 = 3,
}
impl From<Ssls> for u8 {
    #[inline(always)]
    fn from(variant: Ssls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssls {
    type Ux = u8;
}
#[doc = "Field `SSLS` reader - Start/Stop trigger interrupt line selection"]
pub type SslsR = crate::FieldReader<Ssls>;
impl SslsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssls {
        match self.bits {
            0 => Ssls::Value1,
            1 => Ssls::Value2,
            2 => Ssls::Value3,
            3 => Ssls::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ssls::Value1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ssls::Value2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ssls::Value3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ssls::Value4
    }
}
#[doc = "Field `SSLS` writer - Start/Stop trigger interrupt line selection"]
pub type SslsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ssls>;
impl<'a, REG> SslsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssls::Value1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ssls::Value2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ssls::Value3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ssls::Value4)
    }
}
#[doc = "Shadow transfer done interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stls {
    #[doc = "0: CSGySR0"]
    Value1 = 0,
    #[doc = "1: CSGySR1"]
    Value2 = 1,
    #[doc = "2: CSGySR2"]
    Value3 = 2,
    #[doc = "3: CSGySR3"]
    Value4 = 3,
}
impl From<Stls> for u8 {
    #[inline(always)]
    fn from(variant: Stls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stls {
    type Ux = u8;
}
#[doc = "Field `STLS` reader - Shadow transfer done interrupt line selection"]
pub type StlsR = crate::FieldReader<Stls>;
impl StlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stls {
        match self.bits {
            0 => Stls::Value1,
            1 => Stls::Value2,
            2 => Stls::Value3,
            3 => Stls::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stls::Value1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stls::Value2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Stls::Value3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Stls::Value4
    }
}
#[doc = "Field `STLS` writer - Shadow transfer done interrupt line selection"]
pub type StlsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Stls>;
impl<'a, REG> StlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stls::Value1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stls::Value2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Stls::Value3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Stls::Value4)
    }
}
#[doc = "Comparator rise/fall interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crfls {
    #[doc = "0: CSGySR0"]
    Value1 = 0,
    #[doc = "1: CSGySR1"]
    Value2 = 1,
    #[doc = "2: CSGySR2"]
    Value3 = 2,
    #[doc = "3: CSGySR3"]
    Value4 = 3,
}
impl From<Crfls> for u8 {
    #[inline(always)]
    fn from(variant: Crfls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crfls {
    type Ux = u8;
}
#[doc = "Field `CRFLS` reader - Comparator rise/fall interrupt line selection"]
pub type CrflsR = crate::FieldReader<Crfls>;
impl CrflsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crfls {
        match self.bits {
            0 => Crfls::Value1,
            1 => Crfls::Value2,
            2 => Crfls::Value3,
            3 => Crfls::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Crfls::Value1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Crfls::Value2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Crfls::Value3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Crfls::Value4
    }
}
#[doc = "Field `CRFLS` writer - Comparator rise/fall interrupt line selection"]
pub type CrflsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Crfls>;
impl<'a, REG> CrflsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Crfls::Value1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Crfls::Value2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Crfls::Value3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Crfls::Value4)
    }
}
#[doc = "Comparator clamped state interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csls {
    #[doc = "0: CSGySR0"]
    Value1 = 0,
    #[doc = "1: CSGySR1"]
    Value2 = 1,
    #[doc = "2: CSGySR2"]
    Value3 = 2,
    #[doc = "3: CSGySR3"]
    Value4 = 3,
}
impl From<Csls> for u8 {
    #[inline(always)]
    fn from(variant: Csls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csls {
    type Ux = u8;
}
#[doc = "Field `CSLS` reader - Comparator clamped state interrupt line selection"]
pub type CslsR = crate::FieldReader<Csls>;
impl CslsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csls {
        match self.bits {
            0 => Csls::Value1,
            1 => Csls::Value2,
            2 => Csls::Value3,
            3 => Csls::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Csls::Value1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Csls::Value2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Csls::Value3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Csls::Value4
    }
}
#[doc = "Field `CSLS` writer - Comparator clamped state interrupt line selection"]
pub type CslsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Csls>;
impl<'a, REG> CslsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Csls::Value1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Csls::Value2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Csls::Value3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Csls::Value4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
    #[inline(always)]
    pub fn vls1s(&self) -> Vls1sR {
        Vls1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
    #[inline(always)]
    pub fn vls2s(&self) -> Vls2sR {
        Vls2sR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Conversion trigger interrupt line selection"]
    #[inline(always)]
    pub fn trls(&self) -> TrlsR {
        TrlsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Start/Stop trigger interrupt line selection"]
    #[inline(always)]
    pub fn ssls(&self) -> SslsR {
        SslsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Shadow transfer done interrupt line selection"]
    #[inline(always)]
    pub fn stls(&self) -> StlsR {
        StlsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Comparator rise/fall interrupt line selection"]
    #[inline(always)]
    pub fn crfls(&self) -> CrflsR {
        CrflsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Comparator clamped state interrupt line selection"]
    #[inline(always)]
    pub fn csls(&self) -> CslsR {
        CslsR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn vls1s(&mut self) -> Vls1sW<SrsSpec> {
        Vls1sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn vls2s(&mut self) -> Vls2sW<SrsSpec> {
        Vls2sW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Conversion trigger interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn trls(&mut self) -> TrlsW<SrsSpec> {
        TrlsW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Start/Stop trigger interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn ssls(&mut self) -> SslsW<SrsSpec> {
        SslsW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Shadow transfer done interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn stls(&mut self) -> StlsW<SrsSpec> {
        StlsW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Comparator rise/fall interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn crfls(&mut self) -> CrflsW<SrsSpec> {
        CrflsW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Comparator clamped state interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn csls(&mut self) -> CslsW<SrsSpec> {
        CslsW::new(self, 12)
    }
}
#[doc = "Service request line selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrsSpec;
impl crate::RegisterSpec for SrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srs::R`](R) reader structure"]
impl crate::Readable for SrsSpec {}
#[doc = "`write(|w| ..)` method takes [`srs::W`](W) writer structure"]
impl crate::Writable for SrsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRS to value 0"]
impl crate::Resettable for SrsSpec {
    const RESET_VALUE: u32 = 0;
}
