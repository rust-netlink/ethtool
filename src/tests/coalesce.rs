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
    EthtoolAttr, EthtoolCmd, EthtoolCoalesceAttr, EthtoolHeader, EthtoolMessage,
};

#[test]
fn test_parsing_coalesce_netdevsim() {
    let raw: Vec<u8> = vec![
        0x14, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0xa3, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x73, 0x69, 0x6d, 0x30,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x00, 0x0a, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x03, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x06, 0x00, 0x14, 0x00, 0x00, 0x00, 0x08, 0x00, 0x07, 0x00,
        0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x0a, 0x00,
        0x64, 0x00, 0x00, 0x00, 0x05, 0x00, 0x0b, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x0d, 0x00,
        0xe8, 0x03, 0x00, 0x00, 0x08, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x0f, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x10, 0x00,
        0x03, 0x00, 0x00, 0x00, 0x08, 0x00, 0x11, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x12, 0x00, 0xd0, 0x07, 0x00, 0x00, 0x08, 0x00, 0x13, 0x00,
        0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x14, 0x00, 0x06, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x15, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x16, 0x00,
        0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x17, 0x00, 0x09, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x19, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x1c, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    // ethtool -c sim0 after configuring all coalesce parameters
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::CoalesceGetReply,
            nlas: vec![
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::Header(vec![
                    EthtoolHeader::DevIndex(163),
                    EthtoolHeader::DevName("sim0".to_string()),
                ])),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxUsecs(10)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxMaxFrames(5)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxUsecsIrq(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxMaxFramesIrq(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxUsecs(20)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxMaxFrames(6)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxUsecsIrq(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxMaxFramesIrq(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::StatsBlockUsecs(
                    100
                )),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::UseAdaptiveRx(true)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::UseAdaptiveTx(true)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::PktRateLow(1000)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxUsecsLow(1)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxMaxFramesLow(2)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxUsecsLow(3)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxMaxFramesLow(4)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::PktRateHigh(2000)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxUsecsHigh(5)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxMaxFramesHigh(6)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxUsecsHigh(7)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxMaxFramesHigh(8)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RateSampleInterval(
                    9
                )),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::UseCqeModeTx(false)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::UseCqeModeRx(false)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxAggrMaxBytes(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxAggrMaxFrames(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxAggrTimeUsecs(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxCqeFrames(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxCqeNsecs(0)),
            ],
        }
    );
}

#[test]
fn test_parsing_coalesce_virtio() {
    let raw: Vec<u8> = vec![
        0x14, 0x01, 0x00, 0x00, 0x18, 0x00, 0x01, 0x80, 0x08, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x02, 0x00, 0x65, 0x6e, 0x70, 0x32,
        0x73, 0x30, 0x00, 0x00, 0x08, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x07, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    // ethtool -c enp2s0
    let msg = parse_and_round_trip(&raw);

    assert_eq!(
        msg,
        EthtoolMessage {
            cmd: EthtoolCmd::CoalesceGetReply,
            nlas: vec![
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::Header(vec![
                    EthtoolHeader::DevIndex(2),
                    EthtoolHeader::DevName("enp2s0".to_string()),
                ])),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxUsecs(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::RxMaxFrames(1)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxUsecs(0)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::TxMaxFrames(1)),
                EthtoolAttr::Coalesce(EthtoolCoalesceAttr::UseAdaptiveRx(
                    false
                )),
            ],
        }
    );
}
