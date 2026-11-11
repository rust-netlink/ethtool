// SPDX-License-Identifier: MIT

use futures_util::StreamExt;

use crate::{
    EthtoolError, EthtoolHandle, EthtoolRingGetRequest, EthtoolRingParsed,
    EthtoolRingSetRequest,
};

impl EthtoolHandle {
    /// Retrieve the ring parameters of `iface_name` (equivalent to
    /// `ethtool -g eth1`).
    ///
    /// The `iface_name` is mandatory, use [EthtoolHandle::dump_ring] to
    /// query all interfaces.
    pub async fn get_ring(
        &mut self,
        iface_name: &str,
    ) -> Result<EthtoolRingParsed, EthtoolError> {
        let mut configs = self.ring_configs(Some(iface_name)).await?;

        match configs.len() {
            1 => Ok(configs.remove(0)),
            0 => Err(EthtoolError::Bug(format!(
                "Got no ring reply from kernel for interface `{iface_name}`"
            ))),
            count => Err(EthtoolError::Bug(format!(
                "Got {count} ring replies from kernel for interface \
                 `{iface_name}`"
            ))),
        }
    }

    /// Retrieve the ring parameters of all interfaces (equivalent to
    /// `ethtool -g`).
    pub async fn dump_ring(
        &mut self,
    ) -> Result<Vec<EthtoolRingParsed>, EthtoolError> {
        self.ring_configs(None).await
    }

    /// Apply ring parameters (equivalent to `ethtool -G eth1 ...`).
    ///
    /// Only fields holding `Some` in `config` are sent, the kernel keeps the
    /// current value of the other ring parameters. The interface is taken
    /// from [EthtoolRingParsed::iface_name] or, when it is not set, from
    /// [EthtoolRingParsed::iface_index].
    pub async fn apply_ring(
        &mut self,
        config: EthtoolRingParsed,
    ) -> Result<(), EthtoolError> {
        EthtoolRingSetRequest::new(self.clone(), config)
            .execute()
            .await
    }

    async fn ring_configs(
        &mut self,
        iface_name: Option<&str>,
    ) -> Result<Vec<EthtoolRingParsed>, EthtoolError> {
        let mut stream = EthtoolRingGetRequest::new(self.clone(), iface_name)
            .execute()
            .await?;

        let mut configs = Vec::new();
        while let Some(reply) = stream.next().await {
            configs.push(EthtoolRingParsed::from(reply?.payload));
        }
        Ok(configs)
    }
}

pub struct EthtoolRingHandle(EthtoolHandle);

impl EthtoolRingHandle {
    pub fn new(handle: EthtoolHandle) -> Self {
        EthtoolRingHandle(handle)
    }

    /// Retrieve the ethtool rings of a interface (equivalent to `ethtool -g
    /// eth1`)
    pub fn get(&mut self, iface_name: Option<&str>) -> EthtoolRingGetRequest {
        EthtoolRingGetRequest::new(self.0.clone(), iface_name)
    }

    /// Apply ring parameters (equivalent to `ethtool -G eth1 ...`).
    ///
    /// Only fields holding `Some` in `config` are sent, the kernel keeps
    /// the current value of the other ring parameters.
    pub fn set(&mut self, config: EthtoolRingParsed) -> EthtoolRingSetRequest {
        EthtoolRingSetRequest::new(self.0.clone(), config)
    }
}
