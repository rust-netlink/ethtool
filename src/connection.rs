// SPDX-License-Identifier: MIT

use futures_channel::mpsc::UnboundedReceiver;
use genetlink::{GenetlinkError, RawGenlMessage};
use netlink_packet_core::NetlinkMessage;
use netlink_proto::Connection;
use netlink_sys::{AsyncSocket, SocketAddr};

use crate::EthtoolHandle;

#[cfg(feature = "tokio_socket")]
#[allow(clippy::type_complexity)]
pub fn new_connection() -> Result<
    (
        Connection<RawGenlMessage>,
        EthtoolHandle,
        UnboundedReceiver<(NetlinkMessage<RawGenlMessage>, SocketAddr)>,
    ),
    GenetlinkError,
> {
    new_connection_with_socket()
}

#[allow(clippy::type_complexity)]
pub fn new_connection_with_socket<S>() -> Result<
    (
        Connection<RawGenlMessage, S>,
        EthtoolHandle,
        UnboundedReceiver<(NetlinkMessage<RawGenlMessage>, SocketAddr)>,
    ),
    GenetlinkError,
>
where
    S: AsyncSocket,
{
    let (conn, handle, messages) = genetlink::new_connection_with_socket()?;
    Ok((conn, EthtoolHandle::new(handle), messages))
}
