// SPDX-License-Identifier: MIT

//! Helpers shared by the integration tests.

#![allow(dead_code)]

use ethtool::{EthtoolError, EthtoolHandle};

/// `EPERM`, the kernel returns it when the test process misses
/// `CAP_NET_ADMIN`.
const EPERM: i32 = 1;

/// `EOPNOTSUPP`, the kernel returns it when the interface does not support
/// the command.
const EOPNOTSUPP: i32 = 95;

/// Whether `err` makes a test skip instead of fail.
///
/// A test skips when the interface does not support the command
/// (`EOPNOTSUPP`) or when the process is not privileged enough
/// (`EPERM`), so that `cargo test` passes without `CAP_NET_ADMIN`.
pub fn is_skippable(err: &EthtoolError) -> bool {
    let EthtoolError::NetlinkError(nl_err) = err else {
        return false;
    };
    matches!(
        nl_err.code.map(|code| code.get()),
        Some(code) if code == -EOPNOTSUPP || code == -EPERM
    )
}

/// Name of the first netdevsim interface, or `None` when netdevsim is not
/// available, in which case the caller skips the test.
///
/// Instantiate a device to run the netdevsim tests:
///
/// ```bash
/// sudo modprobe netdevsim
/// echo "1 1" | sudo tee /sys/bus/netdevsim/new_device
/// ```
pub fn netdevsim_iface() -> Option<String> {
    for device in std::fs::read_dir("/sys/bus/netdevsim/devices")
        .ok()?
        .flatten()
    {
        if let Ok(mut ifaces) = std::fs::read_dir(device.path().join("net")) {
            if let Some(Ok(iface)) = ifaces.next() {
                return Some(iface.file_name().to_string_lossy().into_owned());
            }
        }
    }
    None
}

/// Open a new ethtool netlink connection and spawn its processing loop.
pub async fn connect() -> EthtoolHandle {
    let (connection, handle, _) =
        ethtool::new_connection().expect("failed to open netlink socket");
    tokio::spawn(connection);
    handle
}
