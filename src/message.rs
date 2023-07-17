// SPDX-License-Identifier: MIT

use netlink_packet_generic::{GenlFamily, GenlHeader};
use netlink_packet_utils::{
    nla::Nla, DecodeError, Emitable, ParseableParametrized,
};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    channel::{parse_channel_nlas, EthtoolChannelAttr},
    coalesce::{parse_coalesce_nlas, EthtoolCoalesceAttr},
    feature::{parse_feature_nlas, EthtoolFeatureAttr},
    link_mode::{parse_link_mode_nlas, EthtoolLinkModeAttr},
    pause::{parse_pause_nlas, EthtoolPauseAttr},
    ring::{parse_ring_nlas, EthtoolRingAttr},
    tsinfo::{parse_tsinfo_nlas, EthtoolTsInfoAttr},
    EthtoolHeader,
};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, FromPrimitive)]
pub enum EthtoolRequest {
    LinkModeGet = 4,
    LinkModeSet = 5,
    FeatureGet = 11,
    FeatureSet = 12,
    RingGet = 15,
    RingSet = 16,
    ChannelGet = 17,
    ChannelSet = 18,
    CoalesceGet = 19,
    CoalesceSet = 20,
    PauseGet = 21,
    PauseSet = 22,
    TsInfoGet = 25,
    #[num_enum(catch_all)]
    UnSupport(u8),
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, FromPrimitive)]
pub enum EthtoolReply {
    LinkModeGetReply = 4,
    FeatureGetReply = 11,
    RingGetReply = 16,
    ChannelGetReply = 18,
    CoalesceGetReply = 20,
    PauseGetReply = 22,
    TsInfoGetReply = 26,
    #[num_enum(catch_all)]
    UnSupport(u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EthtoolCmd {
    EthtoolRequest(EthtoolRequest),
    EthtoolReply(EthtoolReply),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolAttr {
    Pause(EthtoolPauseAttr),
    Feature(EthtoolFeatureAttr),
    LinkMode(EthtoolLinkModeAttr),
    Ring(EthtoolRingAttr),
    Channel(EthtoolChannelAttr),
    Coalesce(EthtoolCoalesceAttr),
    TsInfo(EthtoolTsInfoAttr),
}

impl From<EthtoolReply> for EthtoolCmd {
    fn from(cmd: EthtoolReply) -> Self {
        EthtoolCmd::EthtoolReply(cmd)
    }
}

impl From<EthtoolRequest> for EthtoolCmd {
    fn from(cmd: EthtoolRequest) -> Self {
        EthtoolCmd::EthtoolRequest(cmd)
    }
}

impl Nla for EthtoolAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Pause(attr) => attr.value_len(),
            Self::Feature(attr) => attr.value_len(),
            Self::LinkMode(attr) => attr.value_len(),
            Self::Ring(attr) => attr.value_len(),
            Self::Channel(attr) => attr.value_len(),
            Self::Coalesce(attr) => attr.value_len(),
            Self::TsInfo(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Pause(attr) => attr.kind(),
            Self::Feature(attr) => attr.kind(),
            Self::LinkMode(attr) => attr.kind(),
            Self::Ring(attr) => attr.kind(),
            Self::Channel(attr) => attr.kind(),
            Self::Coalesce(attr) => attr.kind(),
            Self::TsInfo(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Pause(attr) => attr.emit_value(buffer),
            Self::Feature(attr) => attr.emit_value(buffer),
            Self::LinkMode(attr) => attr.emit_value(buffer),
            Self::Ring(attr) => attr.emit_value(buffer),
            Self::Channel(attr) => attr.emit_value(buffer),
            Self::Coalesce(attr) => attr.emit_value(buffer),
            Self::TsInfo(attr) => attr.emit_value(buffer),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EthtoolMessage {
    pub cmd: EthtoolCmd,
    pub nlas: Vec<EthtoolAttr>,
}

impl GenlFamily for EthtoolMessage {
    fn family_name() -> &'static str {
        "ethtool"
    }

    fn version(&self) -> u8 {
        1
    }

    fn command(&self) -> u8 {
        match self.cmd {
            EthtoolCmd::EthtoolRequest(req) => req.into(),
            EthtoolCmd::EthtoolReply(reply) => reply.into(),
        }
    }
}

impl EthtoolMessage {
    pub fn new_pause_get(iface_name: Option<&str>) -> Self {
        let nlas = match iface_name {
            Some(s) => {
                vec![EthtoolAttr::Pause(EthtoolPauseAttr::Header(vec![
                    EthtoolHeader::DevName(s.to_string()),
                ]))]
            }
            None => vec![EthtoolAttr::Pause(EthtoolPauseAttr::Header(vec![]))],
        };
        EthtoolMessage {
            cmd: EthtoolRequest::PauseGet.into(),
            nlas,
        }
    }

    pub fn new_feature_get(iface_name: Option<&str>) -> Self {
        let nlas = match iface_name {
            Some(s) => {
                vec![EthtoolAttr::Feature(EthtoolFeatureAttr::Header(vec![
                    EthtoolHeader::DevName(s.to_string()),
                ]))]
            }
            None => {
                vec![EthtoolAttr::Feature(EthtoolFeatureAttr::Header(vec![]))]
            }
        };
        EthtoolMessage {
            cmd: EthtoolRequest::FeatureGet.into(),
            nlas,
        }
    }

    pub fn new_link_mode_get(iface_name: Option<&str>) -> Self {
        let nlas = match iface_name {
            Some(s) => {
                vec![EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Header(vec![
                    EthtoolHeader::DevName(s.to_string()),
                ]))]
            }
            None => {
                vec![EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Header(vec![]))]
            }
        };
        EthtoolMessage {
            cmd: EthtoolRequest::LinkModeGet.into(),
            nlas,
        }
    }

    pub fn new_ring_get(iface_name: Option<&str>) -> Self {
        let nlas = match iface_name {
            Some(s) => vec![EthtoolAttr::Ring(EthtoolRingAttr::Header(vec![
                EthtoolHeader::DevName(s.to_string()),
            ]))],
            None => vec![EthtoolAttr::Ring(EthtoolRingAttr::Header(vec![]))],
        };
        EthtoolMessage {
            cmd: EthtoolRequest::RingGet.into(),
            nlas,
        }
    }

    pub fn new_channel_get(iface_name: Option<&str>) -> Self {
        let nlas = match iface_name {
            Some(s) => {
                vec![EthtoolAttr::Channel(EthtoolChannelAttr::Header(vec![
                    EthtoolHeader::DevName(s.to_string()),
                ]))]
            }
            None => {
                vec![EthtoolAttr::Channel(EthtoolChannelAttr::Header(vec![]))]
            }
        };
        EthtoolMessage {
            cmd: EthtoolRequest::ChannelGet.into(),
            nlas,
        }
    }

    pub fn new_coalesce_get(iface_name: Option<&str>) -> Self {
        let nlas = match iface_name {
            Some(s) => {
                vec![EthtoolAttr::Coalesce(EthtoolCoalesceAttr::Header(vec![
                    EthtoolHeader::DevName(s.to_string()),
                ]))]
            }
            None => {
                vec![EthtoolAttr::Coalesce(EthtoolCoalesceAttr::Header(vec![]))]
            }
        };
        EthtoolMessage {
            cmd: EthtoolRequest::CoalesceGet.into(),
            nlas,
        }
    }

    pub fn new_tsinfo_get(iface_name: Option<&str>) -> Self {
        let nlas = match iface_name {
            Some(s) => {
                vec![EthtoolAttr::TsInfo(EthtoolTsInfoAttr::Header(vec![
                    EthtoolHeader::DevName(s.to_string()),
                ]))]
            }
            None => {
                vec![EthtoolAttr::TsInfo(EthtoolTsInfoAttr::Header(vec![]))]
            }
        };
        EthtoolMessage {
            cmd: EthtoolRequest::TsInfoGet.into(),
            nlas,
        }
    }
}

impl Emitable for EthtoolMessage {
    fn buffer_len(&self) -> usize {
        self.nlas.as_slice().buffer_len()
    }

    fn emit(&self, buffer: &mut [u8]) {
        self.nlas.as_slice().emit(buffer)
    }
}

impl ParseableParametrized<[u8], GenlHeader> for EthtoolMessage {
    fn parse_with_param(
        buffer: &[u8],
        header: GenlHeader,
    ) -> Result<Self, DecodeError> {
        Ok(match EthtoolReply::from(header.cmd) {
            EthtoolReply::PauseGetReply => Self {
                cmd: EthtoolReply::PauseGetReply.into(),
                nlas: parse_pause_nlas(buffer)?,
            },
            EthtoolReply::FeatureGetReply => Self {
                cmd: EthtoolReply::FeatureGetReply.into(),
                nlas: parse_feature_nlas(buffer)?,
            },
            EthtoolReply::LinkModeGetReply => Self {
                cmd: EthtoolReply::LinkModeGetReply.into(),
                nlas: parse_link_mode_nlas(buffer)?,
            },
            EthtoolReply::RingGetReply => Self {
                cmd: EthtoolReply::RingGetReply.into(),
                nlas: parse_ring_nlas(buffer)?,
            },
            EthtoolReply::ChannelGetReply => Self {
                cmd: EthtoolReply::ChannelGetReply.into(),
                nlas: parse_channel_nlas(buffer)?,
            },
            EthtoolReply::CoalesceGetReply => Self {
                cmd: EthtoolReply::CoalesceGetReply.into(),
                nlas: parse_coalesce_nlas(buffer)?,
            },
            EthtoolReply::TsInfoGetReply => Self {
                cmd: EthtoolReply::TsInfoGetReply.into(),
                nlas: parse_tsinfo_nlas(buffer)?,
            },
            EthtoolReply::UnSupport(cmd) => {
                return Err(DecodeError::from(format!(
                    "Unsupported ethtool reply command: {cmd}"
                )))
            }
        })
    }
}
