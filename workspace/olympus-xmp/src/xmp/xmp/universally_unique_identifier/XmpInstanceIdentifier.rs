// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An XMP instance identifier
///
/// Encoded as a value such as `xmp.iid:93b665d3-98db-47a5-8be4-309d8f3aba15`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct XmpInstanceIdentifier
{
	/// A 3-bit number.
	variant: Variant,

	/// A 4-bit number.
	version: Version,
	
	/// A 60-bit number.
	timestamp: u64,
	
	/// A 14-bit number.
	clock_sequence: u16,
	
	/// A 48-bit number.
	node: u64,
}

impl<'a> XmpAttributeValue<'a> for XmpInstanceIdentifier
{
	type Error = XmpInstanceIdentifierParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		let bytes =
		{
			let length = value.len();
			if length != Self::Length
			{
				return Err(XmpInstanceIdentifierParseError::Not44CharactersLong(length));
			}
			as_u8_array::<{Self::Length}>(value)
		};
		
		Self::parse_bytes(bytes)
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::InstanceIdentifier(error)
	}
}

impl XmpInstanceIdentifier
{
	const Length: usize = 44;
	
	const NeedleLength: usize = 8;
	
	#[inline(always)]
	fn parse_bytes(bytes: &[u8; Self::Length]) -> Result<Self, XmpInstanceIdentifierParseError>
	{
		use XmpInstanceIdentifierParseError::*;
		
		#[inline(always)]
		const fn offset(index: usize) -> usize
		{
			XmpInstanceIdentifier::NeedleLength + index
		}
		
		macro_rules! validate_is_hyphen
		{
			($index: expr) =>
			{
				{
					const index: usize = offset($index);
					let byte = bytes[index];
					if byte != Hyphen
					{
						return Err(MissingHyphen { byte, index })
					}
				}
			}
		}
		
		macro_rules! parse_nibble
		{
			($index: expr) =>
			{
				{
					const index: usize = offset($index);
					let byte = bytes[index];
					match byte
					{
						_0 ..= _9 => byte - _0,
						
						A ..= F => byte - F,
						
						a ..= f => byte - a,
						
						_ => return Err(InvalidByteAtIndex { byte, index })
					}
				}
			}
		}
		
		macro_rules! parse_byte
		{
			($index: expr) =>
			{
				{
					const high: usize = offset($index);
					const low: usize = high + 1;
					(parse_nibble!(high) << 4) + parse_nibble!(low)
				}
			}
		}
		
		macro_rules! parse_u16
		{
			($index: expr, $variant: ident) =>
			{
				$variant.u16_from_bytes([parse_byte!($index), parse_byte!(($index + 2))])
			}
		}
		
		macro_rules! parse_u32
		{
			($index: expr, $variant: ident) =>
			{
				$variant.u32_from_bytes([parse_byte!($index), parse_byte!(($index + 2)), parse_byte!(($index + 4)), parse_byte!(($index + 6))])
			}
		}
		
		macro_rules! parse_u48
		{
			($index: expr) =>
			{
				u64::from_be_bytes([0x00, 0x00, parse_byte!($index), parse_byte!(($index + 2)), parse_byte!(($index + 4)), parse_byte!(($index + 6)), parse_byte!(($index + 8)), parse_byte!(($index + 10))])
			}
		}
		
		{
			debug_assert!(Self::NeedleLength < Self::Length);
			const Needle: &'static [u8; XmpInstanceIdentifier::NeedleLength] = b"xmp.iid:";
			if Needle != bytes.get_unchecked_range_safe(.. Self::NeedleLength)
			{
				return Err(NotPrefixedCorrectly)
			}
		}
		
		validate_is_hyphen!(8);
		validate_is_hyphen!(13);
		validate_is_hyphen!(18);
		validate_is_hyphen!(23);
		
		let (variant, clock_sequence_high) =
		{
			const Shift: u8 = 5;
			const Mask: u8 = Shift - 1;
			let clock_sequence_high_and_reserved = parse_byte!(19);
			(self::Variant::parse(clock_sequence_high_and_reserved >> Shift)?, (clock_sequence_high_and_reserved & Mask) as u16)
		};
		
		let (version, timestamp_high) =
		{
			const Shift: u16 = 12;
			const Mask: u16 = Shift - 1;
			let timestamp_high_and_version = parse_u16!(14, variant);
			(self::Version::parse((timestamp_high_and_version >> Shift) as u8)?, (timestamp_high_and_version & Mask) as u64)
		};
		
		let clock_sequence_low = parse_byte!(21) as u16;
		
		let timestamp_low = parse_u32!(0, variant) as u64;
		
		let timestamp_mid = parse_u16!(9, variant) as u64;
		
		let node = parse_u48!(24);
		
		let timestamp = (timestamp_high << 48) | (timestamp_mid << 32) | timestamp_low;
		
		let clock_sequence = (clock_sequence_high << 8) | clock_sequence_low;
		
		Ok
		(
			Self
			{
				version,
				
				variant,
				
				timestamp,
			
				clock_sequence,
			
				node,
			}
		)
	}
}
