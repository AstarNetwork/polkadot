// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use frame_election_provider_support::SortedListProvider;
use frame_support::traits::{Get, PalletInfo};
use remote_externalities::{Builder, Mode, OnlineConfig};
use sp_runtime::traits::Block as BlockT;
use sp_std::convert::TryInto;

/// Execute the sanity check of the bags-list.
pub(crate) async fn execute<Runtime: crate::RuntimeT, Block: BlockT>(ws_url: String) {
	let mut ext = Builder::<Block>::new()
		.mode(Mode::Online(OnlineConfig {
			transport: ws_url.to_string().into(),
			pallets: vec![<Runtime as frame_system::Config>::PalletInfo::name::<
				pallet_bags_list::Pallet<Runtime>,
			>()
			.expect("Pallet always has name; qed.")
			.to_string()],
			at: None,
			state_snapshot: None,
		}))
		.build()
		.await
		.unwrap();

	ext.execute_with(|| {
		sp_core::crypto::set_default_ss58_version(Runtime::SS58Prefix::get().try_into().unwrap());
		pallet_bags_list::Pallet::<Runtime>::sanity_check().unwrap();
		log::info!(target: crate::LOG_TARGET, "executed bags-list sanity check with no errors.");
	});
}
