use crate::Randomness;
use codec::{Decode, Encode};
use frame_support::log::{debug, error, trace};
use pallet_contracts::{
	chain_extension::{
		ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
	},
	Config,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;
use webb_primitives::verifier::*;

/// Contract extension for `FetchRandom`
pub struct VerifyProofExtension;

impl<C: Config> ChainExtension<C> for VerifyProofExtension {
	fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
	where
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		match func_id {
			1101 => {
				let mut env = env.buf_in_buf_out();
				let address = env.ext().address(); // contract
				let (public_inputs, proof_input): (Vec<u8>, Vec<u8>) =
					env.read_as_unbounded(env.in_len())?;
				let result = crate::MixerVerifierBn254::verify(&public_inputs, &proof_input);
				let result_slice = result.unwrap().encode();
				trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);
				env.write(&result_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call verify"))?;
			},

			_ => {
				error!("Called an unregistered `func_id`: {:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"))
			},
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}
