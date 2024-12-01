#[doc = "Register `CC` reader"]
pub type R = crate::R<CC_SPEC>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CC_SPEC>;
#[doc = "External blanking trigger selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IBS_A {
    #[doc = "0: HRPWMx.BLyA"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.BLyB"]
    VALUE2 = 1,
    #[doc = "2: HRPWMx.BLyC"]
    VALUE3 = 2,
    #[doc = "3: HRPWMx.BLyD"]
    VALUE4 = 3,
    #[doc = "4: HRPWMx.BLyE"]
    VALUE5 = 4,
    #[doc = "5: HRPWMx.BLyF"]
    VALUE6 = 5,
    #[doc = "6: HRPWMx.BLyG"]
    VALUE7 = 6,
    #[doc = "7: HRPWMx.BLyH"]
    VALUE8 = 7,
    #[doc = "8: HRPWMx.BLyI"]
    VALUE9 = 8,
    #[doc = "9: HRPWMx.BLyJ"]
    VALUE10 = 9,
    #[doc = "10: HRPWMx.BLyK"]
    VALUE11 = 10,
    #[doc = "11: HRPWMx.BLyL"]
    VALUE12 = 11,
    #[doc = "12: HRPWMx.BLyM"]
    VALUE13 = 12,
    #[doc = "13: HRPWMx.BLyN"]
    VALUE14 = 13,
    #[doc = "14: HRPWMx.BLyO"]
    VALUE15 = 14,
    #[doc = "15: HRPWMx.BLyP"]
    VALUE16 = 15,
}
impl From<IBS_A> for u8 {
    #[inline(always)]
    fn from(variant: IBS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IBS_A {
    type Ux = u8;
}
impl crate::IsEnum for IBS_A {}
#[doc = "Field `IBS` reader - External blanking trigger selector"]
pub type IBS_R = crate::FieldReader<IBS_A>;
impl IBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IBS_A {
        match self.bits {
            0 => IBS_A::VALUE1,
            1 => IBS_A::VALUE2,
            2 => IBS_A::VALUE3,
            3 => IBS_A::VALUE4,
            4 => IBS_A::VALUE5,
            5 => IBS_A::VALUE6,
            6 => IBS_A::VALUE7,
            7 => IBS_A::VALUE8,
            8 => IBS_A::VALUE9,
            9 => IBS_A::VALUE10,
            10 => IBS_A::VALUE11,
            11 => IBS_A::VALUE12,
            12 => IBS_A::VALUE13,
            13 => IBS_A::VALUE14,
            14 => IBS_A::VALUE15,
            15 => IBS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "HRPWMx.BLyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IBS_A::VALUE1
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IBS_A::VALUE2
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == IBS_A::VALUE3
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == IBS_A::VALUE4
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == IBS_A::VALUE5
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == IBS_A::VALUE6
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == IBS_A::VALUE7
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == IBS_A::VALUE8
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == IBS_A::VALUE9
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == IBS_A::VALUE10
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == IBS_A::VALUE11
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == IBS_A::VALUE12
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == IBS_A::VALUE13
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == IBS_A::VALUE14
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == IBS_A::VALUE15
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == IBS_A::VALUE16
    }
}
#[doc = "Field `IBS` writer - External blanking trigger selector"]
pub type IBS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, IBS_A, crate::Safe>;
impl<'a, REG> IBS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HRPWMx.BLyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE1)
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE2)
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE3)
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE4)
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE5)
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE6)
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE7)
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE8)
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE9)
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE10)
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE11)
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE12)
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE13)
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE14)
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE15)
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(IBS_A::VALUE16)
    }
}
#[doc = "Inverting comparator input selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMCS_A {
    #[doc = "0: HRPWMx.CyINA"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.CyINB"]
    VALUE2 = 1,
}
impl From<IMCS_A> for bool {
    #[inline(always)]
    fn from(variant: IMCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMCS` reader - Inverting comparator input selector"]
pub type IMCS_R = crate::BitReader<IMCS_A>;
impl IMCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IMCS_A {
        match self.bits {
            false => IMCS_A::VALUE1,
            true => IMCS_A::VALUE2,
        }
    }
    #[doc = "HRPWMx.CyINA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IMCS_A::VALUE1
    }
    #[doc = "HRPWMx.CyINB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IMCS_A::VALUE2
    }
}
#[doc = "Field `IMCS` writer - Inverting comparator input selector"]
pub type IMCS_W<'a, REG> = crate::BitWriter<'a, REG, IMCS_A>;
impl<'a, REG> IMCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.CyINA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IMCS_A::VALUE1)
    }
    #[doc = "HRPWMx.CyINB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IMCS_A::VALUE2)
    }
}
#[doc = "Comparator input switching configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IMCC_A {
    #[doc = "0: Dynamic switch disabled"]
    VALUE1 = 0,
    #[doc = "1: Comparator input is connected to HRPWMx.CyINB when the control signal is HIGH"]
    VALUE2 = 1,
    #[doc = "2: Comparator input is connected to HRPWMx.CyINA when the control signal is HIGH"]
    VALUE3 = 2,
}
impl From<IMCC_A> for u8 {
    #[inline(always)]
    fn from(variant: IMCC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IMCC_A {
    type Ux = u8;
}
impl crate::IsEnum for IMCC_A {}
#[doc = "Field `IMCC` reader - Comparator input switching configuration"]
pub type IMCC_R = crate::FieldReader<IMCC_A>;
impl IMCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IMCC_A> {
        match self.bits {
            0 => Some(IMCC_A::VALUE1),
            1 => Some(IMCC_A::VALUE2),
            2 => Some(IMCC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Dynamic switch disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IMCC_A::VALUE1
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINB when the control signal is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IMCC_A::VALUE2
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINA when the control signal is HIGH"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == IMCC_A::VALUE3
    }
}
#[doc = "Field `IMCC` writer - Comparator input switching configuration"]
pub type IMCC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IMCC_A>;
impl<'a, REG> IMCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dynamic switch disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IMCC_A::VALUE1)
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINB when the control signal is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IMCC_A::VALUE2)
    }
    #[doc = "Comparator input is connected to HRPWMx.CyINA when the control signal is HIGH"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(IMCC_A::VALUE3)
    }
}
#[doc = "Field `ESE` reader - External triggered switch enable"]
pub type ESE_R = crate::BitReader;
#[doc = "Field `ESE` writer - External triggered switch enable"]
pub type ESE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIE` reader - Comparator output inversion enable"]
pub type OIE_R = crate::BitReader;
#[doc = "Field `OIE` writer - Comparator output inversion enable"]
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSE` reader - Comparator output synchronization enable"]
pub type OSE_R = crate::BitReader;
#[doc = "Field `OSE` writer - Comparator output synchronization enable"]
pub type OSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Blanking mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLMC_A {
    #[doc = "0: Blanking disabled"]
    VALUE1 = 0,
    #[doc = "1: Blanking on a LOW to HIGH transition"]
    VALUE2 = 1,
    #[doc = "2: Blanking on a HIGH to LOW transition"]
    VALUE3 = 2,
    #[doc = "3: Blanking on both transitions"]
    VALUE4 = 3,
}
impl From<BLMC_A> for u8 {
    #[inline(always)]
    fn from(variant: BLMC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLMC_A {
    type Ux = u8;
}
impl crate::IsEnum for BLMC_A {}
#[doc = "Field `BLMC` reader - Blanking mode"]
pub type BLMC_R = crate::FieldReader<BLMC_A>;
impl BLMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLMC_A {
        match self.bits {
            0 => BLMC_A::VALUE1,
            1 => BLMC_A::VALUE2,
            2 => BLMC_A::VALUE3,
            3 => BLMC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Blanking disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BLMC_A::VALUE1
    }
    #[doc = "Blanking on a LOW to HIGH transition"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BLMC_A::VALUE2
    }
    #[doc = "Blanking on a HIGH to LOW transition"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BLMC_A::VALUE3
    }
    #[doc = "Blanking on both transitions"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BLMC_A::VALUE4
    }
}
#[doc = "Field `BLMC` writer - Blanking mode"]
pub type BLMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BLMC_A, crate::Safe>;
impl<'a, REG> BLMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blanking disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BLMC_A::VALUE1)
    }
    #[doc = "Blanking on a LOW to HIGH transition"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BLMC_A::VALUE2)
    }
    #[doc = "Blanking on a HIGH to LOW transition"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(BLMC_A::VALUE3)
    }
    #[doc = "Blanking on both transitions"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(BLMC_A::VALUE4)
    }
}
#[doc = "Field `EBE` reader - External blanking trigger enabled"]
pub type EBE_R = crate::BitReader;
#[doc = "Field `EBE` writer - External blanking trigger enabled"]
pub type EBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comparator output filter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COFE_A {
    #[doc = "0: Filtering stage disabled"]
    VALUE1 = 0,
    #[doc = "1: Filtering stage enabled"]
    VALUE2 = 1,
}
impl From<COFE_A> for bool {
    #[inline(always)]
    fn from(variant: COFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COFE` reader - Comparator output filter enable"]
pub type COFE_R = crate::BitReader<COFE_A>;
impl COFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COFE_A {
        match self.bits {
            false => COFE_A::VALUE1,
            true => COFE_A::VALUE2,
        }
    }
    #[doc = "Filtering stage disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COFE_A::VALUE1
    }
    #[doc = "Filtering stage enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COFE_A::VALUE2
    }
}
#[doc = "Field `COFE` writer - Comparator output filter enable"]
pub type COFE_W<'a, REG> = crate::BitWriter<'a, REG, COFE_A>;
impl<'a, REG> COFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filtering stage disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(COFE_A::VALUE1)
    }
    #[doc = "Filtering stage enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(COFE_A::VALUE2)
    }
}
#[doc = "Comparator output filter window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COFM_A {
    #[doc = "0: Comparator Output needs to be stable for 2 clock cycles"]
    VALUE1 = 0,
    #[doc = "1: Comparator Output needs to be stable for 3 clock cycles"]
    VALUE2 = 1,
    #[doc = "2: Comparator Output needs to be stable for 4 clock cycles"]
    VALUE3 = 2,
    #[doc = "3: Comparator Output needs to be stable for 5 clock cycles"]
    VALUE4 = 3,
    #[doc = "12: Comparator Output needs to be stable for 14 clock cycles"]
    VALUE5 = 12,
    #[doc = "13: Comparator Output needs to be stable for 15 clock cycles"]
    VALUE6 = 13,
    #[doc = "14: Comparator Output needs to be stable for 16 clock cycles"]
    VALUE7 = 14,
    #[doc = "15: Comparator Output needs to be stable for 32 clock cycles"]
    VALUE8 = 15,
}
impl From<COFM_A> for u8 {
    #[inline(always)]
    fn from(variant: COFM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COFM_A {
    type Ux = u8;
}
impl crate::IsEnum for COFM_A {}
#[doc = "Field `COFM` reader - Comparator output filter window"]
pub type COFM_R = crate::FieldReader<COFM_A>;
impl COFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COFM_A> {
        match self.bits {
            0 => Some(COFM_A::VALUE1),
            1 => Some(COFM_A::VALUE2),
            2 => Some(COFM_A::VALUE3),
            3 => Some(COFM_A::VALUE4),
            12 => Some(COFM_A::VALUE5),
            13 => Some(COFM_A::VALUE6),
            14 => Some(COFM_A::VALUE7),
            15 => Some(COFM_A::VALUE8),
            _ => None,
        }
    }
    #[doc = "Comparator Output needs to be stable for 2 clock cycles"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COFM_A::VALUE1
    }
    #[doc = "Comparator Output needs to be stable for 3 clock cycles"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COFM_A::VALUE2
    }
    #[doc = "Comparator Output needs to be stable for 4 clock cycles"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == COFM_A::VALUE3
    }
    #[doc = "Comparator Output needs to be stable for 5 clock cycles"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == COFM_A::VALUE4
    }
    #[doc = "Comparator Output needs to be stable for 14 clock cycles"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == COFM_A::VALUE5
    }
    #[doc = "Comparator Output needs to be stable for 15 clock cycles"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == COFM_A::VALUE6
    }
    #[doc = "Comparator Output needs to be stable for 16 clock cycles"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == COFM_A::VALUE7
    }
    #[doc = "Comparator Output needs to be stable for 32 clock cycles"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == COFM_A::VALUE8
    }
}
#[doc = "Field `COFM` writer - Comparator output filter window"]
pub type COFM_W<'a, REG> = crate::FieldWriter<'a, REG, 4, COFM_A>;
impl<'a, REG> COFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator Output needs to be stable for 2 clock cycles"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(COFM_A::VALUE1)
    }
    #[doc = "Comparator Output needs to be stable for 3 clock cycles"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(COFM_A::VALUE2)
    }
    #[doc = "Comparator Output needs to be stable for 4 clock cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(COFM_A::VALUE3)
    }
    #[doc = "Comparator Output needs to be stable for 5 clock cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(COFM_A::VALUE4)
    }
    #[doc = "Comparator Output needs to be stable for 14 clock cycles"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(COFM_A::VALUE5)
    }
    #[doc = "Comparator Output needs to be stable for 15 clock cycles"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(COFM_A::VALUE6)
    }
    #[doc = "Comparator Output needs to be stable for 16 clock cycles"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(COFM_A::VALUE7)
    }
    #[doc = "Comparator Output needs to be stable for 32 clock cycles"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(COFM_A::VALUE8)
    }
}
#[doc = "Comparator output filter control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COFC_A {
    #[doc = "0: Filtering is always done if enabled"]
    VALUE1 = 0,
    #[doc = "1: Filtering is only done when CSGyDSV1 value is currently fed to the DAC"]
    VALUE2 = 1,
    #[doc = "2: Filtering is only done when the CSGyDSV2 value is currently fed to the DAC"]
    VALUE3 = 2,
}
impl From<COFC_A> for u8 {
    #[inline(always)]
    fn from(variant: COFC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COFC_A {
    type Ux = u8;
}
impl crate::IsEnum for COFC_A {}
#[doc = "Field `COFC` reader - Comparator output filter control"]
pub type COFC_R = crate::FieldReader<COFC_A>;
impl COFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COFC_A> {
        match self.bits {
            0 => Some(COFC_A::VALUE1),
            1 => Some(COFC_A::VALUE2),
            2 => Some(COFC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Filtering is always done if enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COFC_A::VALUE1
    }
    #[doc = "Filtering is only done when CSGyDSV1 value is currently fed to the DAC"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COFC_A::VALUE2
    }
    #[doc = "Filtering is only done when the CSGyDSV2 value is currently fed to the DAC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == COFC_A::VALUE3
    }
}
#[doc = "Field `COFC` writer - Comparator output filter control"]
pub type COFC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COFC_A>;
impl<'a, REG> COFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filtering is always done if enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(COFC_A::VALUE1)
    }
    #[doc = "Filtering is only done when CSGyDSV1 value is currently fed to the DAC"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(COFC_A::VALUE2)
    }
    #[doc = "Filtering is only done when the CSGyDSV2 value is currently fed to the DAC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(COFC_A::VALUE3)
    }
}
impl R {
    #[doc = "Bits 0:3 - External blanking trigger selector"]
    #[inline(always)]
    pub fn ibs(&self) -> IBS_R {
        IBS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Inverting comparator input selector"]
    #[inline(always)]
    pub fn imcs(&self) -> IMCS_R {
        IMCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Comparator input switching configuration"]
    #[inline(always)]
    pub fn imcc(&self) -> IMCC_R {
        IMCC_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - External triggered switch enable"]
    #[inline(always)]
    pub fn ese(&self) -> ESE_R {
        ESE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator output inversion enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparator output synchronization enable"]
    #[inline(always)]
    pub fn ose(&self) -> OSE_R {
        OSE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Blanking mode"]
    #[inline(always)]
    pub fn blmc(&self) -> BLMC_R {
        BLMC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - External blanking trigger enabled"]
    #[inline(always)]
    pub fn ebe(&self) -> EBE_R {
        EBE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparator output filter enable"]
    #[inline(always)]
    pub fn cofe(&self) -> COFE_R {
        COFE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - Comparator output filter window"]
    #[inline(always)]
    pub fn cofm(&self) -> COFM_R {
        COFM_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Comparator output filter control"]
    #[inline(always)]
    pub fn cofc(&self) -> COFC_R {
        COFC_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External blanking trigger selector"]
    #[inline(always)]
    pub fn ibs(&mut self) -> IBS_W<CC_SPEC> {
        IBS_W::new(self, 0)
    }
    #[doc = "Bit 8 - Inverting comparator input selector"]
    #[inline(always)]
    pub fn imcs(&mut self) -> IMCS_W<CC_SPEC> {
        IMCS_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Comparator input switching configuration"]
    #[inline(always)]
    pub fn imcc(&mut self) -> IMCC_W<CC_SPEC> {
        IMCC_W::new(self, 9)
    }
    #[doc = "Bit 11 - External triggered switch enable"]
    #[inline(always)]
    pub fn ese(&mut self) -> ESE_W<CC_SPEC> {
        ESE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Comparator output inversion enable"]
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W<CC_SPEC> {
        OIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Comparator output synchronization enable"]
    #[inline(always)]
    pub fn ose(&mut self) -> OSE_W<CC_SPEC> {
        OSE_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Blanking mode"]
    #[inline(always)]
    pub fn blmc(&mut self) -> BLMC_W<CC_SPEC> {
        BLMC_W::new(self, 14)
    }
    #[doc = "Bit 16 - External blanking trigger enabled"]
    #[inline(always)]
    pub fn ebe(&mut self) -> EBE_W<CC_SPEC> {
        EBE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Comparator output filter enable"]
    #[inline(always)]
    pub fn cofe(&mut self) -> COFE_W<CC_SPEC> {
        COFE_W::new(self, 17)
    }
    #[doc = "Bits 18:21 - Comparator output filter window"]
    #[inline(always)]
    pub fn cofm(&mut self) -> COFM_W<CC_SPEC> {
        COFM_W::new(self, 18)
    }
    #[doc = "Bits 24:25 - Comparator output filter control"]
    #[inline(always)]
    pub fn cofc(&mut self) -> COFC_W<CC_SPEC> {
        COFC_W::new(self, 24)
    }
}
#[doc = "Comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    const RESET_VALUE: u32 = 0;
}
