#[doc = "Register `HRBSC` reader"]
pub type R = crate::R<HrbscSpec>;
#[doc = "Register `HRBSC` writer"]
pub type W = crate::W<HrbscSpec>;
#[doc = "Suspend configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Suscfg {
    #[doc = "0: Suspend is ignored."]
    Value1 = 0,
    #[doc = "1: CSGy and HRCy units are halted."]
    Value2 = 1,
    #[doc = "2: Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    Value3 = 2,
    #[doc = "3: CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    Value4 = 3,
}
impl From<Suscfg> for u8 {
    #[inline(always)]
    fn from(variant: Suscfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Suscfg {
    type Ux = u8;
}
impl crate::IsEnum for Suscfg {}
#[doc = "Field `SUSCFG` reader - Suspend configuration"]
pub type SuscfgR = crate::FieldReader<Suscfg>;
impl SuscfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Suscfg> {
        match self.bits {
            0 => Some(Suscfg::Value1),
            1 => Some(Suscfg::Value2),
            2 => Some(Suscfg::Value3),
            3 => Some(Suscfg::Value4),
            _ => None,
        }
    }
    #[doc = "Suspend is ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Suscfg::Value1
    }
    #[doc = "CSGy and HRCy units are halted."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Suscfg::Value2
    }
    #[doc = "Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Suscfg::Value3
    }
    #[doc = "CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Suscfg::Value4
    }
}
#[doc = "Field `SUSCFG` writer - Suspend configuration"]
pub type SuscfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Suscfg>;
impl<'a, REG> SuscfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Suspend is ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value1)
    }
    #[doc = "CSGy and HRCy units are halted."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value2)
    }
    #[doc = "Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value3)
    }
    #[doc = "CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value4)
    }
}
#[doc = "Field `HRBE` reader - HRPWM bias enable"]
pub type HrbeR = crate::BitReader;
#[doc = "Field `HRBE` writer - HRPWM bias enable"]
pub type HrbeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SuscfgR {
        SuscfgR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline(always)]
    pub fn hrbe(&self) -> HrbeR {
        HrbeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline(always)]
    #[must_use]
    pub fn suscfg(&mut self) -> SuscfgW<HrbscSpec> {
        SuscfgW::new(self, 0)
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrbe(&mut self) -> HrbeW<HrbscSpec> {
        HrbeW::new(self, 8)
    }
}
#[doc = "Bias and suspend configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrbsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrbsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrbscSpec;
impl crate::RegisterSpec for HrbscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrbsc::R`](R) reader structure"]
impl crate::Readable for HrbscSpec {}
#[doc = "`write(|w| ..)` method takes [`hrbsc::W`](W) writer structure"]
impl crate::Writable for HrbscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRBSC to value 0"]
impl crate::Resettable for HrbscSpec {
    const RESET_VALUE: u32 = 0;
}
