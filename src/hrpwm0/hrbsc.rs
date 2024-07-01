#[doc = "Register `HRBSC` reader"]
pub type R = crate::R<HRBSC_SPEC>;
#[doc = "Register `HRBSC` writer"]
pub type W = crate::W<HRBSC_SPEC>;
#[doc = "Suspend configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUSCFG_A {
    #[doc = "0: Suspend is ignored."]
    VALUE1 = 0,
    #[doc = "1: CSGy and HRCy units are halted."]
    VALUE2 = 1,
    #[doc = "2: Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE3 = 2,
    #[doc = "3: CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE4 = 3,
}
impl From<SUSCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SUSCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SUSCFG_A {
    type Ux = u8;
}
impl crate::IsEnum for SUSCFG_A {}
#[doc = "Field `SUSCFG` reader - Suspend configuration"]
pub type SUSCFG_R = crate::FieldReader<SUSCFG_A>;
impl SUSCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SUSCFG_A> {
        match self.bits {
            0 => Some(SUSCFG_A::VALUE1),
            1 => Some(SUSCFG_A::VALUE2),
            2 => Some(SUSCFG_A::VALUE3),
            3 => Some(SUSCFG_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Suspend is ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSCFG_A::VALUE1
    }
    #[doc = "CSGy and HRCy units are halted."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUSCFG_A::VALUE2
    }
    #[doc = "Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUSCFG_A::VALUE3
    }
    #[doc = "CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SUSCFG_A::VALUE4
    }
}
#[doc = "Field `SUSCFG` writer - Suspend configuration"]
pub type SUSCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SUSCFG_A>;
impl<'a, REG> SUSCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Suspend is ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSCFG_A::VALUE1)
    }
    #[doc = "CSGy and HRCy units are halted."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SUSCFG_A::VALUE2)
    }
    #[doc = "Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SUSCFG_A::VALUE3)
    }
    #[doc = "CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SUSCFG_A::VALUE4)
    }
}
#[doc = "Field `HRBE` reader - HRPWM bias enable"]
pub type HRBE_R = crate::BitReader;
#[doc = "Field `HRBE` writer - HRPWM bias enable"]
pub type HRBE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SUSCFG_R {
        SUSCFG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline(always)]
    pub fn hrbe(&self) -> HRBE_R {
        HRBE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline(always)]
    #[must_use]
    pub fn suscfg(&mut self) -> SUSCFG_W<HRBSC_SPEC> {
        SUSCFG_W::new(self, 0)
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrbe(&mut self) -> HRBE_W<HRBSC_SPEC> {
        HRBE_W::new(self, 8)
    }
}
#[doc = "Bias and suspend configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hrbsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrbsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRBSC_SPEC;
impl crate::RegisterSpec for HRBSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrbsc::R`](R) reader structure"]
impl crate::Readable for HRBSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hrbsc::W`](W) writer structure"]
impl crate::Writable for HRBSC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRBSC to value 0"]
impl crate::Resettable for HRBSC_SPEC {
    const RESET_VALUE: u32 = 0;
}
