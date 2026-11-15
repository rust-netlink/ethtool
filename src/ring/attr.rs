// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    append_u32, parse_u32, parse_u8, DecodeError, DefaultNla, Emitable,
    EncodeError, ErrorContext, Nla, NlaBuffer, NlasIterator, Parseable,
    NLA_F_NESTED,
};

use crate::{EthtoolAttr, EthtoolHeader};

const ETHTOOL_A_RINGS_HEADER: u16 = 1;
const ETHTOOL_A_RINGS_RX_MAX: u16 = 2;
const ETHTOOL_A_RINGS_RX_MINI_MAX: u16 = 3;
const ETHTOOL_A_RINGS_RX_JUMBO_MAX: u16 = 4;
const ETHTOOL_A_RINGS_TX_MAX: u16 = 5;
const ETHTOOL_A_RINGS_RX: u16 = 6;
const ETHTOOL_A_RINGS_RX_MINI: u16 = 7;
const ETHTOOL_A_RINGS_RX_JUMBO: u16 = 8;
const ETHTOOL_A_RINGS_TX: u16 = 9;
const ETHTOOL_A_RINGS_RX_BUF_LEN: u16 = 10;
const ETHTOOL_A_RINGS_TCP_DATA_SPLIT: u16 = 11;
const ETHTOOL_A_RINGS_CQE_SIZE: u16 = 12;
const ETHTOOL_A_RINGS_TX_PUSH: u16 = 13;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolRingAttr {
    Header(Vec<EthtoolHeader>),
    RxMax(u32),
    RxMiniMax(u32),
    RxJumboMax(u32),
    TxMax(u32),
    Rx(u32),
    RxMini(u32),
    RxJumbo(u32),
    Tx(u32),
    RxBufLen(u32),
    TcpDataSplit(u8),
    CqeSize(u32),
    TxPush(bool),
    Other(DefaultNla),
}

impl Nla for EthtoolRingAttr {
    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_RINGS_HEADER | NLA_F_NESTED,
            Self::RxMax(_) => ETHTOOL_A_RINGS_RX_MAX,
            Self::RxMiniMax(_) => ETHTOOL_A_RINGS_RX_MINI_MAX,
            Self::RxJumboMax(_) => ETHTOOL_A_RINGS_RX_JUMBO_MAX,
            Self::TxMax(_) => ETHTOOL_A_RINGS_TX_MAX,
            Self::Rx(_) => ETHTOOL_A_RINGS_RX,
            Self::RxMini(_) => ETHTOOL_A_RINGS_RX_MINI,
            Self::RxJumbo(_) => ETHTOOL_A_RINGS_RX_JUMBO,
            Self::Tx(_) => ETHTOOL_A_RINGS_TX,
            Self::RxBufLen(_) => ETHTOOL_A_RINGS_RX_BUF_LEN,
            Self::TcpDataSplit(_) => ETHTOOL_A_RINGS_TCP_DATA_SPLIT,
            Self::CqeSize(_) => ETHTOOL_A_RINGS_CQE_SIZE,
            Self::TxPush(_) => ETHTOOL_A_RINGS_TX_PUSH,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(out)?,
            Self::RxMax(d)
            | Self::RxMiniMax(d)
            | Self::RxJumboMax(d)
            | Self::TxMax(d)
            | Self::Rx(d)
            | Self::RxMini(d)
            | Self::RxJumbo(d)
            | Self::RxBufLen(d)
            | Self::CqeSize(d)
            | Self::Tx(d) => append_u32(out, *d),
            Self::TcpDataSplit(d) => out.push(*d),
            Self::TxPush(d) => out.push(*d as u8),
            Self::Other(ref attr) => attr.emit_value(out)?,
        }
        Ok(())
    }
}

impl<'a> Parseable<NlaBuffer<'a>> for EthtoolRingAttr {
    fn parse(buf: &NlaBuffer<'a>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_RINGS_HEADER => {
                let mut nlas = Vec::new();
                let error_msg = "failed to parse ring header attributes";
                for nla in NlasIterator::new(payload).context(error_msg)? {
                    let parsed =
                        EthtoolHeader::parse(&nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_RINGS_RX_MAX => Self::RxMax(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_RX_MAX value")?,
            ),

            ETHTOOL_A_RINGS_RX_MINI_MAX => Self::RxMiniMax(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_RX_MINI_MAX value")?,
            ),
            ETHTOOL_A_RINGS_RX_JUMBO_MAX => Self::RxJumboMax(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_RX_JUMBO_MAX value")?,
            ),
            ETHTOOL_A_RINGS_TX_MAX => Self::TxMax(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_TX_MAX value")?,
            ),
            ETHTOOL_A_RINGS_RX => Self::Rx(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_RX value")?,
            ),
            ETHTOOL_A_RINGS_RX_MINI => Self::RxMini(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_RX_MINI value")?,
            ),
            ETHTOOL_A_RINGS_RX_JUMBO => Self::RxJumbo(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_RX_JUMBO value")?,
            ),
            ETHTOOL_A_RINGS_TX => Self::Tx(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_TX value")?,
            ),
            ETHTOOL_A_RINGS_RX_BUF_LEN => Self::RxBufLen(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_RX_BUF_LEN value")?,
            ),
            ETHTOOL_A_RINGS_TCP_DATA_SPLIT => Self::TcpDataSplit(
                parse_u8(payload)
                    .context("Invalid ETHTOOL_A_RINGS_TCP_DATA_SPLIT value")?,
            ),
            ETHTOOL_A_RINGS_CQE_SIZE => Self::CqeSize(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_RINGS_CQE_SIZE value")?,
            ),
            ETHTOOL_A_RINGS_TX_PUSH => Self::TxPush(
                parse_u8(payload)
                    .context("Invalid ETHTOOL_A_RINGS_TX_PUSH value")?
                    > 0,
            ),
            kind => Self::Other(
                DefaultNla::parse(buf)
                    .context(format!("invalid ethtool ring NLA kind {kind}"))?,
            ),
        })
    }
}

pub(crate) fn parse_ring_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    let mut nlas = Vec::new();
    let error_msg = "Failed to parse ethtool ring message attributes";
    for nla in NlasIterator::new(buffer).context(error_msg)? {
        let parsed = EthtoolRingAttr::parse(&nla).context(error_msg)?;
        nlas.push(EthtoolAttr::Ring(parsed));
    }
    Ok(nlas)
}
