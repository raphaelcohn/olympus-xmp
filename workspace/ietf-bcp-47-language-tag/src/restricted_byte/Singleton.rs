// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Value is one of:
/// * `a-w` and `y-z` (missing `x`).
/// * `0-9`.
/// Case insensitive.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Singleton(u8);

impl const RestrictedByteConst for Singleton
{
	type Error = InvalidSingletonError;
	
	#[inline(always)]
	fn construct(validated_byte: u8) -> Self
	{
		debug_assert!(Self::validate_byte(validated_byte));
		Self(validated_byte)
	}
	
	#[inline(always)]
	fn error<const length: usize>(index: usize, byte: u8) -> Self::Error
	{
		InvalidSingletonError { length, index, byte }
	}
	
	#[inline(always)]
	fn validate_byte(byte: u8) -> bool
	{
		(byte >= _0 && byte <= _9) || (byte >= a && byte <= w) || (byte >= y && byte <= z)
	}
	
	#[inline(always)]
	fn new_array_unchecked<const length: usize>(value: [u8; length]) -> [Self; length]
	{
		new_array_unchecked::<Self, length>(value)
	}
	
	#[inline(always)]
	fn new_array_unchecked_ref<const length: usize>(value: &[u8; length]) -> [Self; length]
	{
		new_array_unchecked_ref::<Self, length>(value)
	}
}

impl RestrictedByte for Singleton
{
	#[inline(always)]
	fn validate_and_convert_byte<E, ErrorConstructor: FnOnce(Self::Error) -> E, const length: usize>(bytes: &[u8], error: ErrorConstructor, index: usize) -> Result<u8, E>
	{
		let byte = bytes.get_unchecked_value_safe(index);
		match byte
		{
			_0 ..= _9 => Ok(byte),
			
			A ..= W | Y ..= Z => Ok(to_lower_case(byte)),
			
			a ..= w | y ..= z => Ok(byte),
			
			_ => Err(error(Self::error::<length>(index, byte)))
		}
	}
}
