// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    emit_u32, parse_u32, DecodeError, DefaultNla, Emitable, ErrorContext, Nla,
    NlaBuffer, NlasIterator, Parseable, NLA_F_NESTED,
};

use crate::{EthtoolAttr, EthtoolHeader};

const ETHTOOL_A_CHANNELS_HEADER: u16 = 1;
const ETHTOOL_A_CHANNELS_RX_MAX: u16 = 2;
const ETHTOOL_A_CHANNELS_TX_MAX: u16 = 3;
const ETHTOOL_A_CHANNELS_OTHER_MAX: u16 = 4;
const ETHTOOL_A_CHANNELS_COMBINED_MAX: u16 = 5;
const ETHTOOL_A_CHANNELS_RX_COUNT: u16 = 6;
const ETHTOOL_A_CHANNELS_TX_COUNT: u16 = 7;
const ETHTOOL_A_CHANNELS_OTHER_COUNT: u16 = 8;
const ETHTOOL_A_CHANNELS_COMBINED_COUNT: u16 = 9;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolChannelAttr {
    Header(Vec<EthtoolHeader>),
    RxMax(u32),
    TxMax(u32),
    OtherMax(u32),
    CombinedMax(u32),
    RxCount(u32),
    TxCount(u32),
    OtherCount(u32),
    CombinedCount(u32),
    Other(DefaultNla),
}

impl Nla for EthtoolChannelAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Header(hdrs) => hdrs.as_slice().buffer_len(),
            Self::RxMax(_)
            | Self::TxMax(_)
            | Self::OtherMax(_)
            | Self::CombinedMax(_)
            | Self::RxCount(_)
            | Self::TxCount(_)
            | Self::OtherCount(_)
            | Self::CombinedCount(_) => 4,
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_CHANNELS_HEADER | NLA_F_NESTED,
            Self::RxMax(_) => ETHTOOL_A_CHANNELS_RX_MAX,
            Self::TxMax(_) => ETHTOOL_A_CHANNELS_TX_MAX,
            Self::OtherMax(_) => ETHTOOL_A_CHANNELS_OTHER_MAX,
            Self::CombinedMax(_) => ETHTOOL_A_CHANNELS_COMBINED_MAX,
            Self::RxCount(_) => ETHTOOL_A_CHANNELS_RX_COUNT,
            Self::TxCount(_) => ETHTOOL_A_CHANNELS_TX_COUNT,
            Self::OtherCount(_) => ETHTOOL_A_CHANNELS_OTHER_COUNT,
            Self::CombinedCount(_) => ETHTOOL_A_CHANNELS_COMBINED_COUNT,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(buffer),
            Self::RxMax(d)
            | Self::TxMax(d)
            | Self::OtherMax(d)
            | Self::CombinedMax(d)
            | Self::RxCount(d)
            | Self::TxCount(d)
            | Self::OtherCount(d)
            | Self::CombinedCount(d) => emit_u32(buffer, *d).unwrap(),
            Self::Other(ref attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolChannelAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_CHANNELS_HEADER => {
                let mut nlas = Vec::new();
                let error_msg = "failed to parse channel header attributes";
                for nla in NlasIterator::new(payload) {
                    let nla = &nla.context(error_msg)?;
                    let parsed =
                        EthtoolHeader::parse(nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_CHANNELS_RX_MAX => Self::RxMax(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_CHANNELS_RX_MAX value")?,
            ),
            ETHTOOL_A_CHANNELS_TX_MAX => Self::TxMax(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_CHANNELS_RX_MAX value")?,
            ),
            ETHTOOL_A_CHANNELS_OTHER_MAX => Self::OtherMax(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_CHANNELS_RX_MAX value")?,
            ),
            ETHTOOL_A_CHANNELS_COMBINED_MAX => Self::CombinedMax(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_CHANNELS_RX_MAX value")?,
            ),
            ETHTOOL_A_CHANNELS_RX_COUNT => Self::RxCount(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_CHANNELS_RX_COUNT value")?,
            ),
            ETHTOOL_A_CHANNELS_TX_COUNT => Self::TxCount(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_CHANNELS_RX_COUNT value")?,
            ),
            ETHTOOL_A_CHANNELS_OTHER_COUNT => Self::OtherCount(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_CHANNELS_RX_COUNT value")?,
            ),
            ETHTOOL_A_CHANNELS_COMBINED_COUNT => Self::CombinedCount(
                parse_u32(payload)
                    .context("Invalid ETHTOOL_A_CHANNELS_RX_COUNT value")?,
            ),
            kind => {
                Self::Other(DefaultNla::parse(buf).context(format!(
                    "invalid ethtool channel NLA kind {kind}"
                ))?)
            }
        })
    }
}

pub(crate) fn parse_channel_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    let mut nlas = Vec::new();
    for nla in NlasIterator::new(buffer) {
        let error_msg = format!(
            "Failed to parse ethtool channel message attribute {nla:?}"
        );
        let nla = &nla.context(error_msg.clone())?;
        let parsed = EthtoolChannelAttr::parse(nla).context(error_msg)?;
        nlas.push(EthtoolAttr::Channel(parsed));
    }
    Ok(nlas)
}
