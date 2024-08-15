
//! Autogenerated weights for pallet_dao_votes
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-13, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `chp`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/genesis-dao-solochain
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_dao_votes
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// pallets/dao-votes/src/weights.rs
// --template
// ./benchmarking/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_dao_votes.
pub trait WeightInfo {
	fn create_proposal() -> Weight;
	fn set_metadata() -> Weight;
	fn fault_proposal() -> Weight;
	fn finalize_proposal() -> Weight;
	fn vote() -> Weight;
	fn set_governance_majority_vote() -> Weight;
	fn mark_implemented() -> Weight;
}

/// Weights for pallet_dao_votes using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	/// Storage: `Votes::Governances` (r:1 w:0)
	/// Proof: `Votes::Governances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `Votes::CurrentProposalId` (r:1 w:1)
	/// Proof: `Votes::CurrentProposalId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::ProposalSlots` (r:0 w:1)
	/// Proof: `Votes::ProposalSlots` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `549`
		//  Estimated: `4014`
		// Minimum execution time: 43_961_000 picoseconds.
		Weight::from_parts(45_236_000, 4014)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Votes::ProposalSlots` (r:1 w:1)
	/// Proof: `Votes::ProposalSlots` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::Proposals` (r:0 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `120`
		//  Estimated: `3585`
		// Minimum execution time: 14_465_000 picoseconds.
		Weight::from_parts(15_090_000, 3585)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Votes::Proposals` (r:1 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	fn fault_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `380`
		//  Estimated: `3893`
		// Minimum execution time: 28_099_000 picoseconds.
		Weight::from_parts(29_175_000, 3893)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Votes::Proposals` (r:1 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::Governances` (r:1 w:0)
	/// Proof: `Votes::Governances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	fn finalize_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431`
		//  Estimated: `3896`
		// Minimum execution time: 32_091_000 picoseconds.
		Weight::from_parts(33_092_000, 3896)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Votes::Proposals` (r:1 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::Governances` (r:1 w:0)
	/// Proof: `Votes::Governances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::Votes` (r:1 w:1)
	/// Proof: `Votes::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	/// Storage: `Assets::AccountHistory` (r:1 w:0)
	/// Proof: `Assets::AccountHistory` (`max_values`: None, `max_size`: Some(14400072), added: 14402547, mode: `MaxEncodedLen`)
	/// Storage: `Hookpoints::SpecificCallbacks` (r:1 w:0)
	/// Proof: `Hookpoints::SpecificCallbacks` (`max_values`: None, `max_size`: Some(162), added: 2637, mode: `MaxEncodedLen`)
	/// Storage: `Hookpoints::GlobalCallbacks` (r:1 w:0)
	/// Proof: `Hookpoints::GlobalCallbacks` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `657`
		//  Estimated: `14403537`
		// Minimum execution time: 34_232_000 picoseconds.
		Weight::from_parts(35_689_000, 14403537)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	/// Storage: `Votes::Governances` (r:0 w:1)
	/// Proof: `Votes::Governances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_governance_majority_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `133`
		//  Estimated: `3893`
		// Minimum execution time: 12_950_000 picoseconds.
		Weight::from_parts(13_844_000, 3893)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Votes::Proposals` (r:1 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	fn mark_implemented() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `3893`
		// Minimum execution time: 15_343_000 picoseconds.
		Weight::from_parts(16_181_000, 3893)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	/// Storage: `Votes::Governances` (r:1 w:0)
	/// Proof: `Votes::Governances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `Votes::CurrentProposalId` (r:1 w:1)
	/// Proof: `Votes::CurrentProposalId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::ProposalSlots` (r:0 w:1)
	/// Proof: `Votes::ProposalSlots` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `549`
		//  Estimated: `4014`
		// Minimum execution time: 43_961_000 picoseconds.
		Weight::from_parts(45_236_000, 4014)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Votes::ProposalSlots` (r:1 w:1)
	/// Proof: `Votes::ProposalSlots` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::Proposals` (r:0 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `120`
		//  Estimated: `3585`
		// Minimum execution time: 14_465_000 picoseconds.
		Weight::from_parts(15_090_000, 3585)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Votes::Proposals` (r:1 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	fn fault_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `380`
		//  Estimated: `3893`
		// Minimum execution time: 28_099_000 picoseconds.
		Weight::from_parts(29_175_000, 3893)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Votes::Proposals` (r:1 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::Governances` (r:1 w:0)
	/// Proof: `Votes::Governances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	fn finalize_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431`
		//  Estimated: `3896`
		// Minimum execution time: 32_091_000 picoseconds.
		Weight::from_parts(33_092_000, 3896)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Votes::Proposals` (r:1 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::Governances` (r:1 w:0)
	/// Proof: `Votes::Governances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Votes::Votes` (r:1 w:1)
	/// Proof: `Votes::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	/// Storage: `Assets::AccountHistory` (r:1 w:0)
	/// Proof: `Assets::AccountHistory` (`max_values`: None, `max_size`: Some(14400072), added: 14402547, mode: `MaxEncodedLen`)
	/// Storage: `Hookpoints::SpecificCallbacks` (r:1 w:0)
	/// Proof: `Hookpoints::SpecificCallbacks` (`max_values`: None, `max_size`: Some(162), added: 2637, mode: `MaxEncodedLen`)
	/// Storage: `Hookpoints::GlobalCallbacks` (r:1 w:0)
	/// Proof: `Hookpoints::GlobalCallbacks` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `657`
		//  Estimated: `14403537`
		// Minimum execution time: 34_232_000 picoseconds.
		Weight::from_parts(35_689_000, 14403537)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	/// Storage: `Votes::Governances` (r:0 w:1)
	/// Proof: `Votes::Governances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_governance_majority_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `133`
		//  Estimated: `3893`
		// Minimum execution time: 12_950_000 picoseconds.
		Weight::from_parts(13_844_000, 3893)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Votes::Proposals` (r:1 w:1)
	/// Proof: `Votes::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DaoCore::Daos` (r:1 w:0)
	/// Proof: `DaoCore::Daos` (`max_values`: None, `max_size`: Some(428), added: 2903, mode: `MaxEncodedLen`)
	fn mark_implemented() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `3893`
		// Minimum execution time: 15_343_000 picoseconds.
		Weight::from_parts(16_181_000, 3893)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}