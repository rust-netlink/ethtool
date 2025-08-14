// SPDX-License-Identifier: MIT

mod attr;
mod get;
mod handle;

pub(crate) use attr::parse_module_eeprom_nlas;

pub use attr::EthtoolModuleEEPROMAttr;
pub use get::EthtoolModuleEEPROMGetRequest;
pub use handle::EthtoolModuleEEPROMHandle;
