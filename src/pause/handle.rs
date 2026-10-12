// SPDX-License-Identifier: MIT

use futures_util::StreamExt;

use crate::{
    EthtoolError, EthtoolHandle, EthtoolPauseGetRequest, EthtoolPauseParsed,
    EthtoolPauseSetRequest,
};

impl EthtoolHandle {
    /// Retrieve the pause configuration of `iface_name` (equivalent to
    /// `ethtool -a eth1`).
    ///
    /// The `iface_name` is mandatory, use [EthtoolHandle::dump_pause] to
    /// query all interfaces.
    pub async fn get_pause(
        &mut self,
        iface_name: &str,
    ) -> Result<EthtoolPauseParsed, EthtoolError> {
        let mut stream =
            EthtoolPauseGetRequest::new(self.clone(), Some(iface_name))
                .execute()
                .await?;

        let mut configs = Vec::new();
        while let Some(config) = stream.next().await {
            configs.push(config?);
        }

        match configs.len() {
            1 => Ok(configs.remove(0)),
            0 => Err(EthtoolError::Bug(format!(
                "Got no pause reply from kernel for interface `{iface_name}`"
            ))),
            count => Err(EthtoolError::Bug(format!(
                "Got {count} pause replies from kernel for interface \
                 `{iface_name}`"
            ))),
        }
    }

    /// Retrieve the pause configuration of all interfaces (equivalent to
    /// `ethtool -a`).
    pub async fn dump_pause(
        &mut self,
    ) -> Result<Vec<EthtoolPauseParsed>, EthtoolError> {
        let mut stream = EthtoolPauseGetRequest::new(self.clone(), None)
            .execute()
            .await?;

        let mut configs = Vec::new();
        while let Some(config) = stream.next().await {
            configs.push(config?);
        }

        Ok(configs)
    }

    /// Apply a pause configuration (equivalent to `ethtool -A eth1 ...`).
    ///
    /// Only fields holding `Some` in `config` are sent, the kernel keeps the
    /// current value of the other pause properties. The interface is taken
    /// from [EthtoolPauseParsed::iface_name] or, when it is not set, from
    /// [EthtoolPauseParsed::iface_index].
    pub async fn apply_pause(
        &mut self,
        config: EthtoolPauseParsed,
    ) -> Result<(), EthtoolError> {
        EthtoolPauseSetRequest::new(self.clone(), config)
            .execute()
            .await
    }
}

pub struct EthtoolPauseHandle(EthtoolHandle);

impl EthtoolPauseHandle {
    pub fn new(handle: EthtoolHandle) -> Self {
        EthtoolPauseHandle(handle)
    }

    /// Retrieve the pause setting of a interface (equivalent to `ethtool -a
    /// eth1`)
    pub fn get(&mut self, iface_name: Option<&str>) -> EthtoolPauseGetRequest {
        EthtoolPauseGetRequest::new(self.0.clone(), iface_name)
    }

    /// Apply a pause configuration (equivalent to `ethtool -A eth1 ...`).
    ///
    /// Only fields holding `Some` in `config` are sent, the kernel keeps
    /// the current value of the other pause properties.
    pub fn set(
        &mut self,
        config: EthtoolPauseParsed,
    ) -> EthtoolPauseSetRequest {
        EthtoolPauseSetRequest::new(self.0.clone(), config)
    }
}
