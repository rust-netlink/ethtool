// SPDX-License-Identifier: MIT

use super::attr::EthtoolRingAttr;
use crate::{
    header::ALTIFNAMSIZ, EthtoolAttr, EthtoolCmd, EthtoolError, EthtoolHeader,
    EthtoolMessage,
};

const ETHTOOL_TCP_DATA_SPLIT_UNKNOWN: u8 = 0;
const ETHTOOL_TCP_DATA_SPLIT_DISABLED: u8 = 1;
const ETHTOOL_TCP_DATA_SPLIT_ENABLED: u8 = 2;

/// Header/data split of TCP packets into separate receive buffers
/// (`ETHTOOL_A_RINGS_TCP_DATA_SPLIT`, `ethtool -G eth1 tcp-data-split`).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EthtoolTcpDataSplit {
    /// In a reply: the driver does not report the state. In a set request:
    /// let the driver choose (`tcp-data-split auto`).
    Unknown,
    Disabled,
    Enabled,
    Other(u8),
}

impl From<u8> for EthtoolTcpDataSplit {
    fn from(d: u8) -> Self {
        match d {
            ETHTOOL_TCP_DATA_SPLIT_UNKNOWN => Self::Unknown,
            ETHTOOL_TCP_DATA_SPLIT_DISABLED => Self::Disabled,
            ETHTOOL_TCP_DATA_SPLIT_ENABLED => Self::Enabled,
            _ => Self::Other(d),
        }
    }
}

impl From<EthtoolTcpDataSplit> for u8 {
    fn from(v: EthtoolTcpDataSplit) -> u8 {
        match v {
            EthtoolTcpDataSplit::Unknown => ETHTOOL_TCP_DATA_SPLIT_UNKNOWN,
            EthtoolTcpDataSplit::Disabled => ETHTOOL_TCP_DATA_SPLIT_DISABLED,
            EthtoolTcpDataSplit::Enabled => ETHTOOL_TCP_DATA_SPLIT_ENABLED,
            EthtoolTcpDataSplit::Other(d) => d,
        }
    }
}

/// Ring parameters of one interface, as `ethtool -g` reports them.
///
/// A field holding `None` is not emitted by [EthtoolRingParsed::build], so
/// a configuration parsed from a reply can be modified and applied without
/// changing what was not touched.
///
/// The maximum sizes ([EthtoolRingParsed::rx_max],
/// [EthtoolRingParsed::rx_mini_max], [EthtoolRingParsed::rx_jumbo_max] and
/// [EthtoolRingParsed::tx_max]) are read-only, the kernel rejects them in a
/// set request, therefore [EthtoolRingParsed::build] ignores them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct EthtoolRingParsed {
    pub iface_name: Option<String>,
    pub iface_index: Option<u32>,
    /// Read-only, ignored by [EthtoolRingParsed::build].
    pub rx_max: Option<u32>,
    /// Read-only, ignored by [EthtoolRingParsed::build].
    pub rx_mini_max: Option<u32>,
    /// Read-only, ignored by [EthtoolRingParsed::build].
    pub rx_jumbo_max: Option<u32>,
    /// Read-only, ignored by [EthtoolRingParsed::build].
    pub tx_max: Option<u32>,
    pub rx: Option<u32>,
    pub rx_mini: Option<u32>,
    pub rx_jumbo: Option<u32>,
    pub tx: Option<u32>,
    pub rx_buf_len: Option<u32>,
    pub tcp_data_split: Option<EthtoolTcpDataSplit>,
    pub cqe_size: Option<u32>,
    pub tx_push: Option<bool>,
}

impl From<EthtoolMessage> for EthtoolRingParsed {
    fn from(msg: EthtoolMessage) -> Self {
        let mut parsed = Self::default();
        for nla in msg.nlas {
            match nla {
                EthtoolAttr::Ring(ring_attr) => match ring_attr {
                    EthtoolRingAttr::Header(headers) => {
                        for header in headers {
                            match header {
                                EthtoolHeader::DevIndex(index) => {
                                    parsed.iface_index = Some(index)
                                }
                                EthtoolHeader::DevName(name) => {
                                    parsed.iface_name = Some(name)
                                }
                                _ => {
                                    log::debug!(
                                        "Unsupported ring header {header:?}"
                                    );
                                }
                            }
                        }
                    }
                    EthtoolRingAttr::RxMax(v) => parsed.rx_max = Some(v),
                    EthtoolRingAttr::RxMiniMax(v) => {
                        parsed.rx_mini_max = Some(v)
                    }
                    EthtoolRingAttr::RxJumboMax(v) => {
                        parsed.rx_jumbo_max = Some(v)
                    }
                    EthtoolRingAttr::TxMax(v) => parsed.tx_max = Some(v),
                    EthtoolRingAttr::Rx(v) => parsed.rx = Some(v),
                    EthtoolRingAttr::RxMini(v) => parsed.rx_mini = Some(v),
                    EthtoolRingAttr::RxJumbo(v) => parsed.rx_jumbo = Some(v),
                    EthtoolRingAttr::Tx(v) => parsed.tx = Some(v),
                    EthtoolRingAttr::RxBufLen(v) => parsed.rx_buf_len = Some(v),
                    EthtoolRingAttr::TcpDataSplit(v) => {
                        parsed.tcp_data_split = Some(v.into())
                    }
                    EthtoolRingAttr::CqeSize(v) => parsed.cqe_size = Some(v),
                    EthtoolRingAttr::TxPush(v) => parsed.tx_push = Some(v),
                    EthtoolRingAttr::Other(nla) => {
                        log::debug!("Unsupported EthtoolRingAttr {nla:?}");
                    }
                },
                nla => log::debug!("Unsupported EthtoolAttr {nla:?}"),
            }
        }
        parsed
    }
}

