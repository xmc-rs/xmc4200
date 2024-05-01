#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    gahbcfg: Gahbcfg,
    gusbcfg: Gusbcfg,
    grstctl: Grstctl,
    gintsts: Gintsts,
    gintmsk: Gintmsk,
    grxstsr: Grxstsr,
    grxstsp: Grxstsp,
    grxfsiz: Grxfsiz,
    gnptxfsiz: Gnptxfsiz,
    _reserved9: [u8; 0x10],
    guid: Guid,
    _reserved10: [u8; 0x1c],
    gdfifocfg: Gdfifocfg,
    _reserved11: [u8; 0xa4],
    dieptxf1: Dieptxf1,
    dieptxf2: Dieptxf2,
    dieptxf3: Dieptxf3,
    dieptxf4: Dieptxf4,
    dieptxf5: Dieptxf5,
    dieptxf6: Dieptxf6,
    _reserved17: [u8; 0x06e4],
    dcfg: Dcfg,
    dctl: Dctl,
    dsts: Dsts,
    _reserved20: [u8; 0x04],
    diepmsk: Diepmsk,
    doepmsk: Doepmsk,
    daint: Daint,
    daintmsk: Daintmsk,
    _reserved24: [u8; 0x08],
    dvbusdis: Dvbusdis,
    dvbuspulse: Dvbuspulse,
    _reserved26: [u8; 0x04],
    diepempmsk: Diepempmsk,
    _reserved27: [u8; 0x05c8],
    pcgcctl: Pcgcctl,
}
impl RegisterBlock {
    #[doc = "0x08 - AHB Configuration Register"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &Gahbcfg {
        &self.gahbcfg
    }
    #[doc = "0x0c - USB Configuration Register"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &Gusbcfg {
        &self.gusbcfg
    }
    #[doc = "0x10 - Reset Register"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &Grstctl {
        &self.grstctl
    }
    #[doc = "0x14 - Interrupt Register"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &Gintsts {
        &self.gintsts
    }
    #[doc = "0x18 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &Gintmsk {
        &self.gintmsk
    }
    #[doc = "0x1c - Receive Status Debug Read Register"]
    #[inline(always)]
    pub const fn grxstsr(&self) -> &Grxstsr {
        &self.grxstsr
    }
    #[doc = "0x20 - Receive Status Read and Pop Register"]
    #[inline(always)]
    pub const fn grxstsp(&self) -> &Grxstsp {
        &self.grxstsp
    }
    #[doc = "0x24 - Receive FIFO Size Register"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &Grxfsiz {
        &self.grxfsiz
    }
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &Gnptxfsiz {
        &self.gnptxfsiz
    }
    #[doc = "0x3c - USB Module Identification Register"]
    #[inline(always)]
    pub const fn guid(&self) -> &Guid {
        &self.guid
    }
    #[doc = "0x5c - Global DFIFO Software Config Register"]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &Gdfifocfg {
        &self.gdfifocfg
    }
    #[doc = "0x104 - Device IN Endpoint 1 Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &Dieptxf1 {
        &self.dieptxf1
    }
    #[doc = "0x108 - Device IN Endpoint 2 Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &Dieptxf2 {
        &self.dieptxf2
    }
    #[doc = "0x10c - Device IN Endpoint 3 Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &Dieptxf3 {
        &self.dieptxf3
    }
    #[doc = "0x110 - Device IN Endpoint 4 Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &Dieptxf4 {
        &self.dieptxf4
    }
    #[doc = "0x114 - Device IN Endpoint 5 Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &Dieptxf5 {
        &self.dieptxf5
    }
    #[doc = "0x118 - Device IN Endpoint 6 Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &Dieptxf6 {
        &self.dieptxf6
    }
    #[doc = "0x800 - Device Configuration Register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &Dcfg {
        &self.dcfg
    }
    #[doc = "0x804 - Device Control Register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0x808 - Device Status Register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &Dsts {
        &self.dsts
    }
    #[doc = "0x810 - Device IN Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &Diepmsk {
        &self.diepmsk
    }
    #[doc = "0x814 - Device OUT Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &Doepmsk {
        &self.doepmsk
    }
    #[doc = "0x818 - Device All Endpoints Interrupt Register"]
    #[inline(always)]
    pub const fn daint(&self) -> &Daint {
        &self.daint
    }
    #[doc = "0x81c - Device All Endpoints Interrupt Mask Register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &Daintmsk {
        &self.daintmsk
    }
    #[doc = "0x828 - Device VBUS Discharge Time Register"]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &Dvbusdis {
        &self.dvbusdis
    }
    #[doc = "0x82c - Device VBUS Pulsing Time Register"]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &Dvbuspulse {
        &self.dvbuspulse
    }
    #[doc = "0x834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &Diepempmsk {
        &self.diepempmsk
    }
    #[doc = "0xe00 - Power and Clock Gating Control Register"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &Pcgcctl {
        &self.pcgcctl
    }
}
#[doc = "GAHBCFG (rw) register accessor: AHB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
#[doc(alias = "GAHBCFG")]
pub type Gahbcfg = crate::Reg<gahbcfg::GahbcfgSpec>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
#[doc(alias = "GUSBCFG")]
pub type Gusbcfg = crate::Reg<gusbcfg::GusbcfgSpec>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
#[doc(alias = "GRSTCTL")]
pub type Grstctl = crate::Reg<grstctl::GrstctlSpec>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`]
module"]
#[doc(alias = "GINTSTS")]
pub type Gintsts = crate::Reg<gintsts::GintstsSpec>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`]
module"]
#[doc(alias = "GINTMSK")]
pub type Gintmsk = crate::Reg<gintmsk::GintmskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: Receive Status Debug Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr`]
module"]
#[doc(alias = "GRXSTSR")]
pub type Grxstsr = crate::Reg<grxstsr::GrxstsrSpec>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: Receive Status Read and Pop Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp`]
module"]
#[doc(alias = "GRXSTSP")]
pub type Grxstsp = crate::Reg<grxstsp::GrxstspSpec>;
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: Receive FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
#[doc(alias = "GRXFSIZ")]
pub type Grxfsiz = crate::Reg<grxfsiz::GrxfsizSpec>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: Non-Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz`]
module"]
#[doc(alias = "GNPTXFSIZ")]
pub type Gnptxfsiz = crate::Reg<gnptxfsiz::GnptxfsizSpec>;
#[doc = "Non-Periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GUID (rw) register accessor: USB Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@guid`]
module"]
#[doc(alias = "GUID")]
pub type Guid = crate::Reg<guid::GuidSpec>;
#[doc = "USB Module Identification Register"]
pub mod guid;
#[doc = "GDFIFOCFG (rw) register accessor: Global DFIFO Software Config Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdfifocfg`]
module"]
#[doc(alias = "GDFIFOCFG")]
pub type Gdfifocfg = crate::Reg<gdfifocfg::GdfifocfgSpec>;
#[doc = "Global DFIFO Software Config Register"]
pub mod gdfifocfg;
#[doc = "DIEPTXF1 (rw) register accessor: Device IN Endpoint 1 Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`]
module"]
#[doc(alias = "DIEPTXF1")]
pub type Dieptxf1 = crate::Reg<dieptxf1::Dieptxf1Spec>;
#[doc = "Device IN Endpoint 1 Transmit FIFO Size Register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: Device IN Endpoint 2 Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`]
module"]
#[doc(alias = "DIEPTXF2")]
pub type Dieptxf2 = crate::Reg<dieptxf2::Dieptxf2Spec>;
#[doc = "Device IN Endpoint 2 Transmit FIFO Size Register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: Device IN Endpoint 3 Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`]
module"]
#[doc(alias = "DIEPTXF3")]
pub type Dieptxf3 = crate::Reg<dieptxf3::Dieptxf3Spec>;
#[doc = "Device IN Endpoint 3 Transmit FIFO Size Register"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: Device IN Endpoint 4 Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`]
module"]
#[doc(alias = "DIEPTXF4")]
pub type Dieptxf4 = crate::Reg<dieptxf4::Dieptxf4Spec>;
#[doc = "Device IN Endpoint 4 Transmit FIFO Size Register"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: Device IN Endpoint 5 Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`]
module"]
#[doc(alias = "DIEPTXF5")]
pub type Dieptxf5 = crate::Reg<dieptxf5::Dieptxf5Spec>;
#[doc = "Device IN Endpoint 5 Transmit FIFO Size Register"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: Device IN Endpoint 6 Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`]
module"]
#[doc(alias = "DIEPTXF6")]
pub type Dieptxf6 = crate::Reg<dieptxf6::Dieptxf6Spec>;
#[doc = "Device IN Endpoint 6 Transmit FIFO Size Register"]
pub mod dieptxf6;
#[doc = "DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
#[doc(alias = "DSTS")]
pub type Dsts = crate::Reg<dsts::DstsSpec>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
#[doc(alias = "DIEPMSK")]
pub type Diepmsk = crate::Reg<diepmsk::DiepmskSpec>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
#[doc(alias = "DOEPMSK")]
pub type Doepmsk = crate::Reg<doepmsk::DoepmskSpec>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
#[doc(alias = "DAINT")]
pub type Daint = crate::Reg<daint::DaintSpec>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
#[doc(alias = "DAINTMSK")]
pub type Daintmsk = crate::Reg<daintmsk::DaintmskSpec>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: Device VBUS Discharge Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`]
module"]
#[doc(alias = "DVBUSDIS")]
pub type Dvbusdis = crate::Reg<dvbusdis::DvbusdisSpec>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: Device VBUS Pulsing Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`]
module"]
#[doc(alias = "DVBUSPULSE")]
pub type Dvbuspulse = crate::Reg<dvbuspulse::DvbuspulseSpec>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DIEPEMPMSK (rw) register accessor: Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
#[doc(alias = "DIEPEMPMSK")]
pub type Diepempmsk = crate::Reg<diepempmsk::DiepempmskSpec>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "PCGCCTL (rw) register accessor: Power and Clock Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`]
module"]
#[doc(alias = "PCGCCTL")]
pub type Pcgcctl = crate::Reg<pcgcctl::PcgcctlSpec>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
