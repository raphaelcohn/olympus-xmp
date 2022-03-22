// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(in crate::bcp_47_language_tag) trait RestrictedByte: RestrictedByteConst
{
	#[unroll_for_loops]
	#[inline(always)]
	fn validate_and_convert_array<OkConstructor: FnOnce([Self; length]) -> O, ErrorConstructor: Copy + FnOnce(Self::Error) -> E, O, E: error::Error, const length: usize>(bytes: &[u8], ok: OkConstructor, error: ErrorConstructor) -> Result<O, E>
	{
		debug_assert_eq!(bytes.len(), length);
		
		let mut converted = UninitialisedArray::<_, length>::default();
		for index in 0 .. length
		{
			let byte = Self::validate_and_convert_byte::<E, ErrorConstructor, length>(bytes, error, index)?;
			converted.convert(byte, index)
		}
		
		Ok(ok(converted.initialise()))
	}
	
	fn validate_and_convert_byte<E, ErrorConstructor: FnOnce(Self::Error) -> E, const length: usize>(bytes: &[u8], error: ErrorConstructor, index: usize) -> Result<u8, E>;
}
