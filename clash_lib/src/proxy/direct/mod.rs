use crate::app::dns::ThreadSafeDNSResolver;
use crate::config::internal::proxy::PROXY_DIRECT;
use crate::proxy::datagram::OutboundDatagramImpl;
use crate::proxy::utils::{new_tcp_stream, new_udp_socket};
use crate::proxy::{AnyOutboundDatagram, AnyOutboundHandler, AnyStream, OutboundHandler};
use crate::session::{Session, SocksAddr};

use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;

use super::OutboundType;

#[derive(Serialize)]
pub struct Handler;

impl Handler {
    pub fn new() -> AnyOutboundHandler {
        Arc::new(Self)
    }
}

#[async_trait]
impl OutboundHandler for Handler {
    fn name(&self) -> &str {
        PROXY_DIRECT
    }

    fn proto(&self) -> OutboundType {
        OutboundType::Direct
    }

    async fn remote_addr(&self) -> Option<SocksAddr> {
        None
    }

    async fn support_udp(&self) -> bool {
        true
    }

    async fn connect_stream(
        &self,
        sess: &Session,
        resolver: ThreadSafeDNSResolver,
    ) -> std::io::Result<AnyStream> {
        new_tcp_stream(
            resolver,
            sess.destination.host().as_str(),
            sess.destination.port(),
            None,
            #[cfg(any(target_os = "linux", target_os = "android"))]
            None,
        )
        .await
    }

    async fn proxy_stream(
        &self,
        s: AnyStream,
        #[allow(unused_variables)] sess: &Session,
        #[allow(unused_variables)] _resolver: ThreadSafeDNSResolver,
    ) -> std::io::Result<AnyStream> {
        Ok(s)
    }

    async fn connect_datagram(
        &self,
        sess: &Session,
        resolver: ThreadSafeDNSResolver,
    ) -> std::io::Result<AnyOutboundDatagram> {
        new_udp_socket(
            None,
            sess.iface.as_ref(),
            #[cfg(any(target_os = "linux", target_os = "android"))]
            None,
        )
        .await
        .map(|x| OutboundDatagramImpl::new(x, resolver))
    }
}
