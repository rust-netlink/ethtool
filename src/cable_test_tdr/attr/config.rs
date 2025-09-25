// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    emit_u32, parse_u32, parse_u8, DecodeError, DefaultNla, Emitable,
    ErrorContext, Nla, NlaBuffer, Parseable,
};

use crate::EthtoolCablePair;

const ETHTOOL_A_CABLE_TEST_TDR_CFG_FIRST: u16 = 1;
const ETHTOOL_A_CABLE_TEST_TDR_CFG_LAST: u16 = 2;
const ETHTOOL_A_CABLE_TEST_TDR_CFG_STEP: u16 = 3;
const ETHTOOL_A_CABLE_TEST_TDR_CFG_PAIR: u16 = 4;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolCableTestTdrConfigAttr {
    First(u32),
    Last(u32),
    Step(u32),
    Pair(EthtoolCablePair),
    Other(DefaultNla),
}

impl Nla for EthtoolCableTestTdrConfigAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::First(_) | Self::Last(_) | Self::Step(_) => {
                std::mem::size_of::<u32>()
            }
            Self::Pair(_) => std::mem::size_of::<u8>(),
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::First(_) => ETHTOOL_A_CABLE_TEST_TDR_CFG_FIRST,
            Self::Last(_) => ETHTOOL_A_CABLE_TEST_TDR_CFG_LAST,
            Self::Step(_) => ETHTOOL_A_CABLE_TEST_TDR_CFG_STEP,
            Self::Pair(_) => ETHTOOL_A_CABLE_TEST_TDR_CFG_PAIR,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::First(first) => emit_u32(buffer, *first).unwrap(),
            Self::Last(last) => emit_u32(buffer, *last).unwrap(),
            Self::Step(step) => emit_u32(buffer, *step).unwrap(),
            Self::Pair(pair) => buffer[0] = (*pair).into(),
            Self::Other(attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolCableTestTdrConfigAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        match buf.kind() {
            ETHTOOL_A_CABLE_TEST_TDR_CFG_FIRST => {
                let value = parse_u32(buf.value()).context(
                    "failed to parse ETHTOOL_A_CABLE_TEST_TDR_CFG_FIRST",
                )?;
                Ok(Self::First(value))
            }
            ETHTOOL_A_CABLE_TEST_TDR_CFG_LAST => {
                let value = parse_u32(buf.value()).context(
                    "failed to parse ETHTOOL_A_CABLE_TEST_TDR_CFG_LAST",
                )?;
                Ok(Self::Last(value))
            }
            ETHTOOL_A_CABLE_TEST_TDR_CFG_STEP => {
                let value = parse_u32(buf.value()).context(
                    "failed to parse ETHTOOL_A_CABLE_TEST_TDR_CFG_STEP",
                )?;
                Ok(Self::Step(value))
            }
            ETHTOOL_A_CABLE_TEST_TDR_CFG_PAIR => {
                let value = parse_u8(buf.value()).context(
                    "failed to parse ETHTOOL_A_CABLE_TEST_TDR_CFG_PAIR",
                )?;
                Ok(Self::Pair(value.into()))
            }
            _ => {
                let other = DefaultNla::parse(buf)
                    .context("failed to parse unknown NLA for TDR config")?;
                Ok(Self::Other(other))
            }
        }
    }
}
