use indexmap::IndexMap;
use papyrus_config::{Config, Configurable, ConfigurationBuilder};
use starknet_api::core::{ClassHash, CompiledClassHash, ContractAddress, Nonce, PatriciaKey};
use starknet_api::deprecated_contract_class::ContractClass as DeprecatedContractClass;
use starknet_api::hash::{StarkFelt, StarkHash};
use starknet_api::state::{ContractClass, StateDiff, StorageKey};
use starknet_api::{patricia_key, stark_felt};

use crate::{sort_state_diff, SyncConfig};

#[test]
fn test_config() {
    let config = get_built_configuration();

    let sync_config = SyncConfig::new(&config);

    assert_eq!(std::time::Duration::from_secs(10), sync_config.block_propagation_sleep_duration);
    assert_eq!(std::time::Duration::from_secs(10), sync_config.recoverable_error_sleep_duration);
    assert_eq!(1000u32, sync_config.blocks_max_stream_size);
    assert_eq!(1000u32, sync_config.state_updates_max_stream_size);
}

fn get_built_configuration() -> Config {
    ConfigurationBuilder::apply_default()
        .apply_config_file()
        .apply_env()
        .apply_command_line()
        .build()
}
// TODO(anatg): Add a test to check that the sync calls the sort_state_diff function
// before writing to the storage.
#[test]
fn state_sorted() {
    let hash0 = stark_felt!("0x0");
    let patricia_key0 = patricia_key!("0x0");
    let hash1 = stark_felt!("0x1");
    let patricia_key1 = patricia_key!("0x1");

    let dep_contract_0 = (ContractAddress(patricia_key0), ClassHash(hash0));
    let dep_contract_1 = (ContractAddress(patricia_key1), ClassHash(hash1));
    let storage_key_0 = StorageKey(patricia_key!("0x0"));
    let storage_key_1 = StorageKey(patricia_key!("0x1"));
    let declare_class_0 =
        (ClassHash(hash0), (CompiledClassHash::default(), ContractClass::default()));
    let declare_class_1 =
        (ClassHash(hash1), (CompiledClassHash::default(), ContractClass::default()));
    let deprecated_declared_0 = (ClassHash(hash0), DeprecatedContractClass::default());
    let deprecated_declared_1 = (ClassHash(hash1), DeprecatedContractClass::default());
    let nonce_0 = (ContractAddress(patricia_key0), Nonce(hash0));
    let nonce_1 = (ContractAddress(patricia_key1), Nonce(hash1));
    let replaced_class_0 = (ContractAddress(patricia_key0), ClassHash(hash0));
    let replaced_class_1 = (ContractAddress(patricia_key1), ClassHash(hash1));

    let unsorted_deployed_contracts = IndexMap::from([dep_contract_1, dep_contract_0]);
    let unsorted_declared_classes =
        IndexMap::from([declare_class_1.clone(), declare_class_0.clone()]);
    let unsorted_deprecated_declared =
        IndexMap::from([deprecated_declared_1.clone(), deprecated_declared_0.clone()]);
    let unsorted_nonces = IndexMap::from([nonce_1, nonce_0]);
    let unsorted_storage_entries = IndexMap::from([(storage_key_1, hash1), (storage_key_0, hash0)]);
    let unsorted_storage_diffs = IndexMap::from([
        (ContractAddress(patricia_key1), unsorted_storage_entries.clone()),
        (ContractAddress(patricia_key0), unsorted_storage_entries),
    ]);
    let unsorted_replaced_classes = IndexMap::from([replaced_class_1, replaced_class_0]);

    let mut state_diff = StateDiff {
        deployed_contracts: unsorted_deployed_contracts,
        storage_diffs: unsorted_storage_diffs,
        deprecated_declared_classes: unsorted_deprecated_declared,
        declared_classes: unsorted_declared_classes,
        nonces: unsorted_nonces,
        replaced_classes: unsorted_replaced_classes,
    };

    let sorted_deployed_contracts = IndexMap::from([dep_contract_0, dep_contract_1]);
    let sorted_declared_classes = IndexMap::from([declare_class_0, declare_class_1]);
    let sorted_deprecated_declared = IndexMap::from([deprecated_declared_0, deprecated_declared_1]);
    let sorted_nonces = IndexMap::from([nonce_0, nonce_1]);
    let sorted_storage_entries = IndexMap::from([(storage_key_0, hash0), (storage_key_1, hash1)]);
    let sorted_storage_diffs = IndexMap::from([
        (ContractAddress(patricia_key0), sorted_storage_entries.clone()),
        (ContractAddress(patricia_key1), sorted_storage_entries.clone()),
    ]);
    let sorted_replaced_classes = IndexMap::from([replaced_class_0, replaced_class_1]);

    sort_state_diff(&mut state_diff);
    assert_eq!(
        state_diff.deployed_contracts.get_index(0).unwrap(),
        sorted_deployed_contracts.get_index(0).unwrap(),
    );
    assert_eq!(
        state_diff.declared_classes.get_index(0).unwrap(),
        sorted_declared_classes.get_index(0).unwrap(),
    );
    assert_eq!(
        state_diff.deprecated_declared_classes.get_index(0).unwrap(),
        sorted_deprecated_declared.get_index(0).unwrap(),
    );
    assert_eq!(
        state_diff.storage_diffs.get_index(0).unwrap(),
        sorted_storage_diffs.get_index(0).unwrap(),
    );
    assert_eq!(
        state_diff.storage_diffs.get_index(0).unwrap().1.get_index(0).unwrap(),
        sorted_storage_entries.get_index(0).unwrap(),
    );
    assert_eq!(state_diff.nonces.get_index(0).unwrap(), sorted_nonces.get_index(0).unwrap());
    assert_eq!(
        state_diff.replaced_classes.get_index(0).unwrap(),
        sorted_replaced_classes.get_index(0).unwrap(),
    );
}
