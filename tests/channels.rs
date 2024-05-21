use ethtool::{
    EthtoolAttr, EthtoolChannelAttr, EthtoolCmd, EthtoolHeader, EthtoolMessage,
};
use netlink_packet_generic::{GenlBuffer, GenlHeader};
use netlink_packet_utils::{Emitable, Parseable, ParseableParametrized};

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

    assert_eq!(
        expected,
        EthtoolMessage::parse_with_param(
            &raw[4..],
            GenlHeader::parse(&GenlBuffer::new(&raw)).unwrap()
        )
        .unwrap(),
    );
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
