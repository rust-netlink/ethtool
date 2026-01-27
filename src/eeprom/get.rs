// SPDX-License-Identifier: MIT

use futures_util::TryStream;
use netlink_packet_generic::GenlMessage;

use crate::{ethtool_execute, EthtoolError, EthtoolHandle, EthtoolMessage};

pub struct EthtoolModuleEEPROMGetRequest {
    handle: EthtoolHandle,
    iface_name: Option<String>,
    offset: u32,
    length: u32,
    page: u8,
    bank: u8,
    i2c_address: u8,
}

impl EthtoolModuleEEPROMGetRequest {
    pub(crate) fn new(
        handle: EthtoolHandle,
        iface_name: Option<&str>,
        offset: u32,
        length: u32,
        page: u8,
        bank: u8,
        i2c_address: u8,
    ) -> Self {
        EthtoolModuleEEPROMGetRequest {
            handle,
            iface_name: iface_name.map(|i| i.to_string()),
            offset,
            length,
            page,
            bank,
            i2c_address,
        }
    }

    pub async fn execute(
        self,
    ) -> impl TryStream<Ok = GenlMessage<EthtoolMessage>, Error = EthtoolError>
    {
        let EthtoolModuleEEPROMGetRequest {
            mut handle,
            iface_name,
            offset,
            length,
            page,
            bank,
            i2c_address,
        } = self;

        let ethtool_msg = EthtoolMessage::new_module_eeprom_get(
            iface_name.as_deref(),
            offset,
            length,
            page,
            bank,
            i2c_address,
        );
        ethtool_execute(&mut handle, iface_name.is_none(), ethtool_msg).await
    }
}
