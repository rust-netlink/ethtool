// SPDX-License-Identifier: MIT

use anyhow::Context;
use byteorder::{ByteOrder, NativeEndian};
use netlink_packet_utils::{
    nla::{DefaultNla, Nla, NlaBuffer, NlasIterator, NLA_F_NESTED},
    DecodeError, Emitable, Parseable,
};

use crate::{EthtoolAttr, EthtoolHeader};


const ETHTOOL_A_MODULE_EEPROM_HEADER: u16 = 1;
const ETHTOOL_A_MODULE_EEPROM_OFFSET: u16 = 2;
const ETHTOOL_A_MODULE_EEPROM_LENGTH: u16 = 3;
const ETHTOOL_A_MODULE_EEPROM_PAGE: u16 = 4;
const ETHTOOL_A_MODULE_EEPROM_BANK: u16 = 5;
const ETHTOOL_A_MODULE_EEPROM_I2C_ADDRESS: u16 = 6;
const ETHTOOL_A_MODULE_EEPROM_DATA: u16 = 7;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolModuleEEPROMAttr {
    Header(Vec<EthtoolHeader>),
    Offset(u32),
    Length(u32),
    Page(u8),
    Bank(u8),
    I2CAddress(u8),
    Data(Vec<u8>),
    Other(DefaultNla),
}

impl Nla for EthtoolModuleEEPROMAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Header(hdrs) => hdrs.as_slice().buffer_len(),
            Self::Data(data) => data.len(),
            Self::Page(_)
            | Self::Bank(_)
            | Self::I2CAddress(_) => 1,
            Self::Offset(_)
            | Self::Length(_) => 4,
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Header(_) => ETHTOOL_A_MODULE_EEPROM_HEADER | NLA_F_NESTED,
            Self::Offset(_) => ETHTOOL_A_MODULE_EEPROM_OFFSET,
            Self::Length(_) => ETHTOOL_A_MODULE_EEPROM_LENGTH,
            Self::Page(_) => ETHTOOL_A_MODULE_EEPROM_PAGE,
            Self::Bank(_) => ETHTOOL_A_MODULE_EEPROM_BANK,
            Self::I2CAddress(_) => ETHTOOL_A_MODULE_EEPROM_I2C_ADDRESS,
            Self::Data(_) => ETHTOOL_A_MODULE_EEPROM_DATA,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(buffer),
            Self::Data(d) => buffer.copy_from_slice(d.as_slice()),
            Self::Page(d)
            | Self::Bank(d)
            | Self::I2CAddress(d) => buffer[0] = *d,
            Self::Offset(d)
            | Self::Length(d) => NativeEndian::write_u32(buffer, *d),
            Self::Other(ref attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolModuleEEPROMAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_MODULE_EEPROM_HEADER => {
                let mut nlas = Vec::new();
                let error_msg = "failed to parse module eeprom header attributes";
                for nla in NlasIterator::new(payload) {
                    let nla = &nla.context(error_msg)?;
                    let parsed =
                        EthtoolHeader::parse(nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_MODULE_EEPROM_DATA => Self::Data(
                Vec::from(payload),
            ),
            kind => Self::Other(
                DefaultNla::parse(buf)
                    .context(format!("invalid ethtool module eeprom NLA kind {kind}"))?,
            ),
        })
    }
}

pub(crate) fn parse_module_eeprom_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    let mut nlas = Vec::new();
    for nla in NlasIterator::new(buffer) {
        let error_msg =
            format!("Failed to parse ethtool module eeprom message attribute {nla:?}");
        let nla = &nla.context(error_msg.clone())?;
        let parsed = EthtoolModuleEEPROMAttr::parse(nla).context(error_msg)?;
        nlas.push(EthtoolAttr::ModuleEEPROM(parsed));
    }
    Ok(nlas)
}
