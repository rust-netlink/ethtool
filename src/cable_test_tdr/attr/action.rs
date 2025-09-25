// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    DecodeError, DefaultNla, Emitable, ErrorContext, Nla, NlaBuffer,
    NlasIterator, Parseable, NLA_F_NESTED,
};

use crate::{
    cable_test_tdr::attr::config::EthtoolCableTestTdrConfigAttr, EthtoolAttr,
    EthtoolHeader,
};

const ETHTOOL_A_CABLE_TEST_TDR_HEADER: u16 = 1;
const ETHTOOL_A_CABLE_TEST_TDR_CFG: u16 = 2;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolCableTestTdrActionAttr {
    Header(Vec<EthtoolHeader>),
    Config(Vec<EthtoolCableTestTdrConfigAttr>),
    Other(DefaultNla),
}

impl Nla for EthtoolCableTestTdrActionAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Header(header) => header.as_slice().buffer_len(),
            Self::Config(config) => config.as_slice().buffer_len(),
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_CABLE_TEST_TDR_HEADER | NLA_F_NESTED,
            Self::Config(_) => ETHTOOL_A_CABLE_TEST_TDR_CFG | NLA_F_NESTED,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Header(header) => header.as_slice().emit(buffer),
            Self::Config(config) => config.as_slice().emit(buffer),
            Self::Other(attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolCableTestTdrActionAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        match buf.kind() {
            ETHTOOL_A_CABLE_TEST_TDR_HEADER => {
                let nlas = NlasIterator::new(buf.value())
                    .map(|nla| {
                        EthtoolHeader::parse(&nla?).context(
                            "failed to parse ETHTOOL_A_CABLE_TEST_TDR_HEADER",
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Header(nlas))
            }
            ETHTOOL_A_CABLE_TEST_TDR_CFG => {
                let nlas = NlasIterator::new(buf.value())
                    .map(|nla| {
                        EthtoolCableTestTdrConfigAttr::parse(&nla?).context(
                            "failed to parse ETHTOOL_A_CABLE_TEST_TDR_CFG",
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Config(nlas))
            }
            _ => {
                let other = DefaultNla::parse(buf)
                    .context("failed to parse unknown NLA for TDR action")?;
                Ok(Self::Other(other))
            }
        }
    }
}

pub(crate) fn parse_cable_test_tdr_action_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    NlasIterator::new(buffer)
        .map(|nla| {
            let nla =
                nla.context("failed to get ethtool cable test TDR action NLA")?;
            let parsed = EthtoolCableTestTdrActionAttr::parse(&nla)
                .context("failed to parse ethtool cable test TDR action NLA")?;
            Ok(EthtoolAttr::CableTestTdrAction(parsed))
        })
        .collect()
}
