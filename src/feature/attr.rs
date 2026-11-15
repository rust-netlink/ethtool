// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    DecodeError, DefaultNla, Emitable, EncodeError, ErrorContext, Nla,
    NlaBuffer, NlasIterator, Parseable, NLA_F_NESTED,
};

use crate::{bitset::EthtoolBitset, EthtoolAttr, EthtoolHeader};

const ETHTOOL_A_FEATURES_HEADER: u16 = 1;
const ETHTOOL_A_FEATURES_HW: u16 = 2; // User changable features
const ETHTOOL_A_FEATURES_WANTED: u16 = 3; // User requested fatures
const ETHTOOL_A_FEATURES_ACTIVE: u16 = 4; // Active features
const ETHTOOL_A_FEATURES_NOCHANGE: u16 = 5;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolFeatureAttr {
    Header(Vec<EthtoolHeader>),
    Hw(EthtoolBitset),
    Wanted(EthtoolBitset),
    Active(EthtoolBitset),
    NoChange(EthtoolBitset),
    Other(DefaultNla),
}

impl Nla for EthtoolFeatureAttr {
    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_FEATURES_HEADER | NLA_F_NESTED,
            Self::Hw(_) => ETHTOOL_A_FEATURES_HW | NLA_F_NESTED,
            Self::Wanted(_) => ETHTOOL_A_FEATURES_WANTED | NLA_F_NESTED,
            Self::Active(_) => ETHTOOL_A_FEATURES_ACTIVE | NLA_F_NESTED,
            Self::NoChange(_) => ETHTOOL_A_FEATURES_NOCHANGE | NLA_F_NESTED,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(out)?,
            Self::Hw(bitset)
            | Self::Wanted(bitset)
            | Self::Active(bitset)
            | Self::NoChange(bitset) => bitset.emit(out)?,
            Self::Other(ref attr) => attr.emit_value(out)?,
        }
        Ok(())
    }
}

impl<'a> Parseable<NlaBuffer<'a>> for EthtoolFeatureAttr {
    fn parse(buf: &NlaBuffer<'a>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_FEATURES_HEADER => {
                let mut nlas = Vec::new();
                let error_msg = "failed to parse feature header attributes";
                for nla in NlasIterator::new(payload).context(error_msg)? {
                    let parsed =
                        EthtoolHeader::parse(&nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_FEATURES_HW => Self::Hw(
                EthtoolBitset::parse(payload)
                    .context("failed to parse ETHTOOL_A_FEATURES_HW bitset")?,
            ),
            ETHTOOL_A_FEATURES_WANTED => {
                Self::Wanted(EthtoolBitset::parse(payload).context(
                    "failed to parse ETHTOOL_A_FEATURES_WANTED bitset",
                )?)
            }
            ETHTOOL_A_FEATURES_ACTIVE => {
                Self::Active(EthtoolBitset::parse(payload).context(
                    "failed to parse ETHTOOL_A_FEATURES_ACTIVE bitset",
                )?)
            }
            ETHTOOL_A_FEATURES_NOCHANGE => {
                Self::NoChange(EthtoolBitset::parse(payload).context(
                    "failed to parse ETHTOOL_A_FEATURES_NOCHANGE bitset",
                )?)
            }
            _ => Self::Other(
                DefaultNla::parse(buf).context("invalid NLA (unknown kind)")?,
            ),
        })
    }
}

pub(crate) fn parse_feature_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    let mut nlas = Vec::new();
    let error_msg = "Failed to parse ethtool feature message attributes";
    for nla in NlasIterator::new(buffer).context(error_msg)? {
        let parsed = EthtoolFeatureAttr::parse(&nla).context(error_msg)?;
        nlas.push(EthtoolAttr::Feature(parsed));
    }
    Ok(nlas)
}
