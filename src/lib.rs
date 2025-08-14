// SPDX-License-Identifier: MIT

mod bitset_util;
mod channel;
mod coalesce;
mod connection;
mod eeprom;
mod error;
mod feature;
mod fec;
mod handle;
mod header;
mod link_mode;
mod macros;
mod message;
mod pause;
mod ring;
mod tsinfo;

pub use self::fec::{
    EthtoolFecAttr, EthtoolFecGetRequest, EthtoolFecHandle, EthtoolFecMode,
    EthtoolFecStat,
};
pub use channel::{
    EthtoolChannelAttr, EthtoolChannelGetRequest, EthtoolChannelHandle,
    EthtoolChannelSetRequest,
};
pub use coalesce::{
    EthtoolCoalesceAttr, EthtoolCoalesceGetRequest, EthtoolCoalesceHandle,
};
#[cfg(feature = "tokio_socket")]
pub use connection::new_connection;
pub use connection::new_connection_with_socket;
pub use eeprom::{EthtoolModuleEEPROMAttr, EthtoolModuleEEPROMGetRequest, EthtoolModuleEEPROMHandle};
pub use error::EthtoolError;
pub use feature::{
    EthtoolFeatureAttr, EthtoolFeatureBit, EthtoolFeatureGetRequest,
    EthtoolFeatureHandle,
};
pub use handle::EthtoolHandle;
pub use header::EthtoolHeader;
pub use link_mode::{
    EthtoolLinkModeAttr, EthtoolLinkModeDuplex, EthtoolLinkModeGetRequest,
    EthtoolLinkModeHandle,
};
pub use message::{EthtoolAttr, EthtoolCmd, EthtoolMessage};
pub use pause::{
    EthtoolPauseAttr, EthtoolPauseGetRequest, EthtoolPauseHandle,
    EthtoolPauseStatAttr,
};
pub use ring::{EthtoolRingAttr, EthtoolRingGetRequest, EthtoolRingHandle};
pub use tsinfo::{
    EthtoolTsInfoAttr, EthtoolTsInfoGetRequest, EthtoolTsInfoHandle,
};

pub(crate) use handle::ethtool_execute;
