#![allow(dead_code)]

use color_eyre::eyre;
use ethers::prelude::{Http, JsonRpcClient, Middleware, TransactionRequest, H160};
use ethers::prelude::{Provider, U256};
use hex_literal::hex;
use std::sync::Arc;

pub struct QueryContract<P: JsonRpcClient + Send + Sync> {
    provider: Arc<Provider<P>>,
    address: H160,
}

impl<P: JsonRpcClient + Send + Sync> QueryContract<P> {
    const GET_PAIRS_SIGNATURE: [u8; 1] = [0x00];
    const GET_RESERVES_SIGNATURE: [u8; 1] = [0x01];

    pub fn new(provider: Arc<Provider<P>>, address: H160) -> Self {
        Self { provider, address }
    }

    pub async fn get_pairs(&self, factory: H160) -> eyre::Result<Vec<H160>> {
        let msg = TransactionRequest::new()
            .to(self.address)
            .from(H160::zero())
            .data([&Self::GET_PAIRS_SIGNATURE, &factory.0[..]].concat())
            .gas(u32::MAX)
            .chain_id(1u64)
            .value(0u64)
            .into();
        let out = self.provider.call(&msg, None).await?;
        Ok((0..out.0.len())
            .step_by(32)
            .map(|offset| H160::from_slice(&out.0[offset + 12..offset + 32]))
            .collect::<Vec<_>>())
    }

    pub async fn get_reserves(&self, factory: H160) -> eyre::Result<Vec<(H160, U256, U256)>> {
        let msg = TransactionRequest::new()
            .to(self.address)
            .data([&Self::GET_PAIRS_SIGNATURE, &factory.0[..]].concat())
            .gas(u32::MAX)
            .chain_id(1u64)
            .value(0u64)
            .into();
        let pairs_raw = self.provider.call(&msg, None).await?;

        let msg = TransactionRequest::new()
            .to(self.address)
            .data([&Self::GET_RESERVES_SIGNATURE, &pairs_raw.0[..]].concat())
            .gas(u32::MAX)
            .into();
        let out = self.provider.call(&msg, None).await?;

        Ok((0..pairs_raw.0.len())
            .step_by(32)
            .zip((0..out.0.len()).step_by(96))
            .map(|(offset, out_offset)| {
                let pair_address = H160::from_slice(&pairs_raw.0[offset + 12..offset + 32]);
                let reserve0 = U256::from(&out.0[out_offset..out_offset + 32]);
                let reserve1 = U256::from(&out.0[out_offset + 32..out_offset + 64]);

                (pair_address, reserve0, reserve1)
            })
            .collect::<Vec<_>>())
    }
}

const UNISWAP_FACTORY: H160 = H160(hex!("5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f"));

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let provider = Arc::new(Provider::<Http>::try_from("http://127.0.0.1:8545")?);
    let query = QueryContract::new(
        provider.clone(),
        H160(hex!("03AfD6B84124e21F8151242DA968F5571007a522")),
    );
    let _pairs = query.get_reserves(UNISWAP_FACTORY).await?;

    Ok(())
}
