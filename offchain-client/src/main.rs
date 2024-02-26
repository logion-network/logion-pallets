use subxt::{OnlineClient, PolkadotConfig};

use crate::polkadot::runtime_types::pallet_lo_authority_list::LegalOfficerData;

#[subxt::subxt(runtime_metadata_path = "metadata/logion-161.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let api = OnlineClient::<PolkadotConfig>::from_url("wss://dev-rpc01.logion.network").await?;

	let version_query = polkadot::storage().lo_authority_list().pallet_storage_version();
	let version = api.storage().at_latest().await?.fetch(&version_query).await?.ok_or("Failed")?;
	println!("lo_authority_list - Storage Version: {:?}", version);

	let lo_query = polkadot::storage().lo_authority_list().legal_officer_set_iter();
	let mut results = api.storage().at_latest().await?.iter(lo_query).await?;
	while let Some(result) = results.next().await {
		let (key, value): (Vec<u8>, LegalOfficerData<_, _>) = result?;
		println!("Key: 0x{}", hex::encode(&key));
		match value {
			LegalOfficerData::Host(data) => {
				let base_url: Vec<u8> = data.base_url.unwrap();
				println!("base_url: {:?}", String::from_utf8(base_url).expect("Failed to decode base_url"));
				println!("region: {:?}", data.region);
			},
			LegalOfficerData::Guest(host) => {
				println!("host: {:?}", host);
			}
		};
	}

	Ok(())
}
