// SPDX-License-Identifier: MIT

// Generated from kernel emitted ethtool netlink packets captured with an
// nlmon device (`modprobe nlmon`, `ip link add nl0 type nlmon`,
// `ip link set nl0 up`, `tcpdump -i nl0 -w capture.pcap`) on Linux 7.2 and
// inlined here. The 16 bytes netlink header is stripped, so the data starts
// at the generic netlink header exactly like the buffers parsed by
// `EthtoolMessage::parse_with_param()`. Each test verifies the captured
// packet is parsed into the expected attributes and emitted back byte for
// byte.

use super::parse_and_round_trip;
use crate::{
    EthtoolAttr, EthtoolBitset, EthtoolBitsetValue, EthtoolCmd, EthtoolHeader,
    EthtoolLinkModeAttr, EthtoolLinkModeDuplex, EthtoolMessage,
};
use netlink_packet_core::DefaultNla;

#[test]
fn test_parsing_link_modes_virtio() {
    let raw: Vec<u8> = vec![
        0x04, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x02, 0x00, 0x65, 0x6e, 0x70, 0x32,
        0x73, 0x30, 0x00, 0x00, 0x05, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x03, 0x80, 0x08, 0x00, 0x02, 0x00, 0x7d, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x03, 0x80, 0x08, 0x00, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff,
        0x05, 0x00, 0x06, 0x00, 0xff, 0x00, 0x00, 0x00, 0x05, 0x00, 0x0a, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    // ethtool enp2s0
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::LinkModeGetReply,
            nlas: vec![
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Header(vec![
                    EthtoolHeader::DevIndex(2),
                    EthtoolHeader::DevName("enp2s0".to_string()),
                ])),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Autoneg(false)),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Ours(
                    EthtoolBitset {
                        nomask: false,
                        size: 125,
                        value: EthtoolBitsetValue::Bits(vec![]),
                    }
                )),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Speed(u32::MAX)),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Duplex(
                    EthtoolLinkModeDuplex::Unknown
                )),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Other(
                    DefaultNla::new(10, vec![0])
                )),
            ],
        }
    );
}

#[test]
fn test_parsing_link_modes_veth() {
    let raw: Vec<u8> = vec![
        0x04, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0x00, 0x01, 0x00, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x76, 0x65, 0x74, 0x68,
        0x30, 0x00, 0x00, 0x00, 0x05, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x03, 0x80, 0x08, 0x00, 0x02, 0x00, 0x7d, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x03, 0x80, 0x08, 0x00, 0x05, 0x00, 0x10, 0x27, 0x00, 0x00,
        0x05, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x00, 0x0a, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    // ethtool veth0
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::LinkModeGetReply,
            nlas: vec![
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Header(vec![
                    EthtoolHeader::DevIndex(256),
                    EthtoolHeader::DevName("veth0".to_string()),
                ])),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Autoneg(false)),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Ours(
                    EthtoolBitset {
                        nomask: false,
                        size: 125,
                        value: EthtoolBitsetValue::Bits(vec![]),
                    }
                )),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Speed(10000)),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Duplex(
                    EthtoolLinkModeDuplex::Full
                )),
                EthtoolAttr::LinkMode(EthtoolLinkModeAttr::Other(
                    DefaultNla::new(10, vec![0])
                )),
            ],
        }
    );
}
