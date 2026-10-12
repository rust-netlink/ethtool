// SPDX-License-Identifier: MIT

use log::warn;
use netlink_packet_core::{
    emit_u32, parse_string, parse_u32, DecodeError, Emitable, ErrorContext,
    Nla, NlasIterator, NLA_F_NESTED,
};

const ETHTOOL_A_BITSET_NOMASK: u16 = 1;
const ETHTOOL_A_BITSET_SIZE: u16 = 2;
const ETHTOOL_A_BITSET_BITS: u16 = 3;
const ETHTOOL_A_BITSET_VALUE: u16 = 4;
const ETHTOOL_A_BITSET_MASK: u16 = 5;

const ETHTOOL_A_BITSET_BITS_BIT: u16 = 1;

const ETHTOOL_A_BITSET_BIT_INDEX: u16 = 1;
const ETHTOOL_A_BITSET_BIT_NAME: u16 = 2;
const ETHTOOL_A_BITSET_BIT_VALUE: u16 = 3;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EthtoolBit {
    pub index: u32,
    pub name: String,
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolBitsetValue {
    /// Verbose (non compact) form: list of bits with names.
    Bits(Vec<EthtoolBit>),
    /// Compact form (`ETHTOOL_FLAG_COMPACT_BITSETS`): value bitmap words
    /// and, unless `EthtoolBitset::nomask` is set, mask bitmap words.
    Compact { value: Vec<u32>, mask: Vec<u32> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EthtoolBitset {
    /// Whether `ETHTOOL_A_BITSET_NOMASK` flag was present, which means
    /// this bitset has no mask.
    pub nomask: bool,
    /// Number of bits in this bitset (`ETHTOOL_A_BITSET_SIZE`).
    pub size: u32,
    pub value: EthtoolBitsetValue,
}

impl EthtoolBitset {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self, DecodeError> {
        let mut nomask = false;
        let mut size = 0u32;
        let mut bits = None;
        let mut value = None;
        let mut mask = None;
        let error_msg = "failed to parse bitset attributes";
        for nla in NlasIterator::new(payload) {
            let nla = &nla.context(error_msg)?;
            match nla.kind() {
                ETHTOOL_A_BITSET_NOMASK => nomask = true,
                ETHTOOL_A_BITSET_SIZE => {
                    size = parse_u32(nla.value())
                        .context("Invalid ETHTOOL_A_BITSET_SIZE value")?
                }
                ETHTOOL_A_BITSET_BITS => {
                    bits = Some(parse_bits(nla.value())?);
                }
                ETHTOOL_A_BITSET_VALUE => {
                    value = Some(parse_u32_words(nla.value())?);
                }
                ETHTOOL_A_BITSET_MASK => {
                    mask = Some(parse_u32_words(nla.value())?);
                }
                kind => {
                    warn!(
                        "Unknown ETHTOOL_A_BITSET_* attribute {kind} {:?}",
                        nla.value()
                    );
                }
            }
        }
        let value = match (bits, value) {
            (Some(mut bits), _) => {
                if nomask {
                    // For bitsets without mask, only set bits are listed,
                    // hence any listed bit is enabled.
                    for bit in bits.iter_mut() {
                        bit.value = true;
                    }
                }
                EthtoolBitsetValue::Bits(bits)
            }
            (None, Some(value)) => EthtoolBitsetValue::Compact {
                value,
                mask: mask.unwrap_or_default(),
            },
            (None, None) => {
                return Err("Invalid bitset: neither BITS nor VALUE".into())
            }
        };
        Ok(Self {
            nomask,
            size,
            value,
        })
    }
}

fn parse_bits(raw: &[u8]) -> Result<Vec<EthtoolBit>, DecodeError> {
    let error_msg = "Failed to parse ETHTOOL_A_BITSET_BITS attributes";
    let mut bits = Vec::new();
    for bit_nla in NlasIterator::new(raw) {
        let bit_nla = &bit_nla.context(error_msg)?;
        match bit_nla.kind() {
            ETHTOOL_A_BITSET_BITS_BIT => {
                bits.push(parse_bit(bit_nla.value())?);
            }
            kind => {
                warn!(
                    "Unknown ETHTOOL_A_BITSET_BITS attribute {kind} {:?}",
                    bit_nla.value()
                );
            }
        }
    }
    Ok(bits)
}

fn parse_bit(raw: &[u8]) -> Result<EthtoolBit, DecodeError> {
    let error_msg = "Failed to parse ETHTOOL_A_BITSET_BITS_BIT attributes";
    let mut bit = EthtoolBit::default();
    let mut index_found = false;
    for nla in NlasIterator::new(raw) {
        let nla = &nla.context(error_msg)?;
        let payload = nla.value();
        match nla.kind() {
            ETHTOOL_A_BITSET_BIT_INDEX => {
                bit.index = parse_u32(payload)
                    .context("Invalid ETHTOOL_A_BITSET_BIT_INDEX value")?;
                index_found = true;
            }
            ETHTOOL_A_BITSET_BIT_NAME => {
                bit.name = parse_string(payload)
                    .context("Invalid ETHTOOL_A_BITSET_BIT_NAME value")?;
            }
            ETHTOOL_A_BITSET_BIT_VALUE => bit.value = true,
            kind => {
                warn!(
                    "Unknown ETHTOOL_A_BITSET_BITS_BIT attribute {kind} {:?}",
                    nla.value()
                );
            }
        }
    }
    if !index_found {
        return Err("Missing ETHTOOL_A_BITSET_BIT_INDEX".into());
    }
    Ok(bit)
}

fn parse_u32_words(raw: &[u8]) -> Result<Vec<u32>, DecodeError> {
    if !raw.len().is_multiple_of(4) {
        return Err(format!(
            "Invalid bitmap length {}, not multiple of 4",
            raw.len()
        )
        .into());
    }
    Ok(raw
        .as_chunks::<4>()
        .0
        .iter()
        .map(|w| u32::from_ne_bytes(*w))
        .collect())
}

const fn align4(len: usize) -> usize {
    (len + 3) & !3
}

const fn nla_len(value_len: usize) -> usize {
    align4(value_len) + 4
}

struct BitsetFlag(u16);

impl Nla for BitsetFlag {
    fn value_len(&self) -> usize {
        0
    }

    fn kind(&self) -> u16 {
        self.0
    }

    fn emit_value(&self, _buffer: &mut [u8]) {}
}

struct BitsetU32(u16, u32);

impl Nla for BitsetU32 {
    fn value_len(&self) -> usize {
        4
    }

    fn kind(&self) -> u16 {
        self.0
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        emit_u32(buffer, self.1).unwrap();
    }
}

struct BitsetStr<'a>(u16, &'a str);

impl Nla for BitsetStr<'_> {
    fn value_len(&self) -> usize {
        self.1.len() + 1
    }

    fn kind(&self) -> u16 {
        self.0
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        buffer[..self.1.len()].copy_from_slice(self.1.as_bytes());
        buffer[self.1.len()] = 0;
    }
}

struct BitsetWords<'a>(u16, &'a [u32]);

