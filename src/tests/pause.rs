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
    EthtoolAttr, EthtoolCmd, EthtoolHeader, EthtoolMessage, EthtoolPauseAttr,
    EthtoolPauseParsed, EthtoolPauseStatAttr,
};
use netlink_packet_core::Emitable;
use netlink_packet_generic::GenlHeader;

#[test]
fn test_parsing_pause_netdevsim() {
    let raw: Vec<u8> = vec![
        0x16, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0xa3, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x73, 0x69, 0x6d, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x00, 0x00,
    ];

    // ethtool -a sim0 after ethtool -A sim0 autoneg off rx on tx on
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::PauseGetReply,
            nlas: vec![
                EthtoolAttr::Pause(EthtoolPauseAttr::Header(vec![
                    EthtoolHeader::DevIndex(163),
                    EthtoolHeader::DevName("sim0".to_string()),
                ])),
                EthtoolAttr::Pause(EthtoolPauseAttr::AutoNeg(false)),
                EthtoolAttr::Pause(EthtoolPauseAttr::Rx(true)),
                EthtoolAttr::Pause(EthtoolPauseAttr::Tx(true)),
            ],
        }
    );

    let parsed = EthtoolPauseParsed::from(msg);

    assert_eq!(
        parsed,
        EthtoolPauseParsed {
            iface_name: Some("sim0".to_string()),
            iface_index: Some(163),
            autoneg: Some(false),
            rx: Some(true),
            tx: Some(true),
            ..Default::default()
        }
    );
}

#[test]
fn test_parsing_pause_stats_netdevsim() {
    let raw: Vec<u8> = vec![
        0x16, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0xa3, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x73, 0x69, 0x6d, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x1c, 0x00, 0x05, 0x80, 0x0c, 0x00, 0x02, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    // ethtool -a --include-statistics sim0
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::PauseGetReply,
            nlas: vec![
                EthtoolAttr::Pause(EthtoolPauseAttr::Header(vec![
                    EthtoolHeader::DevIndex(163),
                    EthtoolHeader::DevName("sim0".to_string()),
                ])),
                EthtoolAttr::Pause(EthtoolPauseAttr::AutoNeg(false)),
                EthtoolAttr::Pause(EthtoolPauseAttr::Rx(true)),
                EthtoolAttr::Pause(EthtoolPauseAttr::Tx(true)),
                EthtoolAttr::Pause(EthtoolPauseAttr::StatsSrc(0)),
                EthtoolAttr::Pause(EthtoolPauseAttr::Stats(vec![
                    EthtoolPauseStatAttr::Tx(2),
                    EthtoolPauseStatAttr::Rx(1),
                ])),
            ],
        }
    );

    let parsed = EthtoolPauseParsed::from(msg);

    assert_eq!(
        parsed,
        EthtoolPauseParsed {
            iface_name: Some("sim0".to_string()),
            iface_index: Some(163),
            autoneg: Some(false),
            rx: Some(true),
            tx: Some(true),
            rx_frames: Some(1),
            tx_frames: Some(2),
            stats_src: Some(0),
            ..Default::default()
        }
    );
}

#[test]
fn test_building_pause_set_netdevsim() {
    // Kernel reply to `ethtool -a sim0` with the configuration below
    // applied, captured with an nlmon device on Linux 7.3.
    let raw_reply: Vec<u8> = vec![
        0x16, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0xa3, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x73, 0x69, 0x6d, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x00, 0x00,
    ];

    let parsed = EthtoolPauseParsed::from(parse_and_round_trip(&raw_reply));

    // Request of `ethtool -A sim0 autoneg off rx on tx on`, captured with an
    // nlmon device on Linux 7.3. The kernel resolves the interface from the
    // name, the device index of the reply is not part of the request.
    let raw_request: Vec<u8> = vec![
        0x16, 0x01, 0x00, 0x00, 0x10, 0x00, 0x01, 0x80, 0x09, 0x00, 0x02, 0x00,
        0x73, 0x69, 0x6d, 0x30, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];

    let msg = parsed.build().unwrap();
    let header = GenlHeader {
        cmd: EthtoolCmd::PauseSet.into(),
        version: 1,
    };
    let mut buffer = vec![0; header.buffer_len() + msg.buffer_len()];
    header.emit(&mut buffer);
    msg.emit(&mut buffer[header.buffer_len()..]);

    assert_eq!(buffer, raw_request);
}
