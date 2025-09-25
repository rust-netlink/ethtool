// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    emit_u32, parse_u32, DecodeError, DefaultNla, Emitable, ErrorContext, Nla,
    NlaBuffer, Parseable,
};

const ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE: u16 = 1;
const ETHTOOL_A_CABLE_STEP_LAST_DISTANCE: u16 = 2;
const ETHTOOL_A_CABLE_STEP_STEP_DISTANCE: u16 = 3;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolCableTestTdrStepAttr {
    First(u32),
    Last(u32),
    Step(u32),
    Other(DefaultNla),
}

impl Nla for EthtoolCableTestTdrStepAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::First(_) | Self::Last(_) | Self::Step(_) => {
                std::mem::size_of::<u32>()
            }
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::First(_) => ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE,
            Self::Last(_) => ETHTOOL_A_CABLE_STEP_LAST_DISTANCE,
            Self::Step(_) => ETHTOOL_A_CABLE_STEP_STEP_DISTANCE,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::First(value) | Self::Last(value) | Self::Step(value) => {
                emit_u32(buffer, *value).unwrap()
            }
            Self::Other(attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolCableTestTdrStepAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        match buf.kind() {
            ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE => {
                let value = parse_u32(buf.value()).context(
                    "failed to parse ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE",
                )?;
                Ok(Self::First(value))
            }
            ETHTOOL_A_CABLE_STEP_LAST_DISTANCE => {
                let value = parse_u32(buf.value()).context(
                    "failed to parse ETHTOOL_A_CABLE_STEP_LAST_DISTANCE",
                )?;
                Ok(Self::Last(value))
            }
            ETHTOOL_A_CABLE_STEP_STEP_DISTANCE => {
                let value = parse_u32(buf.value()).context(
                    "failed to parse ETHTOOL_A_CABLE_STEP_STEP_DISTANCE",
                )?;
                Ok(Self::Step(value))
            }
            _ => {
                let other = DefaultNla::parse(buf)
                    .context("failed to parse unknown NLA for TDR step")?;
                Ok(Self::Other(other))
            }
        }
    }
}
