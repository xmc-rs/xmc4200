#[doc = "Register `MIRRALLREQ` writer"]
pub type W = crate::W<MIRRALLREQ_SPEC>;
#[doc = "Mirror All Execution Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQ_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Start mirror update"]
    VALUE2 = 1,
}
impl From<REQ_A> for bool {
    #[inline(always)]
    fn from(variant: REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQ` writer - Mirror All Execution Request"]
pub type REQ_W<'a, REG> = crate::BitWriter<'a, REG, REQ_A>;
impl<'a, REG> REQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REQ_A::VALUE1)
    }
    #[doc = "Start mirror update"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REQ_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Mirror All Execution Request"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> REQ_W<MIRRALLREQ_SPEC> {
        REQ_W::new(self, 0)
    }
}
#[doc = "Mirror All Request\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mirrallreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIRRALLREQ_SPEC;
impl crate::RegisterSpec for MIRRALLREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mirrallreq::W`](W) writer structure"]
impl crate::Writable for MIRRALLREQ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIRRALLREQ to value 0"]
impl crate::Resettable for MIRRALLREQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
