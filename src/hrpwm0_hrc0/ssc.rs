#[doc = "Reader of register SSC"]
pub type R = crate::R<u32, super::SSC>;
#[doc = "Writer for register SSC"]
pub type W = crate::W<u32, super::SSC>;
#[doc = "Register SSC `reset()`'s with value 0"]
impl crate::ResetValue for super::SSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Source selector for the shadow transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SST_A {
    #[doc = "0: Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    VALUE1 = 0,
    #[doc = "1: Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    VALUE2 = 1,
}
impl From<SST_A> for bool {
    #[inline(always)]
    fn from(variant: SST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SST`"]
pub type SST_R = crate::R<bool, SST_A>;
impl SST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SST_A {
        match self.bits {
            false => SST_A::VALUE1,
            true => SST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SST_A::VALUE2
    }
}
#[doc = "Write proxy for field `SST`"]
pub struct SST_W<'a> {
    w: &'a mut W,
}
impl<'a> SST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SST_A::VALUE1)
    }
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SST_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W {
        SST_W { w: self }
    }
}
