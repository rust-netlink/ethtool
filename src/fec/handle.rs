// SPDX-License-Identifier: MIT

use crate::{EthtoolFecGetRequest, EthtoolHandle};

pub struct EthtoolFecHandle(EthtoolHandle);

impl EthtoolFecHandle {
    pub fn new(handle: EthtoolHandle) -> Self {
        EthtoolFecHandle(handle)
    }

    /// Retrieve the ethtool timestamping capabilities of an interface
    pub fn get(&mut self, iface_name: Option<&str>) -> EthtoolFecGetRequest {
        EthtoolFecGetRequest::new(self.0.clone(), iface_name)
    }
}
