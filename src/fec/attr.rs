// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    append_u32, append_u64, parse_u32, parse_u8, DecodeError, DefaultNla,
    Emitable, EncodeError, ErrorContext, Nla, NlaBuffer, NlasIterator,
    Parseable, NLA_F_NESTED,
};

use crate::{bitset::EthtoolBitset, EthtoolAttr, EthtoolHeader};

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
    Modes(EthtoolBitset),
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
    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_FEC_HEADER | NLA_F_NESTED,
            Self::Modes(_) => ETHTOOL_A_FEC_MODES | NLA_F_NESTED,
            Self::Auto(_) => ETHTOOL_A_FEC_AUTO,
            Self::Active(_) => ETHTOOL_A_FEC_ACTIVE,
            Self::Stats(_) => ETHTOOL_A_FEC_STATS | NLA_F_NESTED,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(out)?,
            Self::Other(ref attr) => attr.emit_value(out)?,
            Self::Modes(bitset) => bitset.emit(out)?,
            Self::Auto(value) => out.push(*value as u8),
            Self::Active(value) => append_u32(out, u32::from(value.clone())),
            Self::Stats(ref nlas) => nlas.as_slice().emit(out)?,
        }
        Ok(())
    }
}

impl<'a> Parseable<NlaBuffer<'a>> for EthtoolFecAttr {
    fn parse(buf: &NlaBuffer<'a>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_FEC_HEADER => {
                let mut nlas = Vec::new();
                let error_msg = "failed to parse link_mode header attributes";
                for nla in NlasIterator::new(payload).context(error_msg)? {
                    let parsed =
                        EthtoolHeader::parse(&nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_FEC_MODES => {
                Self::Modes(EthtoolBitset::parse(payload).context(format!(
                    "Invalid ETHTOOL_A_FEC_MODES {payload:?}"
                ))?)
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
                let error_msg =
                    format!("invalid ETHTOOL_A_FEC_STATS {payload:?}");
                for nla in NlasIterator::new(payload).context(&error_msg)? {
                    let parsed =
                        EthtoolFecStat::parse(&nla).context(&error_msg)?;
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
    let error_msg = "Failed to parse ethtool fec message attributes";
    for nla in NlasIterator::new(buffer).context(error_msg)? {
        let parsed = EthtoolFecAttr::parse(&nla).context(error_msg)?;
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
    /// Corrected blocks, total and per lane.
    Corrected(Vec<u64>),
    /// Uncorrectable blocks, total and per lane.
    Uncorrected(Vec<u64>),
    /// Corrected bits, total and per lane.
    CorrectBits(Vec<u64>),
    Other(DefaultNla),
}

impl Nla for EthtoolFecStat {
    fn kind(&self) -> u16 {
        match self {
            Self::Corrected(_) => ETHTOOL_A_FEC_STAT_CORRECTED,
            Self::Uncorrected(_) => ETHTOOL_A_FEC_STAT_UNCORR,
            Self::CorrectBits(_) => ETHTOOL_A_FEC_STAT_CORR_BITS,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            Self::Corrected(values)
            | Self::Uncorrected(values)
            | Self::CorrectBits(values) => {
                for value in values {
                    append_u64(out, *value);
                }
            }
            Self::Other(attr) => attr.emit_value(out)?,
        }
        Ok(())
    }
}

fn parse_u64_values(payload: &[u8]) -> Result<Vec<u64>, DecodeError> {
    if !payload.len().is_multiple_of(8) {
        return Err(format!(
            "Invalid u64 array length {}, not multiple of 8",
            payload.len()
        )
        .into());
    }
    Ok(payload
        .as_chunks::<8>()
        .0
        .iter()
        .map(|c| u64::from_ne_bytes(*c))
        .collect())
}

impl<'a> Parseable<NlaBuffer<'a>> for EthtoolFecStat {
    fn parse(buf: &NlaBuffer<'a>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_FEC_STAT_CORRECTED => {
                Self::Corrected(parse_u64_values(payload).context(format!(
                    "Invalid ETHTOOL_A_FEC_STAT_CORRECTED {payload:?}"
                ))?)
            }
            ETHTOOL_A_FEC_STAT_UNCORR => {
                Self::Uncorrected(parse_u64_values(payload).context(
                    format!("Invalid ETHTOOL_A_FEC_STAT_UNCORR {payload:?}"),
                )?)
            }
            ETHTOOL_A_FEC_STAT_CORR_BITS => {
                Self::CorrectBits(parse_u64_values(payload).context(
                    format!("Invalid ETHTOOL_A_FEC_STAT_CORR_BITS {payload:?}"),
                )?)
            }
            _ => Self::Other(
                DefaultNla::parse(buf).context("invalid NLA (unknown kind)")?,
            ),
        })
    }
}
