// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    parse_u32, DecodeError, DefaultNla, Emitable, ErrorContext, Nla, NlaBuffer,
    NlasIterator, Parseable, NLA_F_NESTED,
};

use crate::{
    bitset_util::parse_bitset_bits_string_nlas, EthtoolAttr, EthtoolHeader,
};

const ETHTOOL_A_TSINFO_HEADER: u16 = 1;
const ETHTOOL_A_TSINFO_TIMESTAMPING: u16 = 2;
const ETHTOOL_A_TSINFO_TX_TYPES: u16 = 3;
const ETHTOOL_A_TSINFO_RX_FILTERS: u16 = 4;
const ETHTOOL_A_TSINFO_PHC_INDEX: u16 = 5;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolTsInfoAttr {
    Header(Vec<EthtoolHeader>),
    Timestamping(Vec<String>),
    TxTypes(Vec<String>),
    RxFilters(Vec<String>),
    PhcIndex(u32),
    Other(DefaultNla),
}

impl Nla for EthtoolTsInfoAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Header(hdrs) => hdrs.as_slice().buffer_len(),
            Self::Timestamping(_)
            | Self::PhcIndex(_)
            | Self::TxTypes(_)
            | Self::RxFilters(_) => 4,
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_TSINFO_HEADER | NLA_F_NESTED,
            Self::Timestamping(_) => ETHTOOL_A_TSINFO_TIMESTAMPING,
            Self::TxTypes(_) => ETHTOOL_A_TSINFO_TX_TYPES,
            Self::RxFilters(_) => ETHTOOL_A_TSINFO_RX_FILTERS,
            Self::PhcIndex(_) => ETHTOOL_A_TSINFO_PHC_INDEX,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(buffer),
            Self::Other(ref attr) => attr.emit(buffer),
            _ => todo!("Does not support changing ethtool ts info yet"),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolTsInfoAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_TSINFO_HEADER => {
                let mut nlas = Vec::new();
                let error_msg = "failed to parse link_mode header attributes";
                for nla in NlasIterator::new(payload) {
                    let nla = &nla.context(error_msg)?;
                    let parsed =
                        EthtoolHeader::parse(nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_TSINFO_TIMESTAMPING => Self::Timestamping(
                parse_bitset_bits_string_nlas(payload)
                    .context("Invalid ETHTOOL_A_TSINFO_TIMESTAMPING value")?,
            ),
            ETHTOOL_A_TSINFO_TX_TYPES => Self::TxTypes(
                parse_bitset_bits_string_nlas(payload)
                    .context("Invalid ETHTOOL_A_TSINFO_TX_TYPES value")?,
            ),
            ETHTOOL_A_TSINFO_RX_FILTERS => Self::RxFilters(
                parse_bitset_bits_string_nlas(payload)
                    .context("Invalid ETHTOOL_A_TSINFO_RX_FILTERS value")?,
            ),
            ETHTOOL_A_TSINFO_PHC_INDEX => Self::PhcIndex(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_TSINFO_PHC_INDEX value")?,
            ),
            _ => Self::Other(
                DefaultNla::parse(buf).context("invalid NLA (unknown kind)")?,
            ),
        })
    }
}

pub(crate) fn parse_tsinfo_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    let mut nlas = Vec::new();
    for nla in NlasIterator::new(buffer) {
        let error_msg = format!(
            "Failed to parse ethtool tsinfo message attribute {:?}",
            nla
        );
        let nla = &nla.context(error_msg.clone())?;
        let parsed = EthtoolTsInfoAttr::parse(nla).context(error_msg)?;
        nlas.push(EthtoolAttr::TsInfo(parsed));
    }
    Ok(nlas)
}
