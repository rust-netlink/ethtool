// SPDX-License-Identifier: MIT

use crate::{EthtoolHandle, EthtoolTsInfoGetRequest};

pub struct EthtoolTsInfoHandle(EthtoolHandle);

impl EthtoolTsInfoHandle {
    pub fn new(handle: EthtoolHandle) -> Self {
        EthtoolTsInfoHandle(handle)
    }

    /// Retrieve the ethtool timestamping capabilities of an interface
    pub fn get(&mut self, iface_name: Option<&str>) -> EthtoolTsInfoGetRequest {
        EthtoolTsInfoGetRequest::new(self.0.clone(), iface_name)
    }
}
