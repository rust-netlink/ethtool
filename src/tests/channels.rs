// SPDX-License-Identifier: MIT

use super::parse_and_round_trip;
use crate::{
    EthtoolAttr, EthtoolChannelAttr, EthtoolCmd, EthtoolHeader, EthtoolMessage,
};
use netlink_packet_core::{Emitable, ParseableParametrized};
use netlink_packet_generic::GenlHeader;

#[test]
fn test_channels_get_reply() {
    let raw: Vec<u8> = vec![
        0x12, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x65, 0x74, 0x68, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x05, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x09, 0x00, 0x02, 0x00, 0x00, 0x00,
    ];

    let expected = EthtoolMessage {
        cmd: EthtoolCmd::ChannelGetReply,
        nlas: vec![
            EthtoolAttr::Channel(EthtoolChannelAttr::Header(vec![
                EthtoolHeader::DevIndex(2),
                EthtoolHeader::DevName("eth0".to_string()),
            ])),
            EthtoolAttr::Channel(EthtoolChannelAttr::CombinedMax(4)),
            EthtoolAttr::Channel(EthtoolChannelAttr::CombinedCount(2)),
        ],
    };

    let header = GenlHeader::parse(&raw[..]).unwrap();

    assert_eq!(
        expected,
        EthtoolMessage::parse_with_param(&raw[4..], header).unwrap(),
    );

    let mut buffer = vec![0; expected.buffer_len() + header.buffer_len()];
    header.emit(&mut buffer);
    expected.emit(&mut buffer[4..]);
    assert_eq!(&buffer, &raw);
}

#[test]
fn test_channels_set_rx() {
    let expected: Vec<u8> = vec![
        0x10, 0x00, 0x01, 0x80, 0x09, 0x00, 0x02, 0x00, 0x65, 0x74, 0x68, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x06, 0x00, 0x02, 0x00, 0x00, 0x00,
    ];

    let msg = EthtoolMessage {
        cmd: EthtoolCmd::ChannelSet,
        nlas: vec![
            EthtoolAttr::Channel(EthtoolChannelAttr::Header(vec![
                EthtoolHeader::DevName("eth0".to_string()),
            ])),
            EthtoolAttr::Channel(EthtoolChannelAttr::RxCount(2)),
        ],
    };

    let mut raw = vec![0; msg.buffer_len()];
    msg.emit(&mut raw);

    assert_eq!(expected, raw,);
}

#[test]
fn test_channels_set_tx() {
    let expected: Vec<u8> = vec![
        0x10, 0x00, 0x01, 0x80, 0x09, 0x00, 0x02, 0x00, 0x65, 0x74, 0x68, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x07, 0x00, 0x02, 0x00, 0x00, 0x00,
    ];

    let msg = EthtoolMessage {
        cmd: EthtoolCmd::ChannelSet,
        nlas: vec![
            EthtoolAttr::Channel(EthtoolChannelAttr::Header(vec![
                EthtoolHeader::DevName("eth0".to_string()),
            ])),
            EthtoolAttr::Channel(EthtoolChannelAttr::TxCount(2)),
        ],
    };

    let mut raw = vec![0; msg.buffer_len()];
    msg.emit(&mut raw);

    assert_eq!(expected, raw,);
}

#[test]
fn test_channels_set_other() {
    let expected: Vec<u8> = vec![
        0x10, 0x00, 0x01, 0x80, 0x09, 0x00, 0x02, 0x00, 0x65, 0x74, 0x68, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x02, 0x00, 0x00, 0x00,
    ];

    let msg = EthtoolMessage {
        cmd: EthtoolCmd::ChannelSet,
        nlas: vec![
            EthtoolAttr::Channel(EthtoolChannelAttr::Header(vec![
                EthtoolHeader::DevName("eth0".to_string()),
            ])),
            EthtoolAttr::Channel(EthtoolChannelAttr::OtherCount(2)),
        ],
    };

    let mut raw = vec![0; msg.buffer_len()];
    msg.emit(&mut raw);

    assert_eq!(expected, raw,);
}

#[test]
fn test_channels_set_combined() {
    let expected: Vec<u8> = vec![
        0x10, 0x00, 0x01, 0x80, 0x09, 0x00, 0x02, 0x00, 0x65, 0x74, 0x68, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x09, 0x00, 0x02, 0x00, 0x00, 0x00,
    ];

    let msg = EthtoolMessage {
        cmd: EthtoolCmd::ChannelSet,
        nlas: vec![
            EthtoolAttr::Channel(EthtoolChannelAttr::Header(vec![
                EthtoolHeader::DevName("eth0".to_string()),
            ])),
            EthtoolAttr::Channel(EthtoolChannelAttr::CombinedCount(2)),
        ],
    };

    let mut raw = vec![0; msg.buffer_len()];
    msg.emit(&mut raw);

    assert_eq!(expected, raw,);
}
#[test]
fn test_parsing_channels_netdevsim() {
    let raw: Vec<u8> = vec![
        0x12, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0xa3, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x73, 0x69, 0x6d, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x09, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];

    // ethtool -l sim0 after ethtool -L sim0 combined 1
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::ChannelGetReply,
            nlas: vec![
                EthtoolAttr::Channel(EthtoolChannelAttr::Header(vec![
                    EthtoolHeader::DevIndex(163),
                    EthtoolHeader::DevName("sim0".to_string()),
                ])),
                EthtoolAttr::Channel(EthtoolChannelAttr::CombinedMax(1)),
                EthtoolAttr::Channel(EthtoolChannelAttr::CombinedCount(1)),
            ],
        }
    );
}

#[test]
fn test_parsing_channels_veth() {
    let raw: Vec<u8> = vec![
        0x12, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0x00, 0x01, 0x00, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x76, 0x65, 0x74, 0x68,
        0x30, 0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x03, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x08, 0x00, 0x07, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];

    // ethtool -l veth0
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::ChannelGetReply,
            nlas: vec![
                EthtoolAttr::Channel(EthtoolChannelAttr::Header(vec![
                    EthtoolHeader::DevIndex(256),
                    EthtoolHeader::DevName("veth0".to_string()),
                ])),
                EthtoolAttr::Channel(EthtoolChannelAttr::RxMax(16)),
                EthtoolAttr::Channel(EthtoolChannelAttr::RxCount(1)),
                EthtoolAttr::Channel(EthtoolChannelAttr::TxMax(16)),
                EthtoolAttr::Channel(EthtoolChannelAttr::TxCount(1)),
            ],
        }
    );
}
