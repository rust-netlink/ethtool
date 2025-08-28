// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    emit_u64, parse_u32, parse_u64, parse_u8, DecodeError, DefaultNla,
    Emitable, ErrorContext, Nla, NlaBuffer, NlasIterator, Parseable,
    NLA_F_NESTED,
};

use crate::{
    bitset_util::{parse_bitset_bits_nlas, EthtoolBitSet},
    EthtoolAttr, EthtoolHeader,
};

const ETHTOOL_A_FEC_HEADER: u16 = 1;
const ETHTOOL_A_FEC_MODES: u16 = 2;

const ETHTOOL_A_FEC_AUTO: u16 = 3;
const ETHTOOL_A_FEC_ACTIVE: u16 = 4;
const ETHTOOL_A_FEC_STATS: u16 = 5;

const ETHTOOL_LINK_MODE_FEC_NONE_BIT: u32 = 49;
const ETHTOOL_LINK_MODE_FEC_RS_BIT: u32 = 50;
const ETHTOOL_LINK_MODE_FEC_BASER_BIT: u32 = 51;
const ETHTOOL_LINK_MODE_FEC_LLRS_BIT: u32 = 74;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolFecAttr {
    Header(Vec<EthtoolHeader>),
    /// Configured modes
    Modes(Vec<EthtoolFecMode>),
    /// FEC mode auto selection
    /// Request the driver to choose FEC mode based on SFP module parameters.
    /// This does not mean autonegotiation.
    Auto(bool),
    /// Active FEC mode
    Active(EthtoolFecMode),
    Stats(Vec<EthtoolFecStat>),
    Other(DefaultNla),
}

impl Nla for EthtoolFecAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Header(hdrs) => hdrs.as_slice().buffer_len(),
            Self::Modes(_) => todo!("Does not support emitting EthtoolFecAttr"),
            Self::Auto(_) => 1,
            Self::Active(_) => 4,
            Self::Stats(s) => s.as_slice().buffer_len(),
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_FEC_HEADER | NLA_F_NESTED,
            Self::Modes(_) => ETHTOOL_A_FEC_MODES,
            Self::Auto(_) => ETHTOOL_A_FEC_AUTO,
            Self::Active(_) => ETHTOOL_A_FEC_ACTIVE,
            Self::Stats(_) => ETHTOOL_A_FEC_STATS,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(buffer),
            Self::Other(ref attr) => attr.emit(buffer),
            _ => todo!("Does not support changing ethtool fec yet"),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolFecAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_FEC_HEADER => {
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
            ETHTOOL_A_FEC_MODES => {
                let bits = parse_bitset_bits_nlas(payload).context(format!(
                    "Invalid ETHTOOL_A_FEC_TIMESTAMPING {payload:?}"
                ))?;
                Self::Modes(
                    bits.into_iter().map(EthtoolFecMode::from).collect(),
                )
            }
            ETHTOOL_A_FEC_AUTO => Self::Auto(
                parse_u8(payload).context(format!(
                    "Invalid ETHTOOL_A_FEC_AUTO {payload:?}"
                ))? > 0,
            ),
            ETHTOOL_A_FEC_ACTIVE => Self::Active(
                parse_u32(payload)
                    .context(format!(
                        "Invalid ETHTOOL_A_FEC_ACTIVE {payload:?}"
                    ))?
                    .into(),
            ),
            ETHTOOL_A_FEC_STATS => {
                let mut stats = Vec::new();
                for nla in NlasIterator::new(payload) {
                    let nla = &nla.context(format!(
                        "invalid ETHTOOL_A_FEC_STATS {payload:?}"
                    ))?;
                    let parsed = EthtoolFecStat::parse(nla)?;
                    stats.push(parsed);
                }
                Self::Stats(stats)
            }
            _ => Self::Other(
                DefaultNla::parse(buf).context("invalid NLA (unknown kind)")?,
            ),
        })
    }
}

