// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Value is one of `a-z`.
/// Case insensitive.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Alpha(u8);

impl RestrictedByte for Alpha
{
	type Error = InvalidAlphaError;
	
	#[inline(always)]
	fn construct(validated_byte: u8) -> Self
	{
		debug_assert!(validated_byte >= a && validated_byte <= z);
		Self(validated_byte)
	}
	
	#[inline(always)]
	fn error<const length: usize>(index: usize, byte: u8) -> Self::Error
	{
		InvalidAlphaError { length, index, byte }
	}
}

impl Alpha
{
	#[unroll_for_loops]
	#[inline(always)]
	pub(super) fn validate_alpha_to_lower_case<OkConstructor: FnOnce([Self; length]) -> O, ErrorConstructor: FnOnce(InvalidAlphaError) -> E, O, E: error::Error, const length: usize>(bytes: &[u8], ok: OkConstructor, error: ErrorConstructor) -> Result<O, E>
	{
		debug_assert_eq!(bytes.len(), length);
		
		let mut converted = UninitialisedArray::<_, length>::default();
		for index in 0 .. length
		{
			let original_byte = bytes.get_unchecked_value_safe(index);
			match to_lower_case(original_byte)
			{
				byte @ a ..= z => converted.convert(byte, index),
				
				_ => return Err(error(InvalidAlphaError { length, index, byte: original_byte }))
			}
		}
		
		Ok(ok(converted.initialise()))
	}
	
	#[inline(always)]
	pub(super) fn lower_case_alpha<const index: u8>(bytes: &[u8]) -> u8
	{
		Self::lower_case_alpha_indexed(bytes, index)
	}
	
	#[inline(always)]
	pub(super) fn lower_case_alpha_indexed<AUI: AsUsizeIndex>(bytes: &[u8], index: AUI) -> u8
	{
		let index = index.as_usize();
		debug_assert!(index < bytes.len());
		
		to_lower_case(bytes.get_unchecked_value_safe(index))
	}
}
