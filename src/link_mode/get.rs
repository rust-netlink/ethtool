// SPDX-License-Identifier: MIT

use futures_util::Stream;
use netlink_packet_generic::GenlMessage;

use crate::{ethtool_execute, EthtoolError, EthtoolHandle, EthtoolMessage};

pub struct EthtoolLinkModeGetRequest {
    handle: EthtoolHandle,
    iface_name: Option<String>,
}

impl EthtoolLinkModeGetRequest {
    pub(crate) fn new(handle: EthtoolHandle, iface_name: Option<&str>) -> Self {
        EthtoolLinkModeGetRequest {
            handle,
            iface_name: iface_name.map(|i| i.to_string()),
        }
    }

    pub async fn execute(
        self,
    ) -> Result<
        impl Stream<Item = Result<GenlMessage<EthtoolMessage>, EthtoolError>>,
        EthtoolError,
    > {
        let EthtoolLinkModeGetRequest {
            mut handle,
            iface_name,
        } = self;

        let ethtool_msg =
            EthtoolMessage::new_link_mode_get(iface_name.as_deref());
        ethtool_execute(&mut handle, iface_name.is_none(), ethtool_msg).await
    }
}
