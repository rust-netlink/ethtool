// SPDX-License-Identifier: MIT

use crate::{
    EthtoolChannelGetRequest, EthtoolChannelSetRequest, EthtoolHandle,
};

pub struct EthtoolChannelHandle(EthtoolHandle);

impl EthtoolChannelHandle {
    pub fn new(handle: EthtoolHandle) -> Self {
        EthtoolChannelHandle(handle)
    }

    /// Retrieve the ethtool Channels of a interface (equivalent to `ethtool -l
    /// eth1`)
    pub fn get(
        &mut self,
        iface_name: Option<&str>,
    ) -> EthtoolChannelGetRequest {
        EthtoolChannelGetRequest::new(self.0.clone(), iface_name)
    }

    /// Set the ethtool Channels of a interface (equivalent to `ethtool -L
    /// eth1`)
    pub fn set(&mut self, iface_name: &str) -> EthtoolChannelSetRequest {
        EthtoolChannelSetRequest::new(self.0.clone(), iface_name)
    }
}
