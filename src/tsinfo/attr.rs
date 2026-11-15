// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    append_u32, parse_u32, DecodeError, DefaultNla, Emitable, EncodeError,
    ErrorContext, Nla, NlaBuffer, NlasIterator, Parseable, NLA_F_NESTED,
};

use crate::{bitset::EthtoolBitset, EthtoolAttr, EthtoolHeader};

const ETHTOOL_A_TSINFO_HEADER: u16 = 1;
const ETHTOOL_A_TSINFO_TIMESTAMPING: u16 = 2;
const ETHTOOL_A_TSINFO_TX_TYPES: u16 = 3;
const ETHTOOL_A_TSINFO_RX_FILTERS: u16 = 4;
const ETHTOOL_A_TSINFO_PHC_INDEX: u16 = 5;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolTsInfoAttr {
    Header(Vec<EthtoolHeader>),
    Timestamping(EthtoolBitset),
    TxTypes(EthtoolBitset),
    RxFilters(EthtoolBitset),
    PhcIndex(u32),
    Other(DefaultNla),
}

impl Nla for EthtoolTsInfoAttr {
    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_TSINFO_HEADER | NLA_F_NESTED,
            Self::Timestamping(_) => {
                ETHTOOL_A_TSINFO_TIMESTAMPING | NLA_F_NESTED
            }
            Self::TxTypes(_) => ETHTOOL_A_TSINFO_TX_TYPES | NLA_F_NESTED,
            Self::RxFilters(_) => ETHTOOL_A_TSINFO_RX_FILTERS | NLA_F_NESTED,
            Self::PhcIndex(_) => ETHTOOL_A_TSINFO_PHC_INDEX,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(out)?,
            Self::Other(ref attr) => attr.emit_value(out)?,
            Self::Timestamping(bitset)
            | Self::TxTypes(bitset)
            | Self::RxFilters(bitset) => bitset.emit(out)?,
            Self::PhcIndex(value) => append_u32(out, *value),
        }
        Ok(())
    }
}

impl<'a> Parseable<NlaBuffer<'a>> for EthtoolTsInfoAttr {
    fn parse(buf: &NlaBuffer<'a>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_TSINFO_HEADER => {
                let mut nlas = Vec::new();
                let error_msg = "failed to parse link_mode header attributes";
                for nla in NlasIterator::new(payload).context(error_msg)? {
                    let parsed =
                        EthtoolHeader::parse(&nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_TSINFO_TIMESTAMPING => Self::Timestamping(
                EthtoolBitset::parse(payload)
                    .context("Invalid ETHTOOL_A_TSINFO_TIMESTAMPING value")?,
            ),
            ETHTOOL_A_TSINFO_TX_TYPES => Self::TxTypes(
                EthtoolBitset::parse(payload)
                    .context("Invalid ETHTOOL_A_TSINFO_TX_TYPES value")?,
            ),
            ETHTOOL_A_TSINFO_RX_FILTERS => Self::RxFilters(
                EthtoolBitset::parse(payload)
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
    let error_msg = "Failed to parse ethtool tsinfo message attributes";
    for nla in NlasIterator::new(buffer).context(error_msg)? {
        let parsed = EthtoolTsInfoAttr::parse(&nla).context(error_msg)?;
        nlas.push(EthtoolAttr::TsInfo(parsed));
    }
    Ok(nlas)
}
