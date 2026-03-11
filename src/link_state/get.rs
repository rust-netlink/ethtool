// SPDX-License-Identifier: MIT

use futures_util::Stream;
use netlink_packet_generic::GenlMessage;

use crate::{ethtool_execute, EthtoolError, EthtoolHandle, EthtoolMessage};

pub struct EthtoolLinkStateGetRequest {
    handle: EthtoolHandle,
    iface_name: Option<String>,
    flags: Option<u32>,
}

impl EthtoolLinkStateGetRequest {
    pub(crate) fn new(
        handle: EthtoolHandle,
        iface_name: Option<&str>,
        flags: Option<u32>,
    ) -> Self {
        EthtoolLinkStateGetRequest {
            handle,
            iface_name: iface_name.map(|i| i.to_string()),
            flags,
        }
    }

    pub async fn execute(
        self,
    ) -> Result<
        impl Stream<Item = Result<GenlMessage<EthtoolMessage>, EthtoolError>>,
        EthtoolError,
    > {
        let EthtoolLinkStateGetRequest {
            mut handle,
            iface_name,
            flags,
        } = self;

        let ethtool_msg =
            EthtoolMessage::new_link_state_get(iface_name.as_deref(), flags);
        ethtool_execute(&mut handle, iface_name.is_none(), ethtool_msg).await
    }
}