impl EthtoolRingParsed {
    /// Build the `RINGS_SET` [EthtoolMessage].
    ///
    /// Only fields holding `Some` are emitted, the kernel keeps the current
    /// value of the other ring parameters. The read-only maximum sizes are
    /// ignored.
    ///
    /// This fails with [EthtoolError::InvalidInput] when neither
    /// [EthtoolRingParsed::iface_name] nor [EthtoolRingParsed::iface_index]
    /// is set or when the interface name is invalid.
    pub fn build(&self) -> Result<EthtoolMessage, EthtoolError> {
        let mut header = Vec::new();
        // The kernel resolves the interface from either the name or the
        // index, sending both makes no sense. The name wins, like
        // `ethtool -G` does.
        if let Some(iface_name) = self.iface_name.as_ref() {
            if iface_name.len() >= ALTIFNAMSIZ {
                return Err(EthtoolError::InvalidInput(format!(
                    "`iface_name` can not be longer than {} bytes, but \
                     `{iface_name}` holds {} bytes",
                    ALTIFNAMSIZ - 1,
                    iface_name.len()
                )));
            }
            if iface_name.contains('\0') {
                return Err(EthtoolError::InvalidInput(
                    "`iface_name` can not hold a NUL byte, the kernel would \
                     use the name in front of it"
                        .to_string(),
                ));
            }
            header.push(EthtoolHeader::DevName(iface_name.clone()));
        } else if let Some(iface_index) = self.iface_index {
            header.push(EthtoolHeader::DevIndex(iface_index));
        } else {
            return Err(EthtoolError::InvalidInput(
                "Neither `iface_name` nor `iface_index` is defined".to_string(),
            ));
        }

        let mut nlas = vec![EthtoolAttr::Ring(EthtoolRingAttr::Header(header))];
        let settable = [
            self.rx.map(EthtoolRingAttr::Rx),
            self.rx_mini.map(EthtoolRingAttr::RxMini),
            self.rx_jumbo.map(EthtoolRingAttr::RxJumbo),
            self.tx.map(EthtoolRingAttr::Tx),
            self.rx_buf_len.map(EthtoolRingAttr::RxBufLen),
            self.tcp_data_split
                .map(|v| EthtoolRingAttr::TcpDataSplit(v.into())),
            self.cqe_size.map(EthtoolRingAttr::CqeSize),
            self.tx_push.map(EthtoolRingAttr::TxPush),
        ];
        nlas.extend(settable.into_iter().flatten().map(EthtoolAttr::Ring));

        Ok(EthtoolMessage {
            cmd: EthtoolCmd::RingSet,
            nlas,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tcp_data_split_converts_from_and_to_u8() {
        for (raw, split) in [
            (0, EthtoolTcpDataSplit::Unknown),
            (1, EthtoolTcpDataSplit::Disabled),
            (2, EthtoolTcpDataSplit::Enabled),
            (3, EthtoolTcpDataSplit::Other(3)),
        ] {
            assert_eq!(EthtoolTcpDataSplit::from(raw), split);
            assert_eq!(u8::from(split), raw);
        }
    }

    #[test]
    fn build_requires_an_interface() {
        let err = EthtoolRingParsed::default().build().unwrap_err();

        assert!(matches!(err, EthtoolError::InvalidInput(_)));
    }

    #[test]
    fn build_ignores_read_only_fields() {
        let parsed = EthtoolRingParsed {
            iface_name: Some("eth1".to_string()),
            iface_index: Some(3),
            rx_max: Some(4096),
            rx_mini_max: Some(4096),
            rx_jumbo_max: Some(4096),
            tx_max: Some(4096),
            tcp_data_split: Some(EthtoolTcpDataSplit::Enabled),
            ..Default::default()
        };

        let msg = parsed.build().unwrap();

        assert_eq!(
            msg.nlas,
            vec![
                EthtoolAttr::Ring(EthtoolRingAttr::Header(vec![
                    EthtoolHeader::DevName("eth1".to_string())
                ])),
                EthtoolAttr::Ring(EthtoolRingAttr::TcpDataSplit(2)),
            ]
        );
    }
}
