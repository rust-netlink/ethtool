// SPDX-License-Identifier: MIT

use anyhow::Context;
use log::warn;
use netlink_packet_utils::{
    nla::NlasIterator, parsers::parse_string, DecodeError,
};

const ETHTOOL_A_BITSET_BITS: u16 = 3;

const ETHTOOL_A_BITSET_BITS_BIT: u16 = 1;

const ETHTOOL_A_BITSET_BIT_INDEX: u16 = 1;
const ETHTOOL_A_BITSET_BIT_NAME: u16 = 2;
const ETHTOOL_A_BITSET_BIT_VALUE: u16 = 3;

pub(crate) fn parse_bitset_bits_nlas(
    raw: &[u8],
) -> Result<Vec<String>, DecodeError> {
    let error_msg = "failed to parse mode bit sets";
    for nla in NlasIterator::new(raw) {
        let nla = &nla.context(error_msg)?;
        if nla.kind() == ETHTOOL_A_BITSET_BITS {
            return parse_bitset_bits_nla(nla.value());
        }
    }
    Err("No ETHTOOL_A_BITSET_BITS NLA found".into())
}

fn parse_bitset_bits_nla(raw: &[u8]) -> Result<Vec<String>, DecodeError> {
    let mut modes = Vec::new();
    let error_msg = "Failed to parse ETHTOOL_A_BITSET_BITS attributes";
    for bit_nla in NlasIterator::new(raw) {
        let bit_nla = &bit_nla.context(error_msg)?;
        match bit_nla.kind() {
            ETHTOOL_A_BITSET_BITS_BIT => {
                let error_msg =
                    "Failed to parse ETHTOOL_A_BITSET_BITS_BIT attributes";
                let nlas = NlasIterator::new(bit_nla.value());
                for nla in nlas {
                    let nla = &nla.context(error_msg)?;
                    let payload = nla.value();
                    match nla.kind() {
                        ETHTOOL_A_BITSET_BIT_INDEX
                        | ETHTOOL_A_BITSET_BIT_VALUE => {
                            // ignored
                        }
                        ETHTOOL_A_BITSET_BIT_NAME => {
                            modes.push(parse_string(payload).context(
                                "Invald ETHTOOL_A_BITSET_BIT_NAME value",
                            )?);
                        }
                        _ => {
                            warn!(
                                "Unknown ETHTOOL_A_BITSET_BITS_BIT {} {:?}",
                                nla.kind(),
                                nla.value(),
                            );
                        }
                    }
                }
            }
            _ => {
                warn!(
                    "Unknown ETHTOOL_A_BITSET_BITS kind {}, {:?}",
                    bit_nla.kind(),
                    bit_nla.value()
                );
            }
        };
    }
    Ok(modes)
}
