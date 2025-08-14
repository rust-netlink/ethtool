// SPDX-License-Identifier: MIT

use crate::{EthtoolHandle, EthtoolModuleEEPROMGetRequest};

pub struct EthtoolModuleEEPROMHandle(EthtoolHandle);

impl EthtoolModuleEEPROMHandle {
    pub fn new(handle: EthtoolHandle) -> Self {
        EthtoolModuleEEPROMHandle(handle)
    }

    /// Retrieve the module eeprom data pages of a interface (used by `ethtool -m
    /// eth1`)
    pub fn get(&mut self, iface_name: Option<&str>,
            offset: u32,
            length: u32,
            page: u8,
            bank: u8,
            i2c_address: u8
    ) -> EthtoolModuleEEPROMGetRequest {
        EthtoolModuleEEPROMGetRequest::new(self.0.clone(), iface_name, offset, length, page, bank, i2c_address)
    }
}
