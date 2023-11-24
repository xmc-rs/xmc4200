#[doc = "Register `PLC` reader"]
pub type R = crate::R<PLC_SPEC>;
#[doc = "Register `PLC` writer"]
pub type W = crate::W<PLC_SPEC>;
#[doc = "Field `IPLS` reader - Clamping control signal selector"]
pub type IPLS_R = crate::FieldReader<IPLS_A>;
#[doc = "Clamping control signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPLS_A {
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
impl From<IPLS_A> for u8 {
    #[inline(always)]
    fn from(variant: IPLS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IPLS_A {
    type Ux = u8;
}
impl IPLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IPLS_A {
        match self.bits {
            0 => IPLS_A::VALUE1,
            1 => IPLS_A::VALUE2,
            2 => IPLS_A::VALUE3,
            3 => IPLS_A::VALUE4,
            4 => IPLS_A::VALUE5,
            5 => IPLS_A::VALUE6,
            6 => IPLS_A::VALUE7,
            7 => IPLS_A::VALUE8,
            8 => IPLS_A::VALUE9,
            9 => IPLS_A::VALUE10,
            10 => IPLS_A::VALUE11,
            11 => IPLS_A::VALUE12,
            12 => IPLS_A::VALUE13,
            13 => IPLS_A::VALUE14,
            14 => IPLS_A::VALUE15,
            15 => IPLS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "HRPWMx.BLyA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPLS_A::VALUE1
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPLS_A::VALUE2
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == IPLS_A::VALUE3
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == IPLS_A::VALUE4
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == IPLS_A::VALUE5
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == IPLS_A::VALUE6
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == IPLS_A::VALUE7
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == IPLS_A::VALUE8
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == IPLS_A::VALUE9
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == IPLS_A::VALUE10
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == IPLS_A::VALUE11
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == IPLS_A::VALUE12
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == IPLS_A::VALUE13
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == IPLS_A::VALUE14
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == IPLS_A::VALUE15
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == IPLS_A::VALUE16
    }
}
#[doc = "Field `IPLS` writer - Clamping control signal selector"]
pub type IPLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, IPLS_A>;
impl<'a, REG> IPLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HRPWMx.BLyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE1)
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE2)
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE3)
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE4)
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE5)
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE6)
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE7)
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE8)
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE9)
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE10)
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE11)
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE12)
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE13)
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE14)
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE15)
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(IPLS_A::VALUE16)
    }
}
#[doc = "Field `PLCL` reader - Clamping control signal level selection"]
pub type PLCL_R = crate::FieldReader<PLCL_A>;
#[doc = "Clamping control signal level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLCL_A {
    #[doc = "0: Clamping control disabled"]
    VALUE1 = 0,
    #[doc = "1: Output is set to clamped level when the control signal is HIGH"]
    VALUE2 = 1,
    #[doc = "2: Output is set to clamped level when the control signal is LOW"]
    VALUE3 = 2,
}
impl From<PLCL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLCL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLCL_A {
    type Ux = u8;
}
impl PLCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLCL_A> {
        match self.bits {
            0 => Some(PLCL_A::VALUE1),
            1 => Some(PLCL_A::VALUE2),
            2 => Some(PLCL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Clamping control disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLCL_A::VALUE1
    }
    #[doc = "Output is set to clamped level when the control signal is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLCL_A::VALUE2
    }
    #[doc = "Output is set to clamped level when the control signal is LOW"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PLCL_A::VALUE3
    }
}
#[doc = "Field `PLCL` writer - Clamping control signal level selection"]
pub type PLCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLCL_A>;
impl<'a, REG> PLCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clamping control disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PLCL_A::VALUE1)
    }
    #[doc = "Output is set to clamped level when the control signal is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PLCL_A::VALUE2)
    }
    #[doc = "Output is set to clamped level when the control signal is LOW"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PLCL_A::VALUE3)
    }
}
#[doc = "Field `PSL` reader - Output passive level value"]
pub type PSL_R = crate::BitReader<PSL_A>;
#[doc = "Output passive level value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSL_A {
    #[doc = "0: Output clamped level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Output clamped level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL_A> for bool {
    #[inline(always)]
    fn from(variant: PSL_A) -> Self {
        variant as u8 != 0
    }
}
impl PSL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSL_A {
        match self.bits {
            false => PSL_A::VALUE1,
            true => PSL_A::VALUE2,
        }
    }
    #[doc = "Output clamped level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL_A::VALUE1
    }
    #[doc = "Output clamped level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL_A::VALUE2
    }
}
#[doc = "Field `PSL` writer - Output passive level value"]
pub type PSL_W<'a, REG> = crate::BitWriter<'a, REG, PSL_A>;
impl<'a, REG> PSL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output clamped level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PSL_A::VALUE1)
    }
    #[doc = "Output clamped level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PSL_A::VALUE2)
    }
}
#[doc = "Field `PLSW` reader - Clamped state exit SW configuration"]
pub type PLSW_R = crate::BitReader<PLSW_A>;
#[doc = "Clamped state exit SW configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLSW_A {
    #[doc = "0: External signal and SW can remove the output from the clamped state"]
    VALUE1 = 0,
    #[doc = "1: Only SW can remove the output from the clamped state"]
    VALUE2 = 1,
}
impl From<PLSW_A> for bool {
    #[inline(always)]
    fn from(variant: PLSW_A) -> Self {
        variant as u8 != 0
    }
}
impl PLSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLSW_A {
        match self.bits {
            false => PLSW_A::VALUE1,
            true => PLSW_A::VALUE2,
        }
    }
    #[doc = "External signal and SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLSW_A::VALUE1
    }
    #[doc = "Only SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLSW_A::VALUE2
    }
}
#[doc = "Field `PLSW` writer - Clamped state exit SW configuration"]
pub type PLSW_W<'a, REG> = crate::BitWriter<'a, REG, PLSW_A>;
impl<'a, REG> PLSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External signal and SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PLSW_A::VALUE1)
    }
    #[doc = "Only SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PLSW_A::VALUE2)
    }
}
#[doc = "Field `PLEC` reader - Passive level enter configuration"]
pub type PLEC_R = crate::FieldReader<PLEC_A>;
#[doc = "Passive level enter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLEC_A {
    #[doc = "0: Passive level is entered immediately"]
    VALUE1 = 0,
    #[doc = "1: Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2 = 1,
    #[doc = "2: Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3 = 2,
}
impl From<PLEC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLEC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLEC_A {
    type Ux = u8;
}
impl PLEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLEC_A> {
        match self.bits {
            0 => Some(PLEC_A::VALUE1),
            1 => Some(PLEC_A::VALUE2),
            2 => Some(PLEC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Passive level is entered immediately"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLEC_A::VALUE1
    }
    #[doc = "Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLEC_A::VALUE2
    }
    #[doc = "Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PLEC_A::VALUE3
    }
}
#[doc = "Field `PLEC` writer - Passive level enter configuration"]
pub type PLEC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLEC_A>;
impl<'a, REG> PLEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Passive level is entered immediately"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PLEC_A::VALUE1)
    }
    #[doc = "Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PLEC_A::VALUE2)
    }
    #[doc = "Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PLEC_A::VALUE3)
    }
}
#[doc = "Field `PLXC` reader - Passive level exit configuration"]
pub type PLXC_R = crate::FieldReader<PLXC_A>;
#[doc = "Passive level exit configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLXC_A {
    #[doc = "0: Passive level is exit immediately"]
    VALUE1 = 0,
    #[doc = "1: Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2 = 1,
    #[doc = "2: Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3 = 2,
}
impl From<PLXC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLXC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLXC_A {
    type Ux = u8;
}
impl PLXC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLXC_A> {
        match self.bits {
            0 => Some(PLXC_A::VALUE1),
            1 => Some(PLXC_A::VALUE2),
            2 => Some(PLXC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Passive level is exit immediately"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLXC_A::VALUE1
    }
    #[doc = "Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLXC_A::VALUE2
    }
    #[doc = "Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PLXC_A::VALUE3
    }
}
#[doc = "Field `PLXC` writer - Passive level exit configuration"]
pub type PLXC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLXC_A>;
impl<'a, REG> PLXC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Passive level is exit immediately"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PLXC_A::VALUE1)
    }
    #[doc = "Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PLXC_A::VALUE2)
    }
    #[doc = "Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PLXC_A::VALUE3)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clamping control signal selector"]
    #[inline(always)]
    pub fn ipls(&self) -> IPLS_R {
        IPLS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Clamping control signal level selection"]
    #[inline(always)]
    pub fn plcl(&self) -> PLCL_R {
        PLCL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output passive level value"]
    #[inline(always)]
    pub fn psl(&self) -> PSL_R {
        PSL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clamped state exit SW configuration"]
    #[inline(always)]
    pub fn plsw(&self) -> PLSW_R {
        PLSW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Passive level enter configuration"]
    #[inline(always)]
    pub fn plec(&self) -> PLEC_R {
        PLEC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Passive level exit configuration"]
    #[inline(always)]
    pub fn plxc(&self) -> PLXC_R {
        PLXC_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clamping control signal selector"]
    #[inline(always)]
    #[must_use]
    pub fn ipls(&mut self) -> IPLS_W<PLC_SPEC> {
        IPLS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Clamping control signal level selection"]
    #[inline(always)]
    #[must_use]
    pub fn plcl(&mut self) -> PLCL_W<PLC_SPEC> {
        PLCL_W::new(self, 8)
    }
    #[doc = "Bit 10 - Output passive level value"]
    #[inline(always)]
    #[must_use]
    pub fn psl(&mut self) -> PSL_W<PLC_SPEC> {
        PSL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clamped state exit SW configuration"]
    #[inline(always)]
    #[must_use]
    pub fn plsw(&mut self) -> PLSW_W<PLC_SPEC> {
        PLSW_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Passive level enter configuration"]
    #[inline(always)]
    #[must_use]
    pub fn plec(&mut self) -> PLEC_W<PLC_SPEC> {
        PLEC_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Passive level exit configuration"]
    #[inline(always)]
    #[must_use]
    pub fn plxc(&mut self) -> PLXC_W<PLC_SPEC> {
        PLXC_W::new(self, 14)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Passive level configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLC_SPEC;
impl crate::RegisterSpec for PLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plc::R`](R) reader structure"]
impl crate::Readable for PLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plc::W`](W) writer structure"]
impl crate::Writable for PLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLC to value 0"]
impl crate::Resettable for PLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
