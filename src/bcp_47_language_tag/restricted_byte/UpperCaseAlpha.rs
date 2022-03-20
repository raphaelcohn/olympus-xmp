// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Value is one of `A-Z`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UpperCaseAlpha(u8);

impl RestrictedByte for UpperCaseAlpha
{
	type Error = InvalidUpperCaseAlphaError;
	
	#[inline(always)]
	fn construct(validated_byte: u8) -> Self
	{
		debug_assert!(Self::validate_byte(validated_byte));
		Self(validated_byte)
	}
	
	#[inline(always)]
	fn error<const length: usize>(index: usize, byte: u8) -> Self::Error
	{
		InvalidUpperCaseAlphaError { length, index, byte }
	}
	
	#[inline(always)]
	fn validate_byte(byte: u8) -> bool
	{
		byte >= A && byte <= Z
	}
	
	#[inline(always)]
	fn validate_and_convert_byte<E, ErrorConstructor: FnOnce(Self::Error) -> E, const length: usize>(bytes: &[u8], error: ErrorConstructor, index: usize) -> Result<u8, E>
	{
		let byte = bytes.get_unchecked_value_safe(index);
		match to_upper_case(byte)
		{
			upper_case_byte @ A ..= Z => Ok(upper_case_byte),
			
			_ => Err(error(Self::error::<length>(index, byte)))
		}
	}
}
