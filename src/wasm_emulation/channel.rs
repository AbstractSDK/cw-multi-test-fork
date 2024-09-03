use anyhow::Result as AnyResult;
use cw_orch::{daemon::GrpcChannel, environment::ChainInfoOwned};
use tokio::runtime::{Handle, Runtime};
use tonic::transport::Channel;

/// Simple helper to get the GRPC transport channel
fn get_channel(chain: &ChainInfoOwned, rt: &Runtime) -> anyhow::Result<tonic::transport::Channel> {
    let channel = rt.block_on(GrpcChannel::connect(&chain.grpc_urls, &chain.chain_id))?;
    Ok(channel)
}

#[derive(Clone)]
pub struct RemoteChannel {
    pub rt: Handle,
    pub channel: Channel,
    pub pub_address_prefix: String,
    // For caching
    pub chain_id: String,
}

impl RemoteChannel {
    pub fn new(
        rt: &Runtime,
        chain: impl Into<ChainInfoOwned>,
        _: impl Into<String>,
    ) -> AnyResult<Self> {
        let chain: ChainInfoOwned = chain.into();
        Ok(Self {
            rt: rt.handle().clone(),
            channel: get_channel(&chain, rt)?,
            pub_address_prefix: chain.network_info.pub_address_prefix,
            chain_id: chain.chain_id,
        })
    }
}
