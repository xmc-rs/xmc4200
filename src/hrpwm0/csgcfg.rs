#[doc = "Register `CSGCFG` reader"]
pub type R = crate::R<CsgcfgSpec>;
#[doc = "Register `CSGCFG` writer"]
pub type W = crate::W<CsgcfgSpec>;
#[doc = "CSG0 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0pm {
    #[doc = "0: CSG0 unit is powered OFF"]
    Value1 = 0,
    #[doc = "1: CSG0 unit is set in Low Speed Mode"]
    Value2 = 1,
    #[doc = "3: CSG0 unit is set in High Speed Mode"]
    Value4 = 3,
}
impl From<C0pm> for u8 {
    #[inline(always)]
    fn from(variant: C0pm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0pm {
    type Ux = u8;
}
impl crate::IsEnum for C0pm {}
#[doc = "Field `C0PM` reader - CSG0 Power Mode"]
pub type C0pmR = crate::FieldReader<C0pm>;
impl C0pmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C0pm> {
        match self.bits {
            0 => Some(C0pm::Value1),
            1 => Some(C0pm::Value2),
            3 => Some(C0pm::Value4),
            _ => None,
        }
    }
    #[doc = "CSG0 unit is powered OFF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0pm::Value1
    }
    #[doc = "CSG0 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0pm::Value2
    }
    #[doc = "CSG0 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C0pm::Value4
    }
}
#[doc = "Field `C0PM` writer - CSG0 Power Mode"]
pub type C0pmW<'a, REG> = crate::FieldWriter<'a, REG, 2, C0pm>;
impl<'a, REG> C0pmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSG0 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0pm::Value1)
    }
    #[doc = "CSG0 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0pm::Value2)
    }
    #[doc = "CSG0 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C0pm::Value4)
    }
}
#[doc = "CSG1 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1pm {
    #[doc = "0: CSG1 unit is powered OFF"]
    Value1 = 0,
    #[doc = "1: CSG1 unit is set in Low Speed Mode"]
    Value2 = 1,
    #[doc = "3: CSG1 unit is set in High Speed Mode"]
    Value4 = 3,
}
impl From<C1pm> for u8 {
    #[inline(always)]
    fn from(variant: C1pm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1pm {
    type Ux = u8;
}
impl crate::IsEnum for C1pm {}
#[doc = "Field `C1PM` reader - CSG1 Power Mode"]
pub type C1pmR = crate::FieldReader<C1pm>;
impl C1pmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C1pm> {
        match self.bits {
            0 => Some(C1pm::Value1),
            1 => Some(C1pm::Value2),
            3 => Some(C1pm::Value4),
            _ => None,
        }
    }
    #[doc = "CSG1 unit is powered OFF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1pm::Value1
    }
    #[doc = "CSG1 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1pm::Value2
    }
    #[doc = "CSG1 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C1pm::Value4
    }
}
#[doc = "Field `C1PM` writer - CSG1 Power Mode"]
pub type C1pmW<'a, REG> = crate::FieldWriter<'a, REG, 2, C1pm>;
impl<'a, REG> C1pmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSG1 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1pm::Value1)
    }
    #[doc = "CSG1 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1pm::Value2)
    }
    #[doc = "CSG1 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C1pm::Value4)
    }
}
#[doc = "CSG2 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C2pm {
    #[doc = "0: CSG2 unit is powered OFF"]
    Value1 = 0,
    #[doc = "1: CSG2 unit is set in Low Speed Mode"]
    Value2 = 1,
    #[doc = "3: CSG2 unit is set in High Speed Mode"]
    Value4 = 3,
}
impl From<C2pm> for u8 {
    #[inline(always)]
    fn from(variant: C2pm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C2pm {
    type Ux = u8;
}
impl crate::IsEnum for C2pm {}
#[doc = "Field `C2PM` reader - CSG2 Power Mode"]
pub type C2pmR = crate::FieldReader<C2pm>;
impl C2pmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C2pm> {
        match self.bits {
            0 => Some(C2pm::Value1),
            1 => Some(C2pm::Value2),
            3 => Some(C2pm::Value4),
            _ => None,
        }
    }
    #[doc = "CSG2 unit is powered OFF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C2pm::Value1
    }
    #[doc = "CSG2 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C2pm::Value2
    }
    #[doc = "CSG2 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C2pm::Value4
    }
}
#[doc = "Field `C2PM` writer - CSG2 Power Mode"]
pub type C2pmW<'a, REG> = crate::FieldWriter<'a, REG, 2, C2pm>;
impl<'a, REG> C2pmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSG2 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C2pm::Value1)
    }
    #[doc = "CSG2 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C2pm::Value2)
    }
    #[doc = "CSG2 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C2pm::Value4)
    }
}
#[doc = "Field `C0CD` reader - CSG0 Clock disable"]
pub type C0cdR = crate::BitReader;
#[doc = "Field `C0CD` writer - CSG0 Clock disable"]
pub type C0cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1CD` reader - CSG1 Clock disable"]
pub type C1cdR = crate::BitReader;
#[doc = "Field `C1CD` writer - CSG1 Clock disable"]
pub type C1cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2CD` reader - CSG2 Clock disable"]
pub type C2cdR = crate::BitReader;
#[doc = "Field `C2CD` writer - CSG2 Clock disable"]
pub type C2cdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline(always)]
    pub fn c0pm(&self) -> C0pmR {
        C0pmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline(always)]
    pub fn c1pm(&self) -> C1pmR {
        C1pmR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline(always)]
    pub fn c2pm(&self) -> C2pmR {
        C2pmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline(always)]
    pub fn c0cd(&self) -> C0cdR {
        C0cdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline(always)]
    pub fn c1cd(&self) -> C1cdR {
        C1cdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline(always)]
    pub fn c2cd(&self) -> C2cdR {
        C2cdR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn c0pm(&mut self) -> C0pmW<CsgcfgSpec> {
        C0pmW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn c1pm(&mut self) -> C1pmW<CsgcfgSpec> {
        C1pmW::new(self, 2)
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn c2pm(&mut self) -> C2pmW<CsgcfgSpec> {
        C2pmW::new(self, 4)
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn c0cd(&mut self) -> C0cdW<CsgcfgSpec> {
        C0cdW::new(self, 16)
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn c1cd(&mut self) -> C1cdW<CsgcfgSpec> {
        C1cdW::new(self, 17)
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn c2cd(&mut self) -> C2cdW<CsgcfgSpec> {
        C2cdW::new(self, 18)
    }
}
#[doc = "Global CSG configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsgcfgSpec;
impl crate::RegisterSpec for CsgcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcfg::R`](R) reader structure"]
impl crate::Readable for CsgcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcfg::W`](W) writer structure"]
impl crate::Writable for CsgcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCFG to value 0"]
impl crate::Resettable for CsgcfgSpec {
    const RESET_VALUE: u32 = 0;
}