impl Nla for BitsetWords<'_> {
    fn value_len(&self) -> usize {
        self.1.len() * 4
    }

    fn kind(&self) -> u16 {
        self.0
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        for (i, word) in self.1.iter().enumerate() {
            emit_u32(&mut buffer[i * 4..], *word).unwrap();
        }
    }
}

struct BitsetBitNla<'a>(&'a EthtoolBit, bool);

impl Nla for BitsetBitNla<'_> {
    fn value_len(&self) -> usize {
        let mut len = nla_len(4); // index
        len += nla_len(self.0.name.len() + 1); // name
        if self.1 && self.0.value {
            len += nla_len(0); // value flag, only for masked bitsets
        }
        len
    }

    fn kind(&self) -> u16 {
        ETHTOOL_A_BITSET_BITS_BIT | NLA_F_NESTED
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        let index = BitsetU32(ETHTOOL_A_BITSET_BIT_INDEX, self.0.index);
        let name = BitsetStr(ETHTOOL_A_BITSET_BIT_NAME, self.0.name.as_str());
        let mut offset = 0;
        index.emit(&mut buffer[offset..]);
        offset += index.buffer_len();
        name.emit(&mut buffer[offset..]);
        offset += name.buffer_len();
        if self.1 && self.0.value {
            let value = BitsetFlag(ETHTOOL_A_BITSET_BIT_VALUE);
            value.emit(&mut buffer[offset..]);
        }
    }
}

struct BitsetBitsNla<'a>(&'a [EthtoolBit], bool);

impl Nla for BitsetBitsNla<'_> {
    fn value_len(&self) -> usize {
        self.0
            .iter()
            .map(|bit| nla_len(BitsetBitNla(bit, self.1).value_len()))
            .sum()
    }

    fn kind(&self) -> u16 {
        ETHTOOL_A_BITSET_BITS | NLA_F_NESTED
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        let mut offset = 0;
        for bit in self.0 {
            let nla = BitsetBitNla(bit, self.1);
            nla.emit(&mut buffer[offset..]);
            offset += nla.buffer_len();
        }
    }
}

impl Emitable for EthtoolBitset {
    fn buffer_len(&self) -> usize {
        let mut len = 0;
        if self.nomask {
            len += nla_len(0);
        }
        len += nla_len(4);
        match &self.value {
            EthtoolBitsetValue::Bits(bits) => {
                len += nla_len(BitsetBitsNla(bits, !self.nomask).value_len());
            }
            EthtoolBitsetValue::Compact { value, mask } => {
                len += nla_len(value.len() * 4);
                if !self.nomask {
                    len += nla_len(mask.len() * 4);
                }
            }
        }
        len
    }

    fn emit(&self, buffer: &mut [u8]) {
        let mut offset = 0;
        if self.nomask {
            let nla = BitsetFlag(ETHTOOL_A_BITSET_NOMASK);
            nla.emit(&mut buffer[offset..]);
            offset += nla.buffer_len();
        }
        let size = BitsetU32(ETHTOOL_A_BITSET_SIZE, self.size);
        size.emit(&mut buffer[offset..]);
        offset += size.buffer_len();
        match &self.value {
            EthtoolBitsetValue::Bits(bits) => {
                let nla = BitsetBitsNla(bits, !self.nomask);
                nla.emit(&mut buffer[offset..]);
            }
            EthtoolBitsetValue::Compact { value, mask } => {
                let nla = BitsetWords(ETHTOOL_A_BITSET_VALUE, value);
                nla.emit(&mut buffer[offset..]);
                offset += nla.buffer_len();
                if !self.nomask {
                    let nla = BitsetWords(ETHTOOL_A_BITSET_MASK, mask);
                    nla.emit(&mut buffer[offset..]);
                }
            }
        }
    }
}
