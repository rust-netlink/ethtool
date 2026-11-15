// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    append_u32, DecodeError, DefaultNla, Emitable, EncodeError, ErrorContext,
    Nla, NlaBuffer, NlasIterator, Parseable, NLA_F_NESTED,
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

    fn emit_value(&self, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match self {
            Self::Header(ref nlas) => nlas.as_slice().emit(out)?,
            Self::Data(d) => out.extend_from_slice(d),
            Self::Page(d) | Self::Bank(d) | Self::I2CAddress(d) => out.push(*d),
            Self::Offset(d) | Self::Length(d) => append_u32(out, *d),
            Self::Other(ref attr) => attr.emit_value(out)?,
        }
        Ok(())
    }
}

impl<'a> Parseable<NlaBuffer<'a>> for EthtoolModuleEEPROMAttr {
    fn parse(buf: &NlaBuffer<'a>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            ETHTOOL_A_MODULE_EEPROM_HEADER => {
                let mut nlas = Vec::new();
                let error_msg =
                    "failed to parse module eeprom header attributes";
                for nla in NlasIterator::new(payload).context(error_msg)? {
                    let parsed =
                        EthtoolHeader::parse(&nla).context(error_msg)?;
                    nlas.push(parsed);
                }
                Self::Header(nlas)
            }
            ETHTOOL_A_MODULE_EEPROM_DATA => Self::Data(Vec::from(payload)),
            kind => Self::Other(DefaultNla::parse(buf).context(format!(
                "invalid ethtool module eeprom NLA kind {kind}"
            ))?),
        })
    }
}

pub(crate) fn parse_module_eeprom_nlas(
    buffer: &[u8],
) -> Result<Vec<EthtoolAttr>, DecodeError> {
    let mut nlas = Vec::new();
    let error_msg = "Failed to parse ethtool module eeprom message attributes";
    for nla in NlasIterator::new(buffer).context(error_msg)? {
        let parsed = EthtoolModuleEEPROMAttr::parse(&nla).context(error_msg)?;
        nlas.push(EthtoolAttr::ModuleEEPROM(parsed));
    }
    Ok(nlas)
}
