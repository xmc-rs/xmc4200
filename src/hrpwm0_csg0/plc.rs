#[doc = "Register `PLC` reader"]
pub type R = crate::R<PlcSpec>;
#[doc = "Register `PLC` writer"]
pub type W = crate::W<PlcSpec>;
#[doc = "Clamping control signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipls {
    #[doc = "0: HRPWMx.BLyA"]
    Value1 = 0,
    #[doc = "1: HRPWMx.BLyB"]
    Value2 = 1,
    #[doc = "2: HRPWMx.BLyC"]
    Value3 = 2,
    #[doc = "3: HRPWMx.BLyD"]
    Value4 = 3,
    #[doc = "4: HRPWMx.BLyE"]
    Value5 = 4,
    #[doc = "5: HRPWMx.BLyF"]
    Value6 = 5,
    #[doc = "6: HRPWMx.BLyG"]
    Value7 = 6,
    #[doc = "7: HRPWMx.BLyH"]
    Value8 = 7,
    #[doc = "8: HRPWMx.BLyI"]
    Value9 = 8,
    #[doc = "9: HRPWMx.BLyJ"]
    Value10 = 9,
    #[doc = "10: HRPWMx.BLyK"]
    Value11 = 10,
    #[doc = "11: HRPWMx.BLyL"]
    Value12 = 11,
    #[doc = "12: HRPWMx.BLyM"]
    Value13 = 12,
    #[doc = "13: HRPWMx.BLyN"]
    Value14 = 13,
    #[doc = "14: HRPWMx.BLyO"]
    Value15 = 14,
    #[doc = "15: HRPWMx.BLyP"]
    Value16 = 15,
}
impl From<Ipls> for u8 {
    #[inline(always)]
    fn from(variant: Ipls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ipls {
    type Ux = u8;
}
impl crate::IsEnum for Ipls {}
#[doc = "Field `IPLS` reader - Clamping control signal selector"]
pub type IplsR = crate::FieldReader<Ipls>;
impl IplsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipls {
        match self.bits {
            0 => Ipls::Value1,
            1 => Ipls::Value2,
            2 => Ipls::Value3,
            3 => Ipls::Value4,
            4 => Ipls::Value5,
            5 => Ipls::Value6,
            6 => Ipls::Value7,
            7 => Ipls::Value8,
            8 => Ipls::Value9,
            9 => Ipls::Value10,
            10 => Ipls::Value11,
            11 => Ipls::Value12,
            12 => Ipls::Value13,
            13 => Ipls::Value14,
            14 => Ipls::Value15,
            15 => Ipls::Value16,
            _ => unreachable!(),
        }
    }
    #[doc = "HRPWMx.BLyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ipls::Value1
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ipls::Value2
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ipls::Value3
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ipls::Value4
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Ipls::Value5
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Ipls::Value6
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Ipls::Value7
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Ipls::Value8
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Ipls::Value9
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Ipls::Value10
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Ipls::Value11
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Ipls::Value12
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Ipls::Value13
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Ipls::Value14
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Ipls::Value15
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Ipls::Value16
    }
}
#[doc = "Field `IPLS` writer - Clamping control signal selector"]
pub type IplsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ipls, crate::Safe>;
impl<'a, REG> IplsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HRPWMx.BLyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value1)
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value2)
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value3)
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value4)
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value5)
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value6)
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value7)
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value8)
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value9)
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value10)
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value11)
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value12)
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value13)
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value14)
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value15)
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Ipls::Value16)
    }
}
#[doc = "Clamping control signal level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plcl {
    #[doc = "0: Clamping control disabled"]
    Value1 = 0,
    #[doc = "1: Output is set to clamped level when the control signal is HIGH"]
    Value2 = 1,
    #[doc = "2: Output is set to clamped level when the control signal is LOW"]
    Value3 = 2,
}
impl From<Plcl> for u8 {
    #[inline(always)]
    fn from(variant: Plcl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plcl {
    type Ux = u8;
}
impl crate::IsEnum for Plcl {}
#[doc = "Field `PLCL` reader - Clamping control signal level selection"]
pub type PlclR = crate::FieldReader<Plcl>;
impl PlclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Plcl> {
        match self.bits {
            0 => Some(Plcl::Value1),
            1 => Some(Plcl::Value2),
            2 => Some(Plcl::Value3),
            _ => None,
        }
    }
    #[doc = "Clamping control disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Plcl::Value1
    }
    #[doc = "Output is set to clamped level when the control signal is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Plcl::Value2
    }
    #[doc = "Output is set to clamped level when the control signal is LOW"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Plcl::Value3
    }
}
#[doc = "Field `PLCL` writer - Clamping control signal level selection"]
pub type PlclW<'a, REG> = crate::FieldWriter<'a, REG, 2, Plcl>;
impl<'a, REG> PlclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clamping control disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Plcl::Value1)
    }
    #[doc = "Output is set to clamped level when the control signal is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Plcl::Value2)
    }
    #[doc = "Output is set to clamped level when the control signal is LOW"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Plcl::Value3)
    }
}
#[doc = "Output passive level value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psl {
    #[doc = "0: Output clamped level is LOW"]
    Value1 = 0,
    #[doc = "1: Output clamped level is HIGH"]
    Value2 = 1,
}
impl From<Psl> for bool {
    #[inline(always)]
    fn from(variant: Psl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL` reader - Output passive level value"]
pub type PslR = crate::BitReader<Psl>;
impl PslR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psl {
        match self.bits {
            false => Psl::Value1,
            true => Psl::Value2,
        }
    }
    #[doc = "Output clamped level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psl::Value1
    }
    #[doc = "Output clamped level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psl::Value2
    }
}
#[doc = "Field `PSL` writer - Output passive level value"]
pub type PslW<'a, REG> = crate::BitWriter<'a, REG, Psl>;
impl<'a, REG> PslW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output clamped level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psl::Value1)
    }
    #[doc = "Output clamped level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psl::Value2)
    }
}
#[doc = "Clamped state exit SW configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plsw {
    #[doc = "0: External signal and SW can remove the output from the clamped state"]
    Value1 = 0,
    #[doc = "1: Only SW can remove the output from the clamped state"]
    Value2 = 1,
}
impl From<Plsw> for bool {
    #[inline(always)]
    fn from(variant: Plsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLSW` reader - Clamped state exit SW configuration"]
pub type PlswR = crate::BitReader<Plsw>;
impl PlswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plsw {
        match self.bits {
            false => Plsw::Value1,
            true => Plsw::Value2,
        }
    }
    #[doc = "External signal and SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Plsw::Value1
    }
    #[doc = "Only SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Plsw::Value2
    }
}
#[doc = "Field `PLSW` writer - Clamped state exit SW configuration"]
pub type PlswW<'a, REG> = crate::BitWriter<'a, REG, Plsw>;
impl<'a, REG> PlswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External signal and SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Plsw::Value1)
    }
    #[doc = "Only SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Plsw::Value2)
    }
}
#[doc = "Passive level enter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plec {
    #[doc = "0: Passive level is entered immediately"]
    Value1 = 0,
    #[doc = "1: Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    Value2 = 1,
    #[doc = "2: Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    Value3 = 2,
}
impl From<Plec> for u8 {
    #[inline(always)]
    fn from(variant: Plec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plec {
    type Ux = u8;
}
impl crate::IsEnum for Plec {}
#[doc = "Field `PLEC` reader - Passive level enter configuration"]
pub type PlecR = crate::FieldReader<Plec>;
impl PlecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Plec> {
        match self.bits {
            0 => Some(Plec::Value1),
            1 => Some(Plec::Value2),
            2 => Some(Plec::Value3),
            _ => None,
        }
    }
    #[doc = "Passive level is entered immediately"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Plec::Value1
    }
    #[doc = "Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Plec::Value2
    }
    #[doc = "Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Plec::Value3
    }
}
#[doc = "Field `PLEC` writer - Passive level enter configuration"]
pub type PlecW<'a, REG> = crate::FieldWriter<'a, REG, 2, Plec>;
impl<'a, REG> PlecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Passive level is entered immediately"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Plec::Value1)
    }
    #[doc = "Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Plec::Value2)
    }
    #[doc = "Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Plec::Value3)
    }
}
#[doc = "Passive level exit configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plxc {
    #[doc = "0: Passive level is exit immediately"]
    Value1 = 0,
    #[doc = "1: Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    Value2 = 1,
    #[doc = "2: Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    Value3 = 2,
}
impl From<Plxc> for u8 {
    #[inline(always)]
    fn from(variant: Plxc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plxc {
    type Ux = u8;
}
impl crate::IsEnum for Plxc {}
#[doc = "Field `PLXC` reader - Passive level exit configuration"]
pub type PlxcR = crate::FieldReader<Plxc>;
impl PlxcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Plxc> {
        match self.bits {
            0 => Some(Plxc::Value1),
            1 => Some(Plxc::Value2),
            2 => Some(Plxc::Value3),
            _ => None,
        }
    }
    #[doc = "Passive level is exit immediately"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Plxc::Value1
    }
    #[doc = "Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Plxc::Value2
    }
    #[doc = "Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Plxc::Value3
    }
}
#[doc = "Field `PLXC` writer - Passive level exit configuration"]
pub type PlxcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Plxc>;
impl<'a, REG> PlxcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Passive level is exit immediately"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Plxc::Value1)
    }
    #[doc = "Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Plxc::Value2)
    }
    #[doc = "Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Plxc::Value3)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clamping control signal selector"]
    #[inline(always)]
    pub fn ipls(&self) -> IplsR {
        IplsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Clamping control signal level selection"]
    #[inline(always)]
    pub fn plcl(&self) -> PlclR {
        PlclR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output passive level value"]
    #[inline(always)]
    pub fn psl(&self) -> PslR {
        PslR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clamped state exit SW configuration"]
    #[inline(always)]
    pub fn plsw(&self) -> PlswR {
        PlswR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Passive level enter configuration"]
    #[inline(always)]
    pub fn plec(&self) -> PlecR {
        PlecR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Passive level exit configuration"]
    #[inline(always)]
    pub fn plxc(&self) -> PlxcR {
        PlxcR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clamping control signal selector"]
    #[inline(always)]
    #[must_use]
    pub fn ipls(&mut self) -> IplsW<PlcSpec> {
        IplsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Clamping control signal level selection"]
    #[inline(always)]
    #[must_use]
    pub fn plcl(&mut self) -> PlclW<PlcSpec> {
        PlclW::new(self, 8)
    }
    #[doc = "Bit 10 - Output passive level value"]
    #[inline(always)]
    #[must_use]
    pub fn psl(&mut self) -> PslW<PlcSpec> {
        PslW::new(self, 10)
    }
    #[doc = "Bit 11 - Clamped state exit SW configuration"]
    #[inline(always)]
    #[must_use]
    pub fn plsw(&mut self) -> PlswW<PlcSpec> {
        PlswW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Passive level enter configuration"]
    #[inline(always)]
    #[must_use]
    pub fn plec(&mut self) -> PlecW<PlcSpec> {
        PlecW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Passive level exit configuration"]
    #[inline(always)]
    #[must_use]
    pub fn plxc(&mut self) -> PlxcW<PlcSpec> {
        PlxcW::new(self, 14)
    }
}
#[doc = "Passive level configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlcSpec;
impl crate::RegisterSpec for PlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plc::R`](R) reader structure"]
impl crate::Readable for PlcSpec {}
#[doc = "`write(|w| ..)` method takes [`plc::W`](W) writer structure"]
impl crate::Writable for PlcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLC to value 0"]
impl crate::Resettable for PlcSpec {
    const RESET_VALUE: u32 = 0;
}
