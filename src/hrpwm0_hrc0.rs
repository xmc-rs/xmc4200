#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gc: GC,
    pl: PL,
    gsel: GSEL,
    tsel: TSEL,
    sc: SC,
    dcr: DCR,
    dcf: DCF,
    cr1: CR1,
    cr2: CR2,
    ssc: SSC,
    sdcr: SDCR,
    sdcf: SDCF,
    scr1: SCR1,
    scr2: SCR2,
}
impl RegisterBlock {
    #[doc = "0x00 - HRC mode configuration"]
    #[inline(always)]
    pub const fn gc(&self) -> &GC {
        &self.gc
    }
    #[doc = "0x04 - HRC output passive level"]
    #[inline(always)]
    pub const fn pl(&self) -> &PL {
        &self.pl
    }
    #[doc = "0x08 - HRC global control selection"]
    #[inline(always)]
    pub const fn gsel(&self) -> &GSEL {
        &self.gsel
    }
    #[doc = "0x0c - HRC timer selection"]
    #[inline(always)]
    pub const fn tsel(&self) -> &TSEL {
        &self.tsel
    }
    #[doc = "0x10 - HRC current source for shadow"]
    #[inline(always)]
    pub const fn sc(&self) -> &SC {
        &self.sc
    }
    #[doc = "0x14 - HRC dead time rising value"]
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    #[doc = "0x18 - HRC dead time falling value"]
    #[inline(always)]
    pub const fn dcf(&self) -> &DCF {
        &self.dcf
    }
    #[doc = "0x1c - HRC rising edge value"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x20 - HRC falling edge value"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x24 - HRC next source for shadow"]
    #[inline(always)]
    pub const fn ssc(&self) -> &SSC {
        &self.ssc
    }
    #[doc = "0x28 - HRC shadow dead time rising"]
    #[inline(always)]
    pub const fn sdcr(&self) -> &SDCR {
        &self.sdcr
    }
    #[doc = "0x2c - HRC shadow dead time falling"]
    #[inline(always)]
    pub const fn sdcf(&self) -> &SDCF {
        &self.sdcf
    }
    #[doc = "0x30 - HRC shadow rising edge value"]
    #[inline(always)]
    pub const fn scr1(&self) -> &SCR1 {
        &self.scr1
    }
    #[doc = "0x34 - HRC shadow falling edge value"]
    #[inline(always)]
    pub const fn scr2(&self) -> &SCR2 {
        &self.scr2
    }
}
#[doc = "GC (rw) register accessor: HRC mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gc`]
module"]
pub type GC = crate::Reg<gc::GC_SPEC>;
#[doc = "HRC mode configuration"]
pub mod gc;
#[doc = "PL (rw) register accessor: HRC output passive level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl`]
module"]
pub type PL = crate::Reg<pl::PL_SPEC>;
#[doc = "HRC output passive level"]
pub mod pl;
#[doc = "GSEL (rw) register accessor: HRC global control selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsel`]
module"]
pub type GSEL = crate::Reg<gsel::GSEL_SPEC>;
#[doc = "HRC global control selection"]
pub mod gsel;
#[doc = "TSEL (rw) register accessor: HRC timer selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsel`]
module"]
pub type TSEL = crate::Reg<tsel::TSEL_SPEC>;
#[doc = "HRC timer selection"]
pub mod tsel;
#[doc = "SC (r) register accessor: HRC current source for shadow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc`]
module"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "HRC current source for shadow"]
pub mod sc;
#[doc = "DCR (r) register accessor: HRC dead time rising value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`]
module"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "HRC dead time rising value"]
pub mod dcr;
#[doc = "DCF (r) register accessor: HRC dead time falling value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcf`]
module"]
pub type DCF = crate::Reg<dcf::DCF_SPEC>;
#[doc = "HRC dead time falling value"]
pub mod dcf;
#[doc = "CR1 (r) register accessor: HRC rising edge value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "HRC rising edge value"]
pub mod cr1;
#[doc = "CR2 (r) register accessor: HRC falling edge value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "HRC falling edge value"]
pub mod cr2;
#[doc = "SSC (rw) register accessor: HRC next source for shadow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssc`]
module"]
pub type SSC = crate::Reg<ssc::SSC_SPEC>;
#[doc = "HRC next source for shadow"]
pub mod ssc;
#[doc = "SDCR (rw) register accessor: HRC shadow dead time rising\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdcr`]
module"]
pub type SDCR = crate::Reg<sdcr::SDCR_SPEC>;
#[doc = "HRC shadow dead time rising"]
pub mod sdcr;
#[doc = "SDCF (rw) register accessor: HRC shadow dead time falling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdcf`]
module"]
pub type SDCF = crate::Reg<sdcf::SDCF_SPEC>;
#[doc = "HRC shadow dead time falling"]
pub mod sdcf;
#[doc = "SCR1 (rw) register accessor: HRC shadow rising edge value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr1`]
module"]
pub type SCR1 = crate::Reg<scr1::SCR1_SPEC>;
#[doc = "HRC shadow rising edge value"]
pub mod scr1;
#[doc = "SCR2 (rw) register accessor: HRC shadow falling edge value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr2`]
module"]
pub type SCR2 = crate::Reg<scr2::SCR2_SPEC>;
#[doc = "HRC shadow falling edge value"]
pub mod scr2;
