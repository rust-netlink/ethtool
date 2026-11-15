// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    append_u32, append_u64, parse_u32, parse_u64, parse_u8, DecodeError,
    DefaultNla, Emitable, EncodeError, ErrorContext, Nla, NlaBuffer,
    NlasIterator, Parseable, NLA_F_NESTED,
};

use crate::{EthtoolAttr, EthtoolHeader};

const ETHTOOL_A_PAUSE_HEADER: u16 = 1;
const ETHTOOL_A_PAUSE_AUTONEG: u16 = 2;
const ETHTOOL_A_PAUSE_RX: u16 = 3;
const ETHTOOL_A_PAUSE_TX: u16 = 4;
const ETHTOOL_A_PAUSE_STATS: u16 = 5;
const ETHTOOL_A_PAUSE_STATS_SRC: u16 = 6;

const ETHTOOL_A_PAUSE_STAT_TX_FRAMES: u16 = 2;
const ETHTOOL_A_PAUSE_STAT_RX_FRAMES: u16 = 3;
const ETHTOOL_A_PAUSE_STAT_TX_PAUSE_STORM_EVENTS: u16 = 4;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolPauseStatAttr {
    Rx(u64),
    Tx(u64),
    TxPauseStormEvents(u64),
    Other(DefaultNla),
}

impl Nla for EthtoolPauseStatAttr {
    fn kind(&self) -> u16 {
        match self {
            Self::Rx(_) => ETHTOOL_A_PAUSE_STAT_RX_FRAMES,
            Self::Tx(_) => ETHTOOL_A_PAUSE_STAT_TX_FRAMES,
            Self::TxPauseStormEvents(_) => {
                ETHTOOL_A_PAUSE_STAT_TX_PAUSE_STORM_EVENTS
            }
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            Self::Rx(value)
            | Self::Tx(value)
            | Self::TxPauseStormEvents(value) => append_u64(out, *value),
            Self::Other(ref attr) => attr.emit_value(out)?,
        }
        Ok(())
    }
}

impl<'a> Parseable<NlaBuffer<'a>> for EthtoolPauseStatAttr {
    fn parse(buf: &NlaBuffer<'a>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_PAUSE_STAT_TX_FRAMES => Self::Tx(
                parse_u64(payload)
                    .context("invalid ETHTOOL_A_PAUSE_STAT_TX_FRAMES value")?,
            ),
            ETHTOOL_A_PAUSE_STAT_RX_FRAMES => Self::Rx(
                parse_u64(payload)
                    .context("invalid ETHTOOL_A_PAUSE_STAT_RX_FRAMES value")?,
            ),
            ETHTOOL_A_PAUSE_STAT_TX_PAUSE_STORM_EVENTS => {
                Self::TxPauseStormEvents(parse_u64(payload).context(
                    "invalid ETHTOOL_A_PAUSE_STAT_TX_PAUSE_STORM_EVENTS value",
                )?)
            }
            _ => Self::Other(
                DefaultNla::parse(buf).context("invalid NLA (unknown kind)")?,
            ),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolPauseAttr {
    Header(Vec<EthtoolHeader>),
    AutoNeg(bool),
    Rx(bool),
    Tx(bool),
    Stats(Vec<EthtoolPauseStatAttr>),
    StatsSrc(u32),
    Other(DefaultNla),
}

impl Nla for EthtoolPauseAttr {
    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_PAUSE_HEADER | NLA_F_NESTED,
            Self::AutoNeg(_) => ETHTOOL_A_PAUSE_AUTONEG,
            Self::Rx(_) => ETHTOOL_A_PAUSE_RX,
            Self::Tx(_) => ETHTOOL_A_PAUSE_TX,
            Self::Stats(_) => ETHTOOL_A_PAUSE_STATS | NLA_F_NESTED,
            Self::StatsSrc(_) => ETHTOOL_A_PAUSE_STATS_SRC,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(out)?,
            Self::AutoNeg(value) | Self::Rx(value) | Self::Tx(value) => {
                out.push(*value as u8)
            }
            Self::Stats(ref nlas) => nlas.as_slice().emit(out)?,
            Self::StatsSrc(value) => append_u32(out, *value),
            Self::Other(ref attr) => attr.emit_value(out)?,
        }
        Ok(())
    }
}

impl<'a> Parseable<NlaBuffer<'a>> for EthtoolPauseAttr {
    fn parse(buf: &NlaBuffer<'a>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_PAUSE_HEADER => {
                let mut nlas = Vec::new();
                let error_msg = "failed to parse pause header attributes";
                for nla in NlasIterator::new(payload).context(error_msg)? {
                    let parsed =
                        EthtoolHeader::parse(&nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_PAUSE_AUTONEG => Self::AutoNeg(
                parse_u8(payload)
                    .context("invalid ETHTOOL_A_PAUSE_AUTONEG value")?
                    == 1,
            ),
            ETHTOOL_A_PAUSE_RX => Self::Rx(
                parse_u8(payload)
                    .context("invalid ETHTOOL_A_PAUSE_RX value")?
                    == 1,
            ),
            ETHTOOL_A_PAUSE_TX => Self::Tx(
                parse_u8(payload)
                    .context("invalid ETHTOOL_A_PAUSE_TX value")?
                    == 1,
            ),
            ETHTOOL_A_PAUSE_STATS => {
                let mut nlas = Vec::new();
                let error_msg = "failed to parse pause stats attributes";
                for nla in NlasIterator::new(payload).context(error_msg)? {
                    let parsed =
                        EthtoolPauseStatAttr::parse(&nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Stats(nlas)
            }
            ETHTOOL_A_PAUSE_STATS_SRC => Self::StatsSrc(
                parse_u32(payload)
                    .context("invalid ETHTOOL_A_PAUSE_STATS_SRC value")?,
            ),
            _ => Self::Other(
                DefaultNla::parse(buf).context("invalid NLA (unknown kind)")?,
            ),
        })
    }
}

pub(crate) fn parse_pause_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    let mut nlas = Vec::new();
    let error_msg = "Failed to parse ethtool pause message attributes";
    for nla in NlasIterator::new(buffer).context(error_msg)? {
        let parsed = EthtoolPauseAttr::parse(&nla).context(error_msg)?;
        nlas.push(EthtoolAttr::Pause(parsed));
    }
    Ok(nlas)
}
