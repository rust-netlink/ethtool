// SPDX-License-Identifier: MIT

mod bitset;
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
// test data are using hard coded little endian byte order, not for big-endian
#[cfg(not(target_endian = "big"))]
#[cfg(test)]
mod tests;
mod tsinfo;

pub use self::fec::{
    EthtoolFecAttr, EthtoolFecGetRequest, EthtoolFecHandle, EthtoolFecMode,
    EthtoolFecStat,
};
pub use bitset::{EthtoolBit, EthtoolBitset, EthtoolBitsetValue};
pub use channel::{
    EthtoolChannelAttr, EthtoolChannelGetRequest, EthtoolChannelHandle,
    EthtoolChannelSetRequest,
};
pub use coalesce::{
    EthtoolCoalesceAttr, EthtoolCoalesceGetRequest, EthtoolCoalesceHandle,
    EthtoolCoalesceIrqModerationAttr, EthtoolCoalesceProfileAttr,
};
#[cfg(feature = "tokio_socket")]
pub use connection::new_connection;
pub use connection::new_connection_with_socket;
pub use eeprom::{
    EthtoolModuleEEPROMAttr, EthtoolModuleEEPROMGetRequest,
    EthtoolModuleEEPROMHandle,
};
pub use error::EthtoolError;
pub use feature::{
    EthtoolFeatureAttr, EthtoolFeatureGetRequest, EthtoolFeatureHandle,
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
    EthtoolPauseParsed, EthtoolPauseSetRequest, EthtoolPauseStatAttr,
};
pub use ring::{EthtoolRingAttr, EthtoolRingGetRequest, EthtoolRingHandle};
pub use tsinfo::{
    EthtoolTsInfoAttr, EthtoolTsInfoGetRequest, EthtoolTsInfoHandle,
};

pub(crate) use handle::ethtool_execute;

// Compile the examples of the README as doc tests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
