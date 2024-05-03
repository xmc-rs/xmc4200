#[doc = "Register `IES` reader"]
pub type R = crate::R<IES_SPEC>;
#[doc = "Register `IES` writer"]
pub type W = crate::W<IES_SPEC>;
#[doc = "External value switch function level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SVLS_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active when input is HIGH"]
    VALUE2 = 1,
    #[doc = "2: Active when input is LOW"]
    VALUE3 = 2,
}
impl From<SVLS_A> for u8 {
    #[inline(always)]
    fn from(variant: SVLS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SVLS_A {
    type Ux = u8;
}
impl crate::IsEnum for SVLS_A {}
#[doc = "Field `SVLS` reader - External value switch function level selection"]
pub type SVLS_R = crate::FieldReader<SVLS_A>;
impl SVLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SVLS_A> {
        match self.bits {
            0 => Some(SVLS_A::VALUE1),
            1 => Some(SVLS_A::VALUE2),
            2 => Some(SVLS_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SVLS_A::VALUE1
    }
    #[doc = "Active when input is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SVLS_A::VALUE2
    }
    #[doc = "Active when input is LOW"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SVLS_A::VALUE3
    }
}
#[doc = "Field `SVLS` writer - External value switch function level selection"]
pub type SVLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SVLS_A>;
impl<'a, REG> SVLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SVLS_A::VALUE1)
    }
    #[doc = "Active when input is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SVLS_A::VALUE2)
    }
    #[doc = "Active when input is LOW"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SVLS_A::VALUE3)
    }
}
#[doc = "External start function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STRES_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Active on both edges"]
    VALUE4 = 3,
}
impl From<STRES_A> for u8 {
    #[inline(always)]
    fn from(variant: STRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STRES_A {
    type Ux = u8;
}
impl crate::IsEnum for STRES_A {}
#[doc = "Field `STRES` reader - External start function edge selection"]
pub type STRES_R = crate::FieldReader<STRES_A>;
impl STRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STRES_A {
        match self.bits {
            0 => STRES_A::VALUE1,
            1 => STRES_A::VALUE2,
            2 => STRES_A::VALUE3,
            3 => STRES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STRES_A::VALUE1
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STRES_A::VALUE2
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STRES_A::VALUE3
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STRES_A::VALUE4
    }
}
#[doc = "Field `STRES` writer - External start function edge selection"]
pub type STRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STRES_A, crate::Safe>;
impl<'a, REG> STRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STRES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STRES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STRES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(STRES_A::VALUE4)
    }
}
#[doc = "External stop function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STPES_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Active on both edges"]
    VALUE4 = 3,
}
impl From<STPES_A> for u8 {
    #[inline(always)]
    fn from(variant: STPES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STPES_A {
    type Ux = u8;
}
impl crate::IsEnum for STPES_A {}
#[doc = "Field `STPES` reader - External stop function edge selection"]
pub type STPES_R = crate::FieldReader<STPES_A>;
impl STPES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STPES_A {
        match self.bits {
            0 => STPES_A::VALUE1,
            1 => STPES_A::VALUE2,
            2 => STPES_A::VALUE3,
            3 => STPES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STPES_A::VALUE1
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STPES_A::VALUE2
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STPES_A::VALUE3
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STPES_A::VALUE4
    }
}
#[doc = "Field `STPES` writer - External stop function edge selection"]
pub type STPES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STPES_A, crate::Safe>;
impl<'a, REG> STPES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STPES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STPES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STPES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(STPES_A::VALUE4)
    }
}
#[doc = "External trigger function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGES_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Active on both edges"]
    VALUE4 = 3,
}
impl From<TRGES_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRGES_A {
    type Ux = u8;
}
impl crate::IsEnum for TRGES_A {}
#[doc = "Field `TRGES` reader - External trigger function edge selection"]
pub type TRGES_R = crate::FieldReader<TRGES_A>;
impl TRGES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRGES_A {
        match self.bits {
            0 => TRGES_A::VALUE1,
            1 => TRGES_A::VALUE2,
            2 => TRGES_A::VALUE3,
            3 => TRGES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRGES_A::VALUE1
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRGES_A::VALUE2
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRGES_A::VALUE3
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRGES_A::VALUE4
    }
}
#[doc = "Field `TRGES` writer - External trigger function edge selection"]
pub type TRGES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRGES_A, crate::Safe>;
impl<'a, REG> TRGES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRGES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRGES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TRGES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TRGES_A::VALUE4)
    }
}
#[doc = "External shadow transfer enable edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STES_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Active on both edges"]
    VALUE4 = 3,
}
impl From<STES_A> for u8 {
    #[inline(always)]
    fn from(variant: STES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STES_A {
    type Ux = u8;
}
impl crate::IsEnum for STES_A {}
#[doc = "Field `STES` reader - External shadow transfer enable edge selection"]
pub type STES_R = crate::FieldReader<STES_A>;
impl STES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STES_A {
        match self.bits {
            0 => STES_A::VALUE1,
            1 => STES_A::VALUE2,
            2 => STES_A::VALUE3,
            3 => STES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STES_A::VALUE1
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STES_A::VALUE2
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STES_A::VALUE3
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STES_A::VALUE4
    }
}
#[doc = "Field `STES` writer - External shadow transfer enable edge selection"]
pub type STES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STES_A, crate::Safe>;
impl<'a, REG> STES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(STES_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline(always)]
    pub fn svls(&self) -> SVLS_R {
        SVLS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline(always)]
    pub fn stres(&self) -> STRES_R {
        STRES_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline(always)]
    pub fn stpes(&self) -> STPES_R {
        STPES_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline(always)]
    pub fn trges(&self) -> TRGES_R {
        TRGES_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline(always)]
    pub fn stes(&self) -> STES_R {
        STES_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline(always)]
    #[must_use]
    pub fn svls(&mut self) -> SVLS_W<IES_SPEC> {
        SVLS_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn stres(&mut self) -> STRES_W<IES_SPEC> {
        STRES_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn stpes(&mut self) -> STPES_W<IES_SPEC> {
        STPES_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn trges(&mut self) -> TRGES_W<IES_SPEC> {
        TRGES_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn stes(&mut self) -> STES_W<IES_SPEC> {
        STES_W::new(self, 8)
    }
}
#[doc = "External input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ies::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ies::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IES_SPEC;
impl crate::RegisterSpec for IES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ies::R`](R) reader structure"]
impl crate::Readable for IES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ies::W`](W) writer structure"]
impl crate::Writable for IES_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IES to value 0"]
impl crate::Resettable for IES_SPEC {
    const RESET_VALUE: u32 = 0;
}
