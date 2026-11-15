// SPDX-License-Identifier: MIT

mod channels;
mod coalesce;
mod features;
mod fec;
mod link_modes;
mod pause;
mod rings;
mod tsinfo;

use crate::EthtoolMessage;
use netlink_packet_core::{Emitable, ParseableParametrized};
use netlink_packet_generic::GenlHeader;

/// Parse a captured generic netlink payload (starting at the generic netlink
/// header, the 16 bytes netlink header having been stripped) and verify it
/// can be emitted back byte for byte.
fn parse_and_round_trip(raw: &[u8]) -> EthtoolMessage {
    let header = GenlHeader::parse(raw).unwrap();
    let msg = EthtoolMessage::parse_with_param(&raw[4..], header).unwrap();
    let mut buffer = Vec::new();
    header.emit(&mut buffer).unwrap();
    msg.emit(&mut buffer).unwrap();
    assert_eq!(buffer, raw);
    msg
}
