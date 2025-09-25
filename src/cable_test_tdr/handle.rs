// SPDX-License-Identifier: MIT

use crate::{
    EthtoolCableTestTdrActionRequest, EthtoolCableTestTdrConfig, EthtoolHandle,
};

pub struct EthtoolCableTestTdrHandle(EthtoolHandle);

impl EthtoolCableTestTdrHandle {
    pub fn new(handle: EthtoolHandle) -> Self {
        EthtoolCableTestTdrHandle(handle)
    }

    pub fn start(
        &mut self,
        iface_name: &str,
    ) -> EthtoolCableTestTdrActionRequest {
        EthtoolCableTestTdrActionRequest::new(self.0.clone(), iface_name, None)
    }

    pub fn start_with_config(
        &mut self,
        iface_name: &str,
        config: EthtoolCableTestTdrConfig,
    ) -> EthtoolCableTestTdrActionRequest {
        EthtoolCableTestTdrActionRequest::new(
            self.0.clone(),
            iface_name,
            Some(config),
        )
    }
}
