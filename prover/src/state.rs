use std::fs;

use aptos_crypto::ed25519::Ed25519PrivateKey;
use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use rust_rapidsnark::FullProver;
use serde::{Deserialize, Serialize};

use crate::groth16_vk::{OnChainGroth16VerificationKey, SnarkJsGroth16VerificationKey};
use crate::prover_key::TrainingWheelsKeyPair;
use crate::{
    config::{self, ProverServiceConfig},
    input_processing::config::CircuitConfig,
};
use std::env;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProverServiceSecrets {
    /// The current training wheel key.
    pub private_key_0: Ed25519PrivateKey,
    /// The training wheel key to use after the next key rotation.
    pub private_key_1: Option<Ed25519PrivateKey>,
}

pub struct ProverServiceState {
    pub full_prover_default: Mutex<FullProver>,
    pub full_prover_new: Option<Mutex<FullProver>>,
    pub new_groth16_vk: Option<OnChainGroth16VerificationKey>,
    pub tw_keypair_default: TrainingWheelsKeyPair,
    pub tw_keypair_new: Option<TrainingWheelsKeyPair>,
    pub config: ProverServiceConfig,
    pub circuit_config: CircuitConfig,
    // Ensures that only one circuit is being proven at a time
}

impl ProverServiceState {
    pub fn init() -> Self {
        let config_file_path = env::var(config::CONFIG_FILE_PATH_ENVVAR)
            .unwrap_or(String::from(config::CONFIG_FILE_PATH));

        // read config and secret key
        let config: ProverServiceConfig = Figment::new()
            .merge(Yaml::file(config_file_path))
            .merge(Env::raw())
            .extract()
            .expect("Couldn't load config");

        let ProverServiceSecrets {
            private_key_0: private_key,
            private_key_1: private_key_new,
        } = Figment::new()
            .merge(Env::raw())
            .extract()
            .expect("Couldn't load private key from environment variable PRIVATE_KEY");

        let tw_keypair_default = TrainingWheelsKeyPair::from_sk(private_key);
        let tw_keypair_new = private_key_new.map(TrainingWheelsKeyPair::from_sk);

        let circuit_config: CircuitConfig = serde_yaml::from_str(
            &fs::read_to_string("conversion_config.yml")
                .expect("Unable to read circuit config file"),
        )
        .expect("Couldn't parse circuit config file");

        println!("using resources dir {}", config.resources_dir);

        // init state
        let full_prover_default = FullProver::new(&config.zkey_path(false))
            .expect("failed to initialize rapidsnark prover with old zkey");

        let (full_prover_new, new_vk) = if config.new_setup_dir.is_some() {
            let full_prover = FullProver::new(&config.zkey_path(true))
                .expect("failed to initialize rapidsnark prover with new zkey");
            let vk_json = fs::read_to_string(config.verification_key_path(true).as_str()).unwrap();
            let local_vk: SnarkJsGroth16VerificationKey =
                serde_json::from_str(vk_json.as_str()).unwrap();
            let onchain_vk = local_vk.try_as_onchain_repr().unwrap();
            (Some(full_prover), Some(onchain_vk))
        } else {
            (None, None)
        };

        ProverServiceState {
            full_prover_default: Mutex::new(full_prover_default),
            full_prover_new: full_prover_new.map(Mutex::new),
            new_groth16_vk: new_vk,
            tw_keypair_default,
            tw_keypair_new,
            config,
            circuit_config,
        }
    }
}
