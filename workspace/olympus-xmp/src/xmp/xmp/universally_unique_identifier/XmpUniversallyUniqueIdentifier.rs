// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An Universally Unique Identifier (UUID).
///
/// Encoded as a value without hyphens in upper case hexadecimal, eg as '3D2D626F98BAE0CC37C5E1ABEAADBBB9'.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct XmpUniversallyUniqueIdentifier(u128);

impl<'a> XmpAttributeValue<'a> for XmpUniversallyUniqueIdentifier
{
	type Error = XmpUniversallyUniqueIdentifierParseError;
	
	#[unroll_for_loops]
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use XmpUniversallyUniqueIdentifierParseError::*;
		
		const Length: usize = 32;
		
		let bytes =
		{
			let length = value.len();
			if length != Length
			{
				return Err(Not32CharactersLong(length))
			}
			as_u8_array::<Length>(value)
		};
		
		let mut result: u128 = 0;
		for index in 0 .. Length
		{
			let byte = bytes.get_unchecked_value_safe(index);
			let correction = match byte
			{
				_0 ..= _9 => _0 as u128,
				
				A ..= F => A as u128,
				
				_ => return Err(InvalidByteAtIndex { byte, index })
			};
			result = (result << 4) + ((byte as u128) - correction);
		}
		
		Ok(Self(result))
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::UniversallyUniqueIdentifier(error)
	}
}

impl XmpUniversallyUniqueIdentifier
{
	/// Zero.
	pub const Zero: Self = Self(0);
}
