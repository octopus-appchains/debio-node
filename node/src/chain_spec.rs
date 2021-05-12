use sp_core::{Pair, Public, sr25519};
use node_template_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature, OrdersConfig,
	ImOnlineConfig, SessionConfig, opaque::SessionKeys,
	StakingConfig, Balance, DOLLARS, OctopusAppchainConfig,
};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::{ChainType, Properties};

use sp_consensus_babe::{AuthorityId as BabeId};
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};

use pallet_staking::StakerStatus;
use sp_runtime::Perbill;
use pallet_octopus_appchain::crypto::AuthorityId as OctopusId;

use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

fn session_keys(
   	babe: BabeId,
    grandpa: GrandpaId,
	im_online: ImOnlineId,
    octopus: OctopusId,
)
    -> SessionKeys 
{
    SessionKeys { babe, grandpa, im_online, octopus }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate ocw, validator, session key and weight from seed.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, BabeId, GrandpaId, ImOnlineId, OctopusId, u64) {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<OctopusId>(s),
        100,
    )
}



pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

        let mut properties = Properties::new();
        properties.insert("tokenSymbol".into(), "DBIO".into());
        properties.insert("tokenDecimals".into(), 15.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"Debio Dev Net",
		// ID
		"debio_dev_net",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Sudo account
			// 5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
			hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
			// Orders Pallet admin key
			// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
			hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
			// Pre-funded accounts
			vec![
				// Sudo         5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
				hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
				// Faucet       5HbNav6B8wUj8F9jRCVEcL6a576iHP8HJhfSfZM7fEHnRs2X
				hex!["f490e69c55aa14d06bb5d62d12b81db20f3c125d6ea5d1cfddfcf98767272e6b"].into(),
				// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
				hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

        let mut properties = Properties::new();
        properties.insert("tokenSymbol".into(), "DBIO".into());
        properties.insert("tokenDecimals".into(), 15.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"Debio Local Testnet",
		// ID
		"debio_local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Sudo account
			// 5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
			hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
			// Orders Pallet admin key
			// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
			hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
			// Pre-funded accounts
			vec![
				// Sudo     5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
				hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
				// Faucet   5HbNav6B8wUj8F9jRCVEcL6a576iHP8HJhfSfZM7fEHnRs2X
				hex!["f490e69c55aa14d06bb5d62d12b81db20f3c125d6ea5d1cfddfcf98767272e6b"].into(),
				// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
				hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
				// Local Validator 1
				hex!["60d4ab568a4e65640e34eb6c73fd25ee507b4cce11e165e32e7c821909d2bf4a"].into(),
				// Local Validator 2
				hex!["c4f832c4a8d7fab80767235d0f20f9d3ce1de635e8c5db652b6afcaa511fac7b"].into(),
				// Local Validator 3
				hex!["74727b4f2707debaaf5bbdfce661c3254fc0e9c46ce4cfbe5ec6e18482d9f922"].into(),
				// Local Validator 4
				hex!["8cdfb79e12bd0ec734dd736d12082f241f6ca21c823bec3c6a2598439eb5a97d"].into(),
				// get_account_id_from_seed::<sr25519::Public>("Alice"),
				// get_account_id_from_seed::<sr25519::Public>("Bob"),
				// get_account_id_from_seed::<sr25519::Public>("Charlie"),
				// get_account_id_from_seed::<sr25519::Public>("Dave"),
				// get_account_id_from_seed::<sr25519::Public>("Eve"),
				// get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				// get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				// get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				// get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				// get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				// get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				// get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}


/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, BabeId, GrandpaId, ImOnlineId, OctopusId, u64)>,
	root_key: AccountId,
	orders_escrow_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, ENDOWMENT)).collect(),
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
						x.1.clone(),
						x.2.clone(),
						x.3.clone(),
						x.4.clone(),
				))
			}).collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		pallet_im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		pallet_octopus_appchain: Some(OctopusAppchainConfig {
			validators: initial_authorities.iter().map(|x| (x.0.clone(), x.5)).collect(),
		}),
		orders: Some(OrdersConfig {
			escrow_key: orders_escrow_key,
		})
	}
}
