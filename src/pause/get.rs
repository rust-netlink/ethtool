// SPDX-License-Identifier: MIT

use futures_util::{Stream, StreamExt};

use crate::{
    ethtool_execute, EthtoolError, EthtoolHandle, EthtoolMessage,
    EthtoolPauseParsed,
};

pub struct EthtoolPauseGetRequest {
    handle: EthtoolHandle,
    iface_name: Option<String>,
}

impl EthtoolPauseGetRequest {
    pub(crate) fn new(handle: EthtoolHandle, iface_name: Option<&str>) -> Self {
        EthtoolPauseGetRequest {
            handle,
            iface_name: iface_name.map(|i| i.to_string()),
        }
    }

    /// Retrieve the pause configuration of one interface, or of all
    /// interfaces when no interface name was given.
    pub async fn execute(
        self,
    ) -> Result<
        impl Stream<Item = Result<EthtoolPauseParsed, EthtoolError>>,
        EthtoolError,
    > {
        let EthtoolPauseGetRequest {
            mut handle,
            iface_name,
        } = self;

        let ethtool_msg = EthtoolMessage::new_pause_get(iface_name.as_deref());
        let response =
            ethtool_execute(&mut handle, iface_name.is_none(), ethtool_msg)
                .await?;

        Ok(response.map(|msg| {
            msg.map(|genl_msg| EthtoolPauseParsed::from(genl_msg.payload))
        }))
    }
}
