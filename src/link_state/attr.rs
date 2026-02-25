// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    parse_u32, parse_u8, DecodeError, DefaultNla, Emitable, ErrorContext, 
    Nla, NlaBuffer, NlasIterator, Parseable, NLA_F_NESTED,
};

use crate::{EthtoolAttr, EthtoolHeader};

const ETHTOOL_A_LINKSTATE_HEADER: u16 = 1;
const ETHTOOL_A_LINKSTATE_LINK: u16 = 2;
const ETHTOOL_A_LINKSTATE_SQI: u16 = 3;
const ETHTOOL_A_LINKSTATE_SQI_MAX: u16 = 4;
const ETHTOOL_A_LINKSTATE_EXT_STATE: u16 = 5;
const ETHTOOL_A_LINKSTATE_EXT_SUBSTATE: u16 = 6;
const ETHTOOL_A_LINKSTATE_EXT_DOWN_CNT: u16 = 7;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolLinkStateAttr {
    Header(Vec<EthtoolHeader>),
    Link(bool),
    Sqi(u32),
    SqiMax(u32),
    ExtState(u8),
    ExtSubstate(u8),
    ExtDownCnt(u32),
    Other(DefaultNla),
}

impl Nla for EthtoolLinkStateAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Header(hdrs) => hdrs.as_slice().buffer_len(),
            Self::Link(_) | Self::ExtState(_) | Self::ExtSubstate(_) => 1,
            Self::Sqi(_) | Self::SqiMax(_) | Self::ExtDownCnt(_) => 4,
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_LINKSTATE_HEADER | NLA_F_NESTED,
            Self::Link(_) => ETHTOOL_A_LINKSTATE_LINK,
            Self::Sqi(_) => ETHTOOL_A_LINKSTATE_SQI,
            Self::SqiMax(_) => ETHTOOL_A_LINKSTATE_SQI_MAX,
            Self::ExtState(_) => ETHTOOL_A_LINKSTATE_EXT_STATE,
            Self::ExtSubstate(_) => ETHTOOL_A_LINKSTATE_EXT_SUBSTATE,
            Self::ExtDownCnt(_) => ETHTOOL_A_LINKSTATE_EXT_DOWN_CNT,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(buffer),
            Self::Other(ref attr) => attr.emit(buffer),
            _ => todo!("Does not support changing link state"),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolLinkStateAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_LINKSTATE_HEADER => {
                let mut nlas = Vec::new();
                let error_msg =
                    "failed to parse link state header attributes";
                for nla in NlasIterator::new(payload) {
                    let nla = &nla.context(error_msg)?;
                    let parsed =
                        EthtoolHeader::parse(nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_LINKSTATE_LINK => Self::Link(
                parse_u8(payload)
                    .context("invalid ETHTOOL_A_LINKSTATE_LINK value")?
                    == 1,
            ),
            ETHTOOL_A_LINKSTATE_SQI => Self::Sqi(
                parse_u32(payload)
                    .context("invalid ETHTOOL_A_LINKSTATE_SQI value")?,
            ),
            ETHTOOL_A_LINKSTATE_SQI_MAX => Self::SqiMax(
                parse_u32(payload)
                    .context("invalid ETHTOOL_A_LINKSTATE_SQI_MAX value")?,
            ),
            ETHTOOL_A_LINKSTATE_EXT_STATE => Self::ExtState(
                parse_u8(payload)
                    .context("invalid ETHTOOL_A_LINKSTATE_EXT_STATE value")?,
            ),
            ETHTOOL_A_LINKSTATE_EXT_SUBSTATE => Self::ExtSubstate(
                parse_u8(payload).context(
                    "invalid ETHTOOL_A_LINKSTATE_EXT_SUBSTATE value",
                )?,
            ),
            ETHTOOL_A_LINKSTATE_EXT_DOWN_CNT => Self::ExtDownCnt(
                parse_u32(payload).context(
                    "invalid ETHTOOL_A_LINKSTATE_EXT_DOWN_CNT value",
                )?,
            ),
            _ => Self::Other(
                DefaultNla::parse(buf).context("invalid NLA (unknown kind)")?,
            ),
        })
    }
}

pub(crate) fn parse_link_state_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    let mut nlas = Vec::new();
    for nla in NlasIterator::new(buffer) {
        let error_msg = format!(
            "Failed to parse ethtool link state message attribute {nla:?}"
        );
        let nla = &nla.context(error_msg.clone())?;
        let parsed = EthtoolLinkStateAttr::parse(nla).context(error_msg)?;
        nlas.push(EthtoolAttr::LinkState(parsed));
    }
    Ok(nlas)
}
