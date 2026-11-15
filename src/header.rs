// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    append_u32, parse_string, parse_u32, DecodeError, DefaultNla, EncodeError,
    ErrorContext, Nla, NlaBuffer, Parseable,
};

pub(crate) const ALTIFNAMSIZ: usize = 128;
const ETHTOOL_A_HEADER_DEV_INDEX: u16 = 1;
const ETHTOOL_A_HEADER_DEV_NAME: u16 = 2;
const ETHTOOL_A_HEADER_FLAGS: u16 = 3;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolHeader {
    DevIndex(u32),
    DevName(String),
    Flags(u32),
    Other(DefaultNla),
}

impl Nla for EthtoolHeader {
    fn kind(&self) -> u16 {
        match self {
            Self::DevIndex(_) => ETHTOOL_A_HEADER_DEV_INDEX,
            Self::DevName(_) => ETHTOOL_A_HEADER_DEV_NAME,
            Self::Flags(_) => ETHTOOL_A_HEADER_FLAGS,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            Self::DevIndex(value) | Self::Flags(value) => {
                append_u32(out, *value);
            }
            Self::DevName(s) => {
                append_zero_ended_string(out, s, ALTIFNAMSIZ);
            }
            Self::Other(ref attr) => attr.emit_value(out)?,
        }
        Ok(())
    }
}

impl<'a> Parseable<NlaBuffer<'a>> for EthtoolHeader {
    fn parse(buf: &NlaBuffer<'a>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_HEADER_DEV_INDEX => Self::DevIndex(
                parse_u32(payload)
                    .context("invalid ETHTOOL_A_HEADER_DEV_INDEX value")?,
            ),
            ETHTOOL_A_HEADER_FLAGS => Self::Flags(
                parse_u32(payload)
                    .context("invalid ETHTOOL_A_HEADER_FLAGS value")?,
            ),
            ETHTOOL_A_HEADER_DEV_NAME => Self::DevName(
                parse_string(payload)
                    .context("invalid ETHTOOL_A_HEADER_DEV_NAME value")?,
            ),
            _ => Self::Other(
                DefaultNla::parse(buf).context("invalid NLA (unknown kind)")?,
            ),
        })
    }
}

fn append_zero_ended_string(out: &mut Vec<u8>, src: &str, max_size: usize) {
    let mut bytes = src.as_bytes().to_vec();
    bytes.push(0);
    let len = bytes.len().min(max_size);
    out.extend_from_slice(&bytes[..len]);
}
