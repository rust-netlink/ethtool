// SPDX-License-Identifier: MIT

use crate::{EthtoolChannelGetRequest, EthtoolHandle};

pub struct EthtoolChannelHandle(EthtoolHandle);

impl EthtoolChannelHandle {
    pub fn new(handle: EthtoolHandle) -> Self {
        EthtoolChannelHandle(handle)
    }

    /// Retrieve the ethtool channels of a interface (equivalent to `ethtool -l
    /// eth1`)
    pub fn get(
        &mut self,
        iface_name: Option<&str>,
    ) -> EthtoolChannelGetRequest {
        EthtoolChannelGetRequest::new(self.0.clone(), iface_name)
    }
}
