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
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use XmpUniversallyUniqueIdentifierParseError::*;
		
		const Length: usize = 32;
		let length = value.len();
		if length != Length
		{
			return Err(Not32CharactersLong(length))
		}
		
		const _0: u128 = b'0' as u128;
		const _A: u128 = b'A' as u128;
		
		let mut result: u128 = 0;
		let bytes = value.as_bytes();
		for index in 0 .. Length
		{
			let byte = bytes.get_unchecked_value_safe(index);
			let correction = match byte
			{
				0 ..= 9 => _0,
				
				b'A' ..= b'F' => _A,
				
				_ => return Err(InvalidByteAtIndex { byte, index })
			};
			result = (result << 4) + ((byte as u128) - correction);
		}
		
		Ok(XmpUniversallyUniqueIdentifier(result))
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
