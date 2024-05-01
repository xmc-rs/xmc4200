#[doc = "Register `MIRRALLREQ` writer"]
pub type W = crate::W<MirrallreqSpec>;
#[doc = "Mirror All Execution Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Req {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Start mirror update"]
    Value2 = 1,
}
impl From<Req> for bool {
    #[inline(always)]
    fn from(variant: Req) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQ` writer - Mirror All Execution Request"]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG, Req>;
impl<'a, REG> ReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Req::Value1)
    }
    #[doc = "Start mirror update"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Req::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Mirror All Execution Request"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<MirrallreqSpec> {
        ReqW::new(self, 0)
    }
}
#[doc = "Mirror All Request\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mirrallreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MirrallreqSpec;
impl crate::RegisterSpec for MirrallreqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mirrallreq::W`](W) writer structure"]
impl crate::Writable for MirrallreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIRRALLREQ to value 0"]
impl crate::Resettable for MirrallreqSpec {
    const RESET_VALUE: u32 = 0;
}