pub(crate) fn parse_fec_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    let mut nlas = Vec::new();
    for nla in NlasIterator::new(buffer) {
        let error_msg =
            format!("Failed to parse ethtool fec message attribute {:?}", nla);
        let nla = &nla.context(error_msg.clone())?;
        let parsed = EthtoolFecAttr::parse(nla).context(error_msg)?;
        nlas.push(EthtoolAttr::Fec(parsed));
    }
    Ok(nlas)
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum EthtoolFecMode {
    #[default]
    None,
    Rs,
    Baser,
    Llrs,
    /// Index and name of FEC mode, only the index matters when setting.
    Other(u32, String),
}

impl std::fmt::Display for EthtoolFecMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::Rs => "rs",
                Self::Baser => "baser",
                Self::Llrs => "llrs",
                Self::Other(_, n) => n.as_str(),
            }
        )
    }
}

impl From<EthtoolBitSet> for EthtoolFecMode {
    fn from(b: EthtoolBitSet) -> Self {
        match b.index {
            ETHTOOL_LINK_MODE_FEC_NONE_BIT => Self::None,
            ETHTOOL_LINK_MODE_FEC_RS_BIT => Self::Rs,
            ETHTOOL_LINK_MODE_FEC_BASER_BIT => Self::Baser,
            ETHTOOL_LINK_MODE_FEC_LLRS_BIT => Self::Llrs,
            _ => Self::Other(b.index, b.name),
        }
    }
}

impl From<u32> for EthtoolFecMode {
    fn from(d: u32) -> Self {
        match d {
            ETHTOOL_LINK_MODE_FEC_NONE_BIT => Self::None,
            ETHTOOL_LINK_MODE_FEC_RS_BIT => Self::Rs,
            ETHTOOL_LINK_MODE_FEC_BASER_BIT => Self::Baser,
            ETHTOOL_LINK_MODE_FEC_LLRS_BIT => Self::Llrs,
            _ => Self::Other(d, String::new()),
        }
    }
}

impl From<EthtoolFecMode> for u32 {
    fn from(v: EthtoolFecMode) -> u32 {
        match v {
            EthtoolFecMode::None => ETHTOOL_LINK_MODE_FEC_NONE_BIT,
            EthtoolFecMode::Rs => ETHTOOL_LINK_MODE_FEC_RS_BIT,
            EthtoolFecMode::Baser => ETHTOOL_LINK_MODE_FEC_BASER_BIT,
            EthtoolFecMode::Llrs => ETHTOOL_LINK_MODE_FEC_LLRS_BIT,
            EthtoolFecMode::Other(d, _) => d,
        }
    }
}

const ETHTOOL_A_FEC_STAT_CORRECTED: u16 = 2;
const ETHTOOL_A_FEC_STAT_UNCORR: u16 = 3;
const ETHTOOL_A_FEC_STAT_CORR_BITS: u16 = 4;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolFecStat {
    Corrected(u64),
    Uncorrected(u64),
    CorrectBits(u64),
    Other(DefaultNla),
}

impl Nla for EthtoolFecStat {
    fn value_len(&self) -> usize {
        match self {
            Self::Other(attr) => attr.value_len(),
            _ => 8,
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Corrected(_) => ETHTOOL_A_FEC_STAT_CORRECTED,
            Self::Uncorrected(_) => ETHTOOL_A_FEC_STAT_UNCORR,
            Self::CorrectBits(_) => ETHTOOL_A_FEC_STAT_CORR_BITS,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Corrected(v)
            | Self::Uncorrected(v)
            | Self::CorrectBits(v) => emit_u64(buffer, *v).unwrap(),
            Self::Other(attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolFecStat
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_FEC_STAT_CORRECTED => {
                Self::Corrected(parse_u64(payload).context(format!(
                    "Invalid ETHTOOL_A_FEC_STAT_CORRECTED {payload:?}"
                ))?)
            }
            ETHTOOL_A_FEC_STAT_UNCORR => {
                Self::Uncorrected(parse_u64(payload).context(format!(
                    "Invalid ETHTOOL_A_FEC_STAT_UNCORR {payload:?}"
                ))?)
            }
            ETHTOOL_A_FEC_STAT_CORR_BITS => {
                Self::CorrectBits(parse_u64(payload).context(format!(
                    "Invalid ETHTOOL_A_FEC_STAT_CORR_BITS {payload:?}"
                ))?)
            }
            _ => Self::Other(
                DefaultNla::parse(buf).context("invalid NLA (unknown kind)")?,
            ),
        })
    }
}
