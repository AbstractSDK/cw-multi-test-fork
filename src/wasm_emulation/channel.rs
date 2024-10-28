use anyhow::Result as AnyResult;
use async_std::task::block_on;
use cw_orch::daemon::GrpcChannel;
use tonic::transport::Channel;

/// Simple helper to get the GRPC transport channel
fn get_channel(
    grpc_urls: &[String],
    chain_id: impl Into<String>,
) -> anyhow::Result<tonic::transport::Channel> {
    let channel = block_on(GrpcChannel::connect(grpc_urls, &chain_id.into()))?;
    Ok(channel)
}

#[derive(Clone)]
pub struct RemoteChannel {
    pub channel: Channel,
    pub pub_address_prefix: String,
    // For caching
    pub chain_id: String,
}

impl RemoteChannel {
    pub fn new(
        grpc_urls: &[&str],
        chain_id: impl Into<String>,
        pub_address_prefix: impl Into<String>,
    ) -> AnyResult<Self> {
        let chain_id = chain_id.into();
        Ok(Self {
            channel: get_channel(
                &grpc_urls
                    .iter()
                    .cloned()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
                chain_id.clone(),
            )?,
            pub_address_prefix: pub_address_prefix.into(),
            chain_id,
        })
    }
}
