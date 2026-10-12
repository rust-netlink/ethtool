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
    EthtoolAttr, EthtoolCmd, EthtoolHeader, EthtoolMessage, EthtoolRingAttr,
};
use netlink_packet_core::DefaultNla;

#[test]
fn test_parsing_rings_netdevsim() {
    let raw: Vec<u8> = vec![
        0x10, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0xa3, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x73, 0x69, 0x6d, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x00, 0x00, 0x10, 0x00, 0x00,
        0x08, 0x00, 0x06, 0x00, 0x00, 0x02, 0x00, 0x00, 0x08, 0x00, 0x03, 0x00,
        0x00, 0x10, 0x00, 0x00, 0x08, 0x00, 0x07, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x04, 0x00, 0x00, 0x10, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x08, 0x00, 0x05, 0x00, 0x00, 0x10, 0x00, 0x00,
        0x08, 0x00, 0x09, 0x00, 0x00, 0x02, 0x00, 0x00, 0x05, 0x00, 0x0b, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x11, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x12, 0x00, 0x00, 0x04, 0x00, 0x00,
    ];

    // ethtool -g sim0 after ethtool -G sim0 rx 512 tx 512 rx-mini 8 rx-jumbo 16
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::RingGetReply,
            nlas: vec![
                EthtoolAttr::Ring(EthtoolRingAttr::Header(vec![
                    EthtoolHeader::DevIndex(163),
                    EthtoolHeader::DevName("sim0".to_string()),
                ])),
                EthtoolAttr::Ring(EthtoolRingAttr::RxMax(4096)),
                EthtoolAttr::Ring(EthtoolRingAttr::Rx(512)),
                EthtoolAttr::Ring(EthtoolRingAttr::RxMiniMax(4096)),
                EthtoolAttr::Ring(EthtoolRingAttr::RxMini(8)),
                EthtoolAttr::Ring(EthtoolRingAttr::RxJumboMax(4096)),
                EthtoolAttr::Ring(EthtoolRingAttr::RxJumbo(16)),
                EthtoolAttr::Ring(EthtoolRingAttr::TxMax(4096)),
                EthtoolAttr::Ring(EthtoolRingAttr::Tx(512)),
                EthtoolAttr::Ring(EthtoolRingAttr::TcpDataSplit(2)),
                EthtoolAttr::Ring(EthtoolRingAttr::TxPush(false)),
                // ETHTOOL_A_RINGS_RX_PUSH (14)
                EthtoolAttr::Ring(EthtoolRingAttr::Other(DefaultNla::new(
                    14,
                    vec![0]
                ))),
                // ETHTOOL_A_RINGS_HDS_THRESH (17)
                EthtoolAttr::Ring(EthtoolRingAttr::Other(DefaultNla::new(
                    17,
                    vec![0, 0, 0, 0]
                ))),
                // ETHTOOL_A_RINGS_HDS_THRESH_MAX (18)
                EthtoolAttr::Ring(EthtoolRingAttr::Other(DefaultNla::new(
                    18,
                    vec![0, 4, 0, 0]
                ))),
            ],
        }
    );
}

#[test]
fn test_parsing_rings_virtio() {
    let raw: Vec<u8> = vec![
        0x10, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x02, 0x00, 0x65, 0x6e, 0x70, 0x32,
        0x73, 0x30, 0x00, 0x00, 0x08, 0x00, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x08, 0x00, 0x06, 0x00, 0x00, 0x01, 0x00, 0x00, 0x08, 0x00, 0x05, 0x00,
        0x00, 0x01, 0x00, 0x00, 0x08, 0x00, 0x09, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x05, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x0e, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    // ethtool -g enp2s0
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::RingGetReply,
            nlas: vec![
                EthtoolAttr::Ring(EthtoolRingAttr::Header(vec![
                    EthtoolHeader::DevIndex(2),
                    EthtoolHeader::DevName("enp2s0".to_string()),
                ])),
                EthtoolAttr::Ring(EthtoolRingAttr::RxMax(256)),
                EthtoolAttr::Ring(EthtoolRingAttr::Rx(256)),
                EthtoolAttr::Ring(EthtoolRingAttr::TxMax(256)),
                EthtoolAttr::Ring(EthtoolRingAttr::Tx(256)),
                EthtoolAttr::Ring(EthtoolRingAttr::TxPush(false)),
                // ETHTOOL_A_RINGS_RX_PUSH (14)
                EthtoolAttr::Ring(EthtoolRingAttr::Other(DefaultNla::new(
                    14,
                    vec![0]
                ))),
            ],
        }
    );
}
