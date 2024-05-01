#[doc = "Register `CC` reader"]
pub type R = crate::R<CcSpec>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CcSpec>;
#[doc = "External blanking trigger selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ibs {
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
impl From<Ibs> for u8 {
    #[inline(always)]
    fn from(variant: Ibs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ibs {
    type Ux = u8;
}
impl crate::IsEnum for Ibs {}
#[doc = "Field `IBS` reader - External blanking trigger selector"]
pub type IbsR = crate::FieldReader<Ibs>;
impl IbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ibs {
        match self.bits {
            0 => Ibs::Value1,
            1 => Ibs::Value2,
            2 => Ibs::Value3,
            3 => Ibs::Value4,
            4 => Ibs::Value5,
            5 => Ibs::Value6,
            6 => Ibs::Value7,
            7 => Ibs::Value8,
            8 => Ibs::Value9,
            9 => Ibs::Value10,
            10 => Ibs::Value11,
            11 => Ibs::Value12,
            12 => Ibs::Value13,
            13 => Ibs::Value14,
            14 => Ibs::Value15,
            15 => Ibs::Value16,
            _ => unreachable!(),
        }
    }
    #[doc = "HRPWMx.BLyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ibs::Value1
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ibs::Value2
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ibs::Value3
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ibs::Value4
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Ibs::Value5
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Ibs::Value6
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Ibs::Value7
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Ibs::Value8
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Ibs::Value9
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Ibs::Value10
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Ibs::Value11
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Ibs::Value12
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Ibs::Value13
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Ibs::Value14
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Ibs::Value15
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Ibs::Value16
    }
}
#[doc = "Field `IBS` writer - External blanking trigger selector"]
pub type IbsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ibs, crate::Safe>;
impl<'a, REG> IbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HRPWMx.BLyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value1)
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value2)
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value3)
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value4)
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value5)
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value6)
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value7)
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value8)
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value9)
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value10)
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value11)
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value12)
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value13)
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value14)
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value15)
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Ibs::Value16)
    }
}
#[doc = "Inverting comparator input selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imcs {
    #[doc = "0: HRPWMx.CyINA"]
    Value1 = 0,
    #[doc = "1: HRPWMx.CyINB"]
    Value2 = 1,
}
impl From<Imcs> for bool {
    #[inline(always)]
    fn from(variant: Imcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMCS` reader - Inverting comparator input selector"]
pub type ImcsR = crate::BitReader<Imcs>;
impl ImcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imcs {
        match self.bits {
            false => Imcs::Value1,
            true => Imcs::Value2,
        }
    }
    #[doc = "HRPWMx.CyINA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Imcs::Value1
    }
    #[doc = "HRPWMx.CyINB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Imcs::Value2
    }
}
#[doc = "Field `IMCS` writer - Inverting comparator input selector"]
pub type ImcsW<'a, REG> = crate::BitWriter<'a, REG, Imcs>;
impl<'a, REG> ImcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.CyINA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Imcs::Value1)
    }
    #[doc = "HRPWMx.CyINB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Imcs::Value2)
    }
}
#[doc = "Comparator input switching configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Imcc {
    #[doc = "0: Dynamic switch disabled"]
    Value1 = 0,
    #[doc = "1: Comparator input is connected to HRPWMx.CyINB when the control signal is HIGH"]
    Value2 = 1,
    #[doc = "2: Comparator input is connected to HRPWMx.CyINA when the control signal is HIGH"]
    Value3 = 2,
}
impl From<Imcc> for u8 {
    #[inline(always)]
    fn from(variant: Imcc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Imcc {
    type Ux = u8;
}
impl crate::IsEnum for Imcc {}
#[doc = "Field `IMCC` reader - Comparator input switching configuration"]
pub type ImccR = crate::FieldReader<Imcc>;
impl ImccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Imcc> {
        match self.bits {
            0 => Some(Imcc::Value1),
            1 => Some(Imcc::Value2),
            2 => Some(Imcc::Value3),
            _ => None,
        }
    }
    #[doc = "Dynamic switch disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Imcc::Value1
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINB when the control signal is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Imcc::Value2
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINA when the control signal is HIGH"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Imcc::Value3
    }
}
#[doc = "Field `IMCC` writer - Comparator input switching configuration"]
pub type ImccW<'a, REG> = crate::FieldWriter<'a, REG, 2, Imcc>;
impl<'a, REG> ImccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dynamic switch disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Imcc::Value1)
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINB when the control signal is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Imcc::Value2)
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINA when the control signal is HIGH"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Imcc::Value3)
    }
}
#[doc = "Field `ESE` reader - External triggered switch enable"]
pub type EseR = crate::BitReader;
#[doc = "Field `ESE` writer - External triggered switch enable"]
pub type EseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIE` reader - Comparator output inversion enable"]
pub type OieR = crate::BitReader;
#[doc = "Field `OIE` writer - Comparator output inversion enable"]
pub type OieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSE` reader - Comparator output synchronization enable"]
pub type OseR = crate::BitReader;
#[doc = "Field `OSE` writer - Comparator output synchronization enable"]
pub type OseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Blanking mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blmc {
    #[doc = "0: Blanking disabled"]
    Value1 = 0,
    #[doc = "1: Blanking on a LOW to HIGH transition"]
    Value2 = 1,
    #[doc = "2: Blanking on a HIGH to LOW transition"]
    Value3 = 2,
    #[doc = "3: Blanking on both transitions"]
    Value4 = 3,
}
impl From<Blmc> for u8 {
    #[inline(always)]
    fn from(variant: Blmc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blmc {
    type Ux = u8;
}
impl crate::IsEnum for Blmc {}
#[doc = "Field `BLMC` reader - Blanking mode"]
pub type BlmcR = crate::FieldReader<Blmc>;
impl BlmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blmc {
        match self.bits {
            0 => Blmc::Value1,
            1 => Blmc::Value2,
            2 => Blmc::Value3,
            3 => Blmc::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Blanking disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Blmc::Value1
    }
    #[doc = "Blanking on a LOW to HIGH transition"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Blmc::Value2
    }
    #[doc = "Blanking on a HIGH to LOW transition"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Blmc::Value3
    }
    #[doc = "Blanking on both transitions"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Blmc::Value4
    }
}
#[doc = "Field `BLMC` writer - Blanking mode"]
pub type BlmcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Blmc, crate::Safe>;
impl<'a, REG> BlmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blanking disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Blmc::Value1)
    }
    #[doc = "Blanking on a LOW to HIGH transition"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Blmc::Value2)
    }
    #[doc = "Blanking on a HIGH to LOW transition"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Blmc::Value3)
    }
    #[doc = "Blanking on both transitions"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Blmc::Value4)
    }
}
#[doc = "Field `EBE` reader - External blanking trigger enabled"]
pub type EbeR = crate::BitReader;
#[doc = "Field `EBE` writer - External blanking trigger enabled"]
pub type EbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comparator output filter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cofe {
    #[doc = "0: Filtering stage disabled"]
    Value1 = 0,
    #[doc = "1: Filtering stage enabled"]
    Value2 = 1,
}
impl From<Cofe> for bool {
    #[inline(always)]
    fn from(variant: Cofe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COFE` reader - Comparator output filter enable"]
pub type CofeR = crate::BitReader<Cofe>;
impl CofeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cofe {
        match self.bits {
            false => Cofe::Value1,
            true => Cofe::Value2,
        }
    }
    #[doc = "Filtering stage disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cofe::Value1
    }
    #[doc = "Filtering stage enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cofe::Value2
    }
}
#[doc = "Field `COFE` writer - Comparator output filter enable"]
pub type CofeW<'a, REG> = crate::BitWriter<'a, REG, Cofe>;
impl<'a, REG> CofeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filtering stage disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cofe::Value1)
    }
    #[doc = "Filtering stage enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cofe::Value2)
    }
}
#[doc = "Comparator output filter window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cofm {
    #[doc = "0: Comparator Output needs to be stable for 2 clock cycles"]
    Value1 = 0,
    #[doc = "1: Comparator Output needs to be stable for 3 clock cycles"]
    Value2 = 1,
    #[doc = "2: Comparator Output needs to be stable for 4 clock cycles"]
    Value3 = 2,
    #[doc = "3: Comparator Output needs to be stable for 5 clock cycles"]
    Value4 = 3,
    #[doc = "12: Comparator Output needs to be stable for 14 clock cycles"]
    Value5 = 12,
    #[doc = "13: Comparator Output needs to be stable for 15 clock cycles"]
    Value6 = 13,
    #[doc = "14: Comparator Output needs to be stable for 16 clock cycles"]
    Value7 = 14,
    #[doc = "15: Comparator Output needs to be stable for 32 clock cycles"]
    Value8 = 15,
}
impl From<Cofm> for u8 {
    #[inline(always)]
    fn from(variant: Cofm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cofm {
    type Ux = u8;
}
impl crate::IsEnum for Cofm {}
#[doc = "Field `COFM` reader - Comparator output filter window"]
pub type CofmR = crate::FieldReader<Cofm>;
impl CofmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cofm> {
        match self.bits {
            0 => Some(Cofm::Value1),
            1 => Some(Cofm::Value2),
            2 => Some(Cofm::Value3),
            3 => Some(Cofm::Value4),
            12 => Some(Cofm::Value5),
            13 => Some(Cofm::Value6),
            14 => Some(Cofm::Value7),
            15 => Some(Cofm::Value8),
            _ => None,
        }
    }
    #[doc = "Comparator Output needs to be stable for 2 clock cycles"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cofm::Value1
    }
    #[doc = "Comparator Output needs to be stable for 3 clock cycles"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cofm::Value2
    }
    #[doc = "Comparator Output needs to be stable for 4 clock cycles"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cofm::Value3
    }
    #[doc = "Comparator Output needs to be stable for 5 clock cycles"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cofm::Value4
    }
    #[doc = "Comparator Output needs to be stable for 14 clock cycles"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Cofm::Value5
    }
    #[doc = "Comparator Output needs to be stable for 15 clock cycles"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Cofm::Value6
    }
    #[doc = "Comparator Output needs to be stable for 16 clock cycles"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Cofm::Value7
    }
    #[doc = "Comparator Output needs to be stable for 32 clock cycles"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Cofm::Value8
    }
}
#[doc = "Field `COFM` writer - Comparator output filter window"]
pub type CofmW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cofm>;
impl<'a, REG> CofmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator Output needs to be stable for 2 clock cycles"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cofm::Value1)
    }
    #[doc = "Comparator Output needs to be stable for 3 clock cycles"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cofm::Value2)
    }
    #[doc = "Comparator Output needs to be stable for 4 clock cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cofm::Value3)
    }
    #[doc = "Comparator Output needs to be stable for 5 clock cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cofm::Value4)
    }
    #[doc = "Comparator Output needs to be stable for 14 clock cycles"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Cofm::Value5)
    }
    #[doc = "Comparator Output needs to be stable for 15 clock cycles"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Cofm::Value6)
    }
    #[doc = "Comparator Output needs to be stable for 16 clock cycles"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Cofm::Value7)
    }
    #[doc = "Comparator Output needs to be stable for 32 clock cycles"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Cofm::Value8)
    }
}
#[doc = "Comparator output filter control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cofc {
    #[doc = "0: Filtering is always done if enabled"]
    Value1 = 0,
    #[doc = "1: Filtering is only done when CSGyDSV1 value is currently fed to the DAC"]
    Value2 = 1,
    #[doc = "2: Filtering is only done when the CSGyDSV2 value is currently fed to the DAC"]
    Value3 = 2,
}
impl From<Cofc> for u8 {
    #[inline(always)]
    fn from(variant: Cofc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cofc {
    type Ux = u8;
}
impl crate::IsEnum for Cofc {}
#[doc = "Field `COFC` reader - Comparator output filter control"]
pub type CofcR = crate::FieldReader<Cofc>;
impl CofcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cofc> {
        match self.bits {
            0 => Some(Cofc::Value1),
            1 => Some(Cofc::Value2),
            2 => Some(Cofc::Value3),
            _ => None,
        }
    }
    #[doc = "Filtering is always done if enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cofc::Value1
    }
    #[doc = "Filtering is only done when CSGyDSV1 value is currently fed to the DAC"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cofc::Value2
    }
    #[doc = "Filtering is only done when the CSGyDSV2 value is currently fed to the DAC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cofc::Value3
    }
}
#[doc = "Field `COFC` writer - Comparator output filter control"]
pub type CofcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cofc>;
impl<'a, REG> CofcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filtering is always done if enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cofc::Value1)
    }
    #[doc = "Filtering is only done when CSGyDSV1 value is currently fed to the DAC"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cofc::Value2)
    }
    #[doc = "Filtering is only done when the CSGyDSV2 value is currently fed to the DAC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cofc::Value3)
    }
}
impl R {
    #[doc = "Bits 0:3 - External blanking trigger selector"]
    #[inline(always)]
    pub fn ibs(&self) -> IbsR {
        IbsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Inverting comparator input selector"]
    #[inline(always)]
    pub fn imcs(&self) -> ImcsR {
        ImcsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Comparator input switching configuration"]
    #[inline(always)]
    pub fn imcc(&self) -> ImccR {
        ImccR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - External triggered switch enable"]
    #[inline(always)]
    pub fn ese(&self) -> EseR {
        EseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator output inversion enable"]
    #[inline(always)]
    pub fn oie(&self) -> OieR {
        OieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparator output synchronization enable"]
    #[inline(always)]
    pub fn ose(&self) -> OseR {
        OseR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Blanking mode"]
    #[inline(always)]
    pub fn blmc(&self) -> BlmcR {
        BlmcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - External blanking trigger enabled"]
    #[inline(always)]
    pub fn ebe(&self) -> EbeR {
        EbeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparator output filter enable"]
    #[inline(always)]
    pub fn cofe(&self) -> CofeR {
        CofeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - Comparator output filter window"]
    #[inline(always)]
    pub fn cofm(&self) -> CofmR {
        CofmR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Comparator output filter control"]
    #[inline(always)]
    pub fn cofc(&self) -> CofcR {
        CofcR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External blanking trigger selector"]
    #[inline(always)]
    #[must_use]
    pub fn ibs(&mut self) -> IbsW<CcSpec> {
        IbsW::new(self, 0)
    }
    #[doc = "Bit 8 - Inverting comparator input selector"]
    #[inline(always)]
    #[must_use]
    pub fn imcs(&mut self) -> ImcsW<CcSpec> {
        ImcsW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Comparator input switching configuration"]
    #[inline(always)]
    #[must_use]
    pub fn imcc(&mut self) -> ImccW<CcSpec> {
        ImccW::new(self, 9)
    }
    #[doc = "Bit 11 - External triggered switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn ese(&mut self) -> EseW<CcSpec> {
        EseW::new(self, 11)
    }
    #[doc = "Bit 12 - Comparator output inversion enable"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OieW<CcSpec> {
        OieW::new(self, 12)
    }
    #[doc = "Bit 13 - Comparator output synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn ose(&mut self) -> OseW<CcSpec> {
        OseW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Blanking mode"]
    #[inline(always)]
    #[must_use]
    pub fn blmc(&mut self) -> BlmcW<CcSpec> {
        BlmcW::new(self, 14)
    }
    #[doc = "Bit 16 - External blanking trigger enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ebe(&mut self) -> EbeW<CcSpec> {
        EbeW::new(self, 16)
    }
    #[doc = "Bit 17 - Comparator output filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cofe(&mut self) -> CofeW<CcSpec> {
        CofeW::new(self, 17)
    }
    #[doc = "Bits 18:21 - Comparator output filter window"]
    #[inline(always)]
    #[must_use]
    pub fn cofm(&mut self) -> CofmW<CcSpec> {
        CofmW::new(self, 18)
    }
    #[doc = "Bits 24:25 - Comparator output filter control"]
    #[inline(always)]
    #[must_use]
    pub fn cofc(&mut self) -> CofcW<CcSpec> {
        CofcW::new(self, 24)
    }
}
#[doc = "Comparator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSpec;
impl crate::RegisterSpec for CcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CcSpec {
    const RESET_VALUE: u32 = 0;
}
