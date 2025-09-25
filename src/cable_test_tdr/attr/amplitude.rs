// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    emit_i16, parse_i16, parse_u8, DecodeError, DefaultNla, Emitable,
    ErrorContext, Nla, NlaBuffer, Parseable,
};

use crate::EthtoolCablePair;

const ETHTOOL_A_CABLE_AMPLITUDE_PAIR: u16 = 1;
const ETHTOOL_A_CABLE_AMPLITUDE_MV: u16 = 2;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolCableTestTdrAmplitudeAttr {
    Pair(EthtoolCablePair),
    Mv(i16),
    Other(DefaultNla),
}

impl Nla for EthtoolCableTestTdrAmplitudeAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Pair(_) => std::mem::size_of::<u8>(),
            Self::Mv(_) => std::mem::size_of::<i16>(),
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Pair(_) => ETHTOOL_A_CABLE_AMPLITUDE_PAIR,
            Self::Mv(_) => ETHTOOL_A_CABLE_AMPLITUDE_MV,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Pair(pair) => buffer[0] = (*pair).into(),
            Self::Mv(mv) => emit_i16(buffer, *mv).unwrap(),
            Self::Other(attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolCableTestTdrAmplitudeAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        match buf.kind() {
            ETHTOOL_A_CABLE_AMPLITUDE_PAIR => {
                let value = parse_u8(buf.value()).context(
                    "failed to parse ETHTOOL_A_CABLE_AMPLITUDE_PAIR",
                )?;
                Ok(Self::Pair(value.into()))
            }
            ETHTOOL_A_CABLE_AMPLITUDE_MV => {
                let value = parse_i16(buf.value())
                    .context("failed to parse ETHTOOL_A_CABLE_AMPLITUDE_MV")?;
                Ok(Self::Mv(value))
            }
            _ => {
                let other = DefaultNla::parse(buf)
                    .context("failed to parse unknown NLA for TDR amplitude")?;
                Ok(Self::Other(other))
            }
        }
    }
}
