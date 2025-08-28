// SPDX-License-Identifier: MIT

use log::warn;
use netlink_packet_core::{
    parse_string, parse_u32, DecodeError, ErrorContext, NlasIterator,
};

const ETHTOOL_A_BITSET_BITS: u16 = 3;

const ETHTOOL_A_BITSET_BITS_BIT: u16 = 1;

const ETHTOOL_A_BITSET_BIT_INDEX: u16 = 1;
const ETHTOOL_A_BITSET_BIT_NAME: u16 = 2;
const ETHTOOL_A_BITSET_BIT_VALUE: u16 = 3;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub(crate) struct EthtoolBitSet {
    pub(crate) index: u32,
    pub(crate) name: String,
    pub(crate) value: bool,
}

pub(crate) fn parse_bitset_bits_nlas(
    raw: &[u8],
) -> Result<Vec<EthtoolBitSet>, DecodeError> {
    let error_msg = "failed to parse mode bit sets";
    for nla in NlasIterator::new(raw) {
        let nla = &nla.context(error_msg)?;
        if nla.kind() == ETHTOOL_A_BITSET_BITS {
            return parse_bitset_bits_nla(nla.value());
        }
    }
    Err("No ETHTOOL_A_BITSET_BITS NLA found".into())
}

pub(crate) fn parse_bitset_bits_string_nlas(
    raw: &[u8],
) -> Result<Vec<String>, DecodeError> {
    let error_msg = "failed to parse mode bit sets";
    for nla in NlasIterator::new(raw) {
        let nla = &nla.context(error_msg)?;
        if nla.kind() == ETHTOOL_A_BITSET_BITS {
            let bits = parse_bitset_bits_nla(nla.value())?;

            return Ok(bits
                .into_iter()
                .filter_map(|b| if b.value { Some(b.name) } else { None })
                .collect::<Vec<String>>());
        }
    }
    Err("No ETHTOOL_A_BITSET_BITS NLA found".into())
}

fn parse_bitset_bits_nla(
    raw: &[u8],
) -> Result<Vec<EthtoolBitSet>, DecodeError> {
    let mut bit_sets = Vec::new();
    let error_msg = "Failed to parse ETHTOOL_A_BITSET_BITS attributes";
    for bit_nla in NlasIterator::new(raw) {
        let bit_nla = &bit_nla.context(error_msg)?;
        match bit_nla.kind() {
            ETHTOOL_A_BITSET_BITS_BIT => {
                let error_msg =
                    "Failed to parse ETHTOOL_A_BITSET_BITS_BIT attributes";
                let mut bit_set = EthtoolBitSet::default();
                let nlas = NlasIterator::new(bit_nla.value());
                for nla in nlas {
                    let nla = &nla.context(error_msg)?;
                    let payload = nla.value();
                    match nla.kind() {
                        ETHTOOL_A_BITSET_BIT_INDEX => {
                            bit_set.index =
                                parse_u32(payload).context(format!(
                                    "Invalid ETHTOOL_A_BITSET_BIT_INDEX \
                                    value {payload:?}"
                                ))?;
                        }
                        ETHTOOL_A_BITSET_BIT_VALUE => {
                            bit_set.value = true;
                        }
                        ETHTOOL_A_BITSET_BIT_NAME => {
                            bit_set.name = parse_string(payload).context(
                                "Invald ETHTOOL_A_BITSET_BIT_NAME value",
                            )?;
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
                bit_sets.push(bit_set);
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
    Ok(bit_sets)
}
