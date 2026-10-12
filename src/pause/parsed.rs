// SPDX-License-Identifier: MIT

use super::attr::{EthtoolPauseAttr, EthtoolPauseStatAttr};
use crate::{
    header::ALTIFNAMSIZ, EthtoolAttr, EthtoolCmd, EthtoolError, EthtoolHeader,
    EthtoolMessage,
};

/// Pause configuration of one interface, as `ethtool -a` reports it.
///
/// A field holding `None` is not emitted by [EthtoolPauseParsed::build], so
/// a configuration parsed from a reply can be modified and applied without
/// changing what was not touched.
///
/// The statistics fields ([EthtoolPauseParsed::rx_frames],
/// [EthtoolPauseParsed::tx_frames],
/// [EthtoolPauseParsed::tx_pause_storm_events] and
/// [EthtoolPauseParsed::stats_src]) are read-only, the kernel rejects them
/// in a set request, therefore [EthtoolPauseParsed::build] ignores them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct EthtoolPauseParsed {
    pub iface_name: Option<String>,
    pub iface_index: Option<u32>,
    pub autoneg: Option<bool>,
    pub rx: Option<bool>,
    pub tx: Option<bool>,
    /// Number of received pause frames.
    pub rx_frames: Option<u64>,
    /// Number of transmitted pause frames.
    pub tx_frames: Option<u64>,
    /// Read-only, ignored by [EthtoolPauseParsed::build].
    pub tx_pause_storm_events: Option<u64>,
    /// Source of the statistics, `ETHTOOL_MAC_STATS_SRC_*`.
    pub stats_src: Option<u32>,
}

impl From<EthtoolMessage> for EthtoolPauseParsed {
    fn from(msg: EthtoolMessage) -> Self {
        let mut parsed = Self::default();
        for nla in msg.nlas {
            match nla {
                EthtoolAttr::Pause(pause_attr) => match pause_attr {
                    EthtoolPauseAttr::Header(headers) => {
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
                                        "Unsupported pause header {header:?}"
                                    );
                                }
                            }
                        }
                    }
                    EthtoolPauseAttr::AutoNeg(value) => {
                        parsed.autoneg = Some(value)
                    }
                    EthtoolPauseAttr::Rx(value) => parsed.rx = Some(value),
                    EthtoolPauseAttr::Tx(value) => parsed.tx = Some(value),
                    EthtoolPauseAttr::Stats(stats) => {
                        for stat in stats {
                            match stat {
                                EthtoolPauseStatAttr::Rx(value) => {
                                    parsed.rx_frames = Some(value)
                                }
                                EthtoolPauseStatAttr::Tx(value) => {
                                    parsed.tx_frames = Some(value)
                                }
                                EthtoolPauseStatAttr::TxPauseStormEvents(
                                    value,
                                ) => parsed.tx_pause_storm_events = Some(value),
                                EthtoolPauseStatAttr::Other(nla) => {
                                    log::debug!(
                                        "Unsupported pause stat attribute \
                                         {nla:?}"
                                    );
                                }
                            }
                        }
                    }
                    EthtoolPauseAttr::StatsSrc(value) => {
                        parsed.stats_src = Some(value)
                    }
                    EthtoolPauseAttr::Other(nla) => {
                        log::debug!("Unsupported EthtoolPauseAttr {nla:?}");
                    }
                },
                nla => log::debug!("Unsupported EthtoolAttr {nla:?}"),
            }
        }
        parsed
    }
}

impl EthtoolPauseParsed {
    /// Build the `PAUSE_SET` [EthtoolMessage].
    ///
    /// Only fields holding `Some` are emitted, the kernel keeps the current
    /// value of the other pause properties. The read-only statistics fields
    /// and the statistics source are ignored.
    ///
    /// This fails with [EthtoolError::InvalidInput] when neither
    /// [EthtoolPauseParsed::iface_name] nor
    /// [EthtoolPauseParsed::iface_index] is set or when the interface name
    /// is invalid.
    pub fn build(&self) -> Result<EthtoolMessage, EthtoolError> {
        let mut header = Vec::new();
        // The kernel resolves the interface from either the name or the
        // index, sending both makes no sense. The name wins, like
        // `ethtool -A` does.
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

        let mut nlas =
            vec![EthtoolAttr::Pause(EthtoolPauseAttr::Header(header))];
        if let Some(autoneg) = self.autoneg {
            nlas.push(EthtoolAttr::Pause(EthtoolPauseAttr::AutoNeg(autoneg)));
        }
        if let Some(rx) = self.rx {
            nlas.push(EthtoolAttr::Pause(EthtoolPauseAttr::Rx(rx)));
        }
        if let Some(tx) = self.tx {
            nlas.push(EthtoolAttr::Pause(EthtoolPauseAttr::Tx(tx)));
        }

        Ok(EthtoolMessage {
            cmd: EthtoolCmd::PauseSet,
            nlas,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_uses_iface_name_when_both_are_defined() {
        // A parsed reply holds both, the kernel only needs one.
        let parsed = EthtoolPauseParsed {
            iface_name: Some("eth1".to_string()),
            iface_index: Some(3),
            ..Default::default()
        };

        let msg = parsed.build().unwrap();

        assert_eq!(
            msg.nlas,
            vec![EthtoolAttr::Pause(EthtoolPauseAttr::Header(vec![
                EthtoolHeader::DevName("eth1".to_string())
            ]))]
        );
    }

    #[test]
    fn build_accepts_iface_index_only() {
        let parsed = EthtoolPauseParsed {
            iface_index: Some(3),
            ..Default::default()
        };

        let msg = parsed.build().unwrap();

        assert_eq!(
            msg.nlas,
            vec![EthtoolAttr::Pause(EthtoolPauseAttr::Header(vec![
                EthtoolHeader::DevIndex(3)
            ]))]
        );
    }

    #[test]
    fn build_requires_an_interface() {
        let err = EthtoolPauseParsed::default().build().unwrap_err();

        assert!(matches!(err, EthtoolError::InvalidInput(_)));
    }

    #[test]
    fn iface_name_is_limited_to_altifnamsiz() {
        let longest = EthtoolPauseParsed {
            iface_name: Some("a".repeat(ALTIFNAMSIZ - 1)),
            ..Default::default()
        };
        assert!(longest.build().is_ok());

        let too_long = EthtoolPauseParsed {
            iface_name: Some("a".repeat(ALTIFNAMSIZ)),
            ..Default::default()
        };
        assert!(matches!(
            too_long.build().unwrap_err(),
            EthtoolError::InvalidInput(_)
        ));

        // The kernel would use the name in front of an embedded NUL.
        let with_nul = EthtoolPauseParsed {
            iface_name: Some("eth0\0eth1".to_string()),
            ..Default::default()
        };
        assert!(matches!(
            with_nul.build().unwrap_err(),
            EthtoolError::InvalidInput(_)
        ));
    }

    #[test]
    fn build_ignores_read_only_fields() {
        let parsed = EthtoolPauseParsed {
            iface_name: Some("eth1".to_string()),
            autoneg: Some(true),
            rx_frames: Some(1),
            tx_frames: Some(2),
            tx_pause_storm_events: Some(3),
            stats_src: Some(1),
            ..Default::default()
        };

        let msg = parsed.build().unwrap();

        assert_eq!(
            msg.nlas,
            vec![
                EthtoolAttr::Pause(EthtoolPauseAttr::Header(vec![
                    EthtoolHeader::DevName("eth1".to_string())
                ])),
                EthtoolAttr::Pause(EthtoolPauseAttr::AutoNeg(true)),
            ]
        );
    }
}
