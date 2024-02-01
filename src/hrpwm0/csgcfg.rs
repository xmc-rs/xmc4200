#[doc = "Register `CSGCFG` reader"]
pub type R = crate::R<CSGCFG_SPEC>;
#[doc = "Register `CSGCFG` writer"]
pub type W = crate::W<CSGCFG_SPEC>;
#[doc = "Field `C0PM` reader - CSG0 Power Mode"]
pub type C0PM_R = crate::FieldReader<C0PM_A>;
#[doc = "CSG0 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0PM_A {
    #[doc = "0: CSG0 unit is powered OFF"]
    VALUE1 = 0,
    #[doc = "1: CSG0 unit is set in Low Speed Mode"]
    VALUE2 = 1,
    #[doc = "3: CSG0 unit is set in High Speed Mode"]
    VALUE4 = 3,
}
impl From<C0PM_A> for u8 {
    #[inline(always)]
    fn from(variant: C0PM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0PM_A {
    type Ux = u8;
}
impl C0PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C0PM_A> {
        match self.bits {
            0 => Some(C0PM_A::VALUE1),
            1 => Some(C0PM_A::VALUE2),
            3 => Some(C0PM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "CSG0 unit is powered OFF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0PM_A::VALUE1
    }
    #[doc = "CSG0 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0PM_A::VALUE2
    }
    #[doc = "CSG0 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C0PM_A::VALUE4
    }
}
#[doc = "Field `C0PM` writer - CSG0 Power Mode"]
pub type C0PM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C0PM_A>;
impl<'a, REG> C0PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSG0 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0PM_A::VALUE1)
    }
    #[doc = "CSG0 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0PM_A::VALUE2)
    }
    #[doc = "CSG0 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C0PM_A::VALUE4)
    }
}
#[doc = "Field `C1PM` reader - CSG1 Power Mode"]
pub type C1PM_R = crate::FieldReader<C1PM_A>;
#[doc = "CSG1 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1PM_A {
    #[doc = "0: CSG1 unit is powered OFF"]
    VALUE1 = 0,
    #[doc = "1: CSG1 unit is set in Low Speed Mode"]
    VALUE2 = 1,
    #[doc = "3: CSG1 unit is set in High Speed Mode"]
    VALUE4 = 3,
}
impl From<C1PM_A> for u8 {
    #[inline(always)]
    fn from(variant: C1PM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1PM_A {
    type Ux = u8;
}
impl C1PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C1PM_A> {
        match self.bits {
            0 => Some(C1PM_A::VALUE1),
            1 => Some(C1PM_A::VALUE2),
            3 => Some(C1PM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "CSG1 unit is powered OFF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1PM_A::VALUE1
    }
    #[doc = "CSG1 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1PM_A::VALUE2
    }
    #[doc = "CSG1 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C1PM_A::VALUE4
    }
}
#[doc = "Field `C1PM` writer - CSG1 Power Mode"]
pub type C1PM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C1PM_A>;
impl<'a, REG> C1PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSG1 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1PM_A::VALUE1)
    }
    #[doc = "CSG1 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1PM_A::VALUE2)
    }
    #[doc = "CSG1 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C1PM_A::VALUE4)
    }
}
#[doc = "Field `C2PM` reader - CSG2 Power Mode"]
pub type C2PM_R = crate::FieldReader<C2PM_A>;
#[doc = "CSG2 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C2PM_A {
    #[doc = "0: CSG2 unit is powered OFF"]
    VALUE1 = 0,
    #[doc = "1: CSG2 unit is set in Low Speed Mode"]
    VALUE2 = 1,
    #[doc = "3: CSG2 unit is set in High Speed Mode"]
    VALUE4 = 3,
}
impl From<C2PM_A> for u8 {
    #[inline(always)]
    fn from(variant: C2PM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C2PM_A {
    type Ux = u8;
}
impl C2PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C2PM_A> {
        match self.bits {
            0 => Some(C2PM_A::VALUE1),
            1 => Some(C2PM_A::VALUE2),
            3 => Some(C2PM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "CSG2 unit is powered OFF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C2PM_A::VALUE1
    }
    #[doc = "CSG2 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C2PM_A::VALUE2
    }
    #[doc = "CSG2 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C2PM_A::VALUE4
    }
}
#[doc = "Field `C2PM` writer - CSG2 Power Mode"]
pub type C2PM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C2PM_A>;
impl<'a, REG> C2PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSG2 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C2PM_A::VALUE1)
    }
    #[doc = "CSG2 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C2PM_A::VALUE2)
    }
    #[doc = "CSG2 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C2PM_A::VALUE4)
    }
}
#[doc = "Field `C0CD` reader - CSG0 Clock disable"]
pub type C0CD_R = crate::BitReader;
#[doc = "Field `C0CD` writer - CSG0 Clock disable"]
pub type C0CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1CD` reader - CSG1 Clock disable"]
pub type C1CD_R = crate::BitReader;
#[doc = "Field `C1CD` writer - CSG1 Clock disable"]
pub type C1CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2CD` reader - CSG2 Clock disable"]
pub type C2CD_R = crate::BitReader;
#[doc = "Field `C2CD` writer - CSG2 Clock disable"]
pub type C2CD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline(always)]
    pub fn c0pm(&self) -> C0PM_R {
        C0PM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline(always)]
    pub fn c1pm(&self) -> C1PM_R {
        C1PM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline(always)]
    pub fn c2pm(&self) -> C2PM_R {
        C2PM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline(always)]
    pub fn c0cd(&self) -> C0CD_R {
        C0CD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline(always)]
    pub fn c1cd(&self) -> C1CD_R {
        C1CD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline(always)]
    pub fn c2cd(&self) -> C2CD_R {
        C2CD_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn c0pm(&mut self) -> C0PM_W<CSGCFG_SPEC> {
        C0PM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn c1pm(&mut self) -> C1PM_W<CSGCFG_SPEC> {
        C1PM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn c2pm(&mut self) -> C2PM_W<CSGCFG_SPEC> {
        C2PM_W::new(self, 4)
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn c0cd(&mut self) -> C0CD_W<CSGCFG_SPEC> {
        C0CD_W::new(self, 16)
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn c1cd(&mut self) -> C1CD_W<CSGCFG_SPEC> {
        C1CD_W::new(self, 17)
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn c2cd(&mut self) -> C2CD_W<CSGCFG_SPEC> {
        C2CD_W::new(self, 18)
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
#[doc = "Global CSG configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCFG_SPEC;
impl crate::RegisterSpec for CSGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcfg::R`](R) reader structure"]
impl crate::Readable for CSGCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csgcfg::W`](W) writer structure"]
impl crate::Writable for CSGCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCFG to value 0"]
impl crate::Resettable for CSGCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
