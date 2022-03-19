// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Value is one of:
/// * `a-z`.
/// * `0-9`.
/// Case insensitive.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Alphanumeric(u8);

impl RestrictedByte for Alphanumeric
{
	type Error = InvalidAlphanumericError;
	
	#[inline(always)]
	fn construct(validated_byte: u8) -> Self
	{
		debug_assert!(Self::validate_byte(validated_byte));
		Self(validated_byte)
	}
	
	#[inline(always)]
	fn error<const length: usize>(index: usize, byte: u8) -> Self::Error
	{
		InvalidAlphanumericError { length, index, byte }
	}
}

impl Alphanumeric
{
	#[inline(always)]
	const fn validate_byte(byte: u8) -> bool
	{
		(byte >= _0 && byte <= _9) || (byte >= a && byte <= z)
	}
	
	#[inline(always)]
	pub(super) const fn new_array_unchecked<const length: usize>(value: &[u8; length]) -> [Self; length]
	{
		if cfg!(debug_assertions)
		{
			let mut index = 0;
			while index < length
			{
				let byte = value[index];
				if !Self::validate_byte(byte)
				{
					panic!("Invalid byte")
				}
				index += 1;
			}
		}
		
		unsafe { transmute_copy(value) }
	}
	
	#[unroll_for_loops]
	#[inline(always)]
	pub(super) fn validate_alphanumeric_to_lower_case<OkConstructor: FnOnce([Self; length]) -> O, ErrorConstructor: FnOnce(InvalidAlphanumericError) -> E, O, E: error::Error, const length: usize>(bytes: &[u8], ok: OkConstructor, error: ErrorConstructor) -> Result<O, E>
	{
		debug_assert_eq!(bytes.len(), length);
		
		let mut converted = UninitialisedArray::<_, length>::default();
		for index in 0 .. length
		{
			let byte = bytes.get_unchecked_value_safe(index);
			match byte
			{
				_0 ..= _9 => converted.convert(byte, index),
				
				A ..= Z => converted.convert(to_lower_case(byte), index),
				
				a ..= z => converted.convert(byte, index),
				
				_ => return Err(error(InvalidAlphanumericError { length, index, byte }))
			}
		}
		
		Ok(ok(converted.initialise()))
	}
}
