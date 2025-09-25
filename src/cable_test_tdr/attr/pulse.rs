// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    emit_i16, parse_i16, DecodeError, DefaultNla, Emitable, ErrorContext, Nla,
    NlaBuffer, Parseable,
};

const ETHTOOL_A_CABLE_PULSE_MV: u16 = 1;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolCableTestTdrPulseAttr {
    Mv(i16),
    Other(DefaultNla),
}

impl Nla for EthtoolCableTestTdrPulseAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Mv(_) => std::mem::size_of::<i16>(),
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Mv(_) => ETHTOOL_A_CABLE_PULSE_MV,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Mv(mv) => emit_i16(buffer, *mv).unwrap(),
            Self::Other(attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolCableTestTdrPulseAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        match buf.kind() {
            ETHTOOL_A_CABLE_PULSE_MV => {
                let value = parse_i16(buf.value())
                    .context("failed to parse ETHTOOL_A_CABLE_PULSE_MV")?;
                Ok(Self::Mv(value))
            }
            _ => {
                let other = DefaultNla::parse(buf)
                    .context("failed to parse unknown NLA for TDR pulse")?;
                Ok(Self::Other(other))
            }
        }
    }
}
