// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Value is one of `a-z`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Alpha(u8);

impl RestrictedByte for Alpha
{
	type Error = InvalidAlphaError;
	
	#[inline(always)]
	fn construct(validated_byte: u8) -> Self
	{
		debug_assert!(Self::validate_byte(validated_byte));
		Self(validated_byte)
	}
	
	#[inline(always)]
	fn error<const length: usize>(index: usize, byte: u8) -> Self::Error
	{
		InvalidAlphaError { length, index, byte }
	}
	
	#[inline(always)]
	fn validate_byte(byte: u8) -> bool
	{
		validated_byte >= a && validated_byte <= z
	}
	
	#[inline(always)]
	fn validate_and_convert_byte<E, ErrorConstructor: FnOnce(Self::Error) -> E, const length: usize>(bytes: &[u8], error: ErrorConstructor, index: usize) -> Result<u8, E>
	{
		let byte = bytes.get_unchecked_value_safe(index);
		match to_lower_case(byte)
		{
			lower_case_byte @ a ..= z => Ok(lower_case_byte),
			
			_ => Err(error(Self::error::<length>(index, byte)))
		}
	}
}

impl Alpha
{
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
