// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    parse_u8, DecodeError, DefaultNla, Emitable, ErrorContext, Nla, NlaBuffer,
    NlasIterator, Parseable, NLA_F_NESTED,
};

use crate::{
    cable_test_tdr::attr::nest::EthtoolCableTestTdrNestAttr, EthtoolAttr,
    EthtoolHeader,
};

const ETHTOOL_A_CABLE_TEST_TDR_NTF_HEADER: u16 = 1;
const ETHTOOL_A_CABLE_TEST_TDR_NTF_STATUS: u16 = 2;
const ETHTOOL_A_CABLE_TEST_TDR_NTF_NEST: u16 = 3;

const ETHTOOL_A_CABLE_TEST_NTF_STATUS_STARTED: u8 = 1;
const ETHTOOL_A_CABLE_TEST_NTF_STATUS_COMPLETED: u8 = 2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EthtoolCableTestTdrStatus {
    Started,
    Completed,
    Other(u8),
}

impl From<u8> for EthtoolCableTestTdrStatus {
    fn from(value: u8) -> Self {
        match value {
            ETHTOOL_A_CABLE_TEST_NTF_STATUS_STARTED => Self::Started,
            ETHTOOL_A_CABLE_TEST_NTF_STATUS_COMPLETED => Self::Completed,
            _ => Self::Other(value),
        }
    }
}

impl From<EthtoolCableTestTdrStatus> for u8 {
    fn from(value: EthtoolCableTestTdrStatus) -> Self {
        match value {
            EthtoolCableTestTdrStatus::Started => {
                ETHTOOL_A_CABLE_TEST_NTF_STATUS_STARTED
            }
            EthtoolCableTestTdrStatus::Completed => {
                ETHTOOL_A_CABLE_TEST_NTF_STATUS_COMPLETED
            }
            EthtoolCableTestTdrStatus::Other(value) => value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolCableTestTdrNotifyAttr {
    Header(Vec<EthtoolHeader>),
    Status(EthtoolCableTestTdrStatus),
    Nest(Vec<EthtoolCableTestTdrNestAttr>),
    Other(DefaultNla),
}

impl Nla for EthtoolCableTestTdrNotifyAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Header(header) => header.as_slice().buffer_len(),
            Self::Status(_) => std::mem::size_of::<u8>(),
            Self::Nest(nest) => nest.as_slice().buffer_len(),
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => {
                ETHTOOL_A_CABLE_TEST_TDR_NTF_HEADER | NLA_F_NESTED
            }
            Self::Status(_) => ETHTOOL_A_CABLE_TEST_TDR_NTF_STATUS,
            Self::Nest(_) => ETHTOOL_A_CABLE_TEST_TDR_NTF_NEST | NLA_F_NESTED,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Header(header) => header.as_slice().emit(buffer),
            Self::Status(status) => {
                buffer.get_mut(0).map(|b| *b = (*status).into()).unwrap()
            }
            Self::Nest(nest) => nest.as_slice().emit(buffer),
            Self::Other(attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolCableTestTdrNotifyAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        match buf.kind() {
            ETHTOOL_A_CABLE_TEST_TDR_NTF_HEADER => {
                let nlas = NlasIterator::new(buf.value())
                    .map(|nla| {
                        EthtoolHeader::parse(&nla?)
                            .context("failed to parse ETHTOOL_A_CABLE_TEST_TDR_NTF_HEADER")
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Header(nlas))
            }
            ETHTOOL_A_CABLE_TEST_TDR_NTF_STATUS => {
                let value = parse_u8(buf.value()).context(
                    "failed to parse ETHTOOL_A_CABLE_TEST_TDR_NTF_STATUS",
                )?;
                Ok(Self::Status(value.into()))
            }
            ETHTOOL_A_CABLE_TEST_TDR_NTF_NEST => {
                let nlas = NlasIterator::new(buf.value())
                    .map(|nla| {
                        EthtoolCableTestTdrNestAttr::parse(&nla?).context(
                            "failed to parse ETHTOOL_A_CABLE_TEST_TDR_NTF_NEST",
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Nest(nlas))
            }
            _ => {
                let other = DefaultNla::parse(buf)
                    .context("failed to parse unknown NLA for TDR notify")?;
                Ok(Self::Other(other))
            }
        }
    }
}

pub(crate) fn parse_cable_test_tdr_notify_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    NlasIterator::new(buffer)
        .map(|nla| {
            let nla = nla.context(
                "failed to get ethtool cable test message attribute",
            )?;
            let parsed = EthtoolCableTestTdrNotifyAttr::parse(&nla)
                .context("failed to parse ethtool cable test TDR NLA")?;
            Ok(EthtoolAttr::CableTestTdrNotify(parsed))
        })
        .collect()
}
