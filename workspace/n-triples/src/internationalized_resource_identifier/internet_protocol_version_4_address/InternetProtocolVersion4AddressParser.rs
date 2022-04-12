// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct InternetProtocolVersion4AddressParser;

impl InternetProtocolVersion4AddressParser
{
	#[inline(always)]
	pub(super) fn parse_knowing_octet_1_is_0_to_9(ihost_and_port_bytes: &[u8]) -> Result<(Ipv4Addr, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		use InternetProtocolVersion4AddressOctetNumber::One;
		Self::parse_remaining_three_octets(Self::parse_octet_0_to_9::<{One}>(ihost_and_port_bytes))
	}
	
	#[inline(always)]
	pub(super) fn parse_knowing_octet_1_is_10_to_99(ihost_and_port_bytes: &[u8], octet0_byte1: u8) -> Result<(Ipv4Addr, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		use InternetProtocolVersion4AddressOctetNumber::One;
		Self::parse_remaining_three_octets(Self::parse_octet_10_to_99::<{One}>(ihost_and_port_bytes, octet0_byte1))
	}
	
	#[inline(always)]
	pub(super) fn parse_knowing_octet_1_is_100_to_255(ihost_and_port_bytes: &[u8], octet0_byte1: u8, octet0_byte2: u8) -> Result<(Ipv4Addr, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		use InternetProtocolVersion4AddressOctetNumber::One;
		Self::parse_remaining_three_octets(Self::parse_octet_100_to_255::<{One}>(ihost_and_port_bytes, octet0_byte1, octet0_byte2))
	}
	
	#[inline(always)]
	fn parse_remaining_three_octets(first_octet_parse_result: Result<(u8, &[u8]), InternetProtocolVersion4AddressParseError>) -> Result<(Ipv4Addr, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		let (octet1, remaining_bytes) = first_octet_parse_result?;
		let (octet2, remaining_bytes) = Self::parse_octet_2(remaining_bytes)?;
		let (octet3, remaining_bytes) = Self::parse_octet_3(remaining_bytes)?;
		let (octet4, remaining_bytes) = Self::parse_octet_4(remaining_bytes)?;
		
		Ok((Ipv4Addr::new(octet1, octet2, octet3, octet4), remaining_bytes))
	}
	
	#[inline(always)]
	fn parse_octet_2(remaining_bytes: &[u8]) -> Result<(u8, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		use InternetProtocolVersion4AddressOctetNumber::Two;
		
		// Minimum length is 4; indices 0-3 inclusive are always valid.
		Self::can_not_be_remaining_three_octets_of_an_internet_protocol_version_4_address(remaining_bytes)?;
		
		/*
			Possible valid sequences
			
				  Index (as now)	  2nd Octet Length
				 0    1    2    3
				0-9   .   0-9   .			1
				0-9   .   1-9  0-9			1
				1-9  0-9   .   0-9			2
				 1   0-9  0-9   .			3
				 2   0-4  0-9   .			3
				 2    5   0-5   .			3
			
			If we get the value at index 1 and it is a period, we have only 2nd Octet Length 1
			If we then get the value at index 2 and it is a period, we have only 2nd Octet Length 2 or 2nd Octet Length 3.
		 */
		let (possible_octet2_byte1, is_period) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_1(remaining_bytes);
		if is_period
		{
			Self::parse_octet_0_to_9::<{Two}>(remaining_bytes)
		}
		else
		{
			let (possible_octet2_byte2, is_period) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_2(remaining_bytes);
			if is_period
			{
				Self::parse_octet_10_to_99::<{Two}>(remaining_bytes, possible_octet2_byte1)
			}
			else
			{
				if Self::get_byte_and_check_if_it_is_not_period_index_3(remaining_bytes)
				{
					return Err(InternetProtocolVersion4AddressParseError::ThreeDigitSecondOctetNotFollowedByPeriod)
				}
				Self::parse_octet_100_to_255::<{Two}>(remaining_bytes, possible_octet2_byte1, possible_octet2_byte2)
			}
		}
	}
	
	#[inline(always)]
	fn parse_octet_3(remaining_bytes: &[u8]) -> Result<(u8, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		use InternetProtocolVersion4AddressOctetNumber::Three;
		use InternetProtocolVersion4AddressParseError::*;
		
		// Minimum length is 3; indices 0-2 inclusive are always valid; index 3 may not be valid.
		const Index3: usize = 3;
		let length = Self::can_not_be_remaining_two_octets_of_an_internet_protocol_version_4_address(remaining_bytes)?;
		
		let (possible_octet3_byte1, is_period) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_1(remaining_bytes);
		if is_period
		{
			Self::parse_octet_0_to_9::<{Three}>(remaining_bytes)
		}
		else
		{
			let (possible_octet3_byte2, is_period) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_2(remaining_bytes);
			if is_period
			{
				Self::parse_octet_10_to_99::<{Three}>(remaining_bytes, possible_octet3_byte1)
			}
			else
			{
				if length <= Index3
				{
					return Err(ThreeDigitThirdOctetEndsBeforePeriod)
				}
				if Self::get_byte_and_check_if_it_is_not_period_index_3(remaining_bytes)
				{
					return Err(ThreeDigitThirdOctetNotFollowedByPeriod)
				}
				Self::parse_octet_100_to_255::<{Three}>(remaining_bytes, possible_octet3_byte1, possible_octet3_byte2)
			}
		}
	}
	
	#[inline(always)]
	fn parse_octet_4(remaining_bytes: &[u8]) -> Result<(u8, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		use InternetProtocolVersion4AddressOctetNumber::Four;
		
		match remaining_bytes.len()
		{
			0 => Err(InternetProtocolVersion4AddressParseError::RemainingOctetTooShort),
			
			1 => Self::parse_octet_0_to_9::<{Four}>(remaining_bytes),
			
			2 => Self::parse_octet_10_to_99::<{Four}>(remaining_bytes, remaining_bytes.get_unchecked_value_safe(1)),
			
			_ => Self::parse_octet_100_to_255::<{Four}>(remaining_bytes, remaining_bytes.get_unchecked_value_safe(1), remaining_bytes.get_unchecked_value_safe(2)),
		}
	}
	
	#[inline(always)]
	fn parse_octet_0_to_9<const octet_number: InternetProtocolVersion4AddressOctetNumber>(remaining_bytes: &[u8]) -> Result<(u8, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		Self::slice_remaining_bytes::<2>(remaining_bytes, Self::parse_octet_0_to_9_without_slicing::<octet_number>(remaining_bytes))
	}
	
	#[inline(always)]
	fn parse_octet_10_to_99<const octet_number: InternetProtocolVersion4AddressOctetNumber>(remaining_bytes: &[u8], octet_byte1: u8) -> Result<(u8, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		Self::slice_remaining_bytes::<3>(remaining_bytes, Self::parse_octet_10_to_99_without_slicing::<octet_number>(remaining_bytes, octet_byte1))
	}
	
	#[inline(always)]
	fn parse_octet_100_to_255<const octet_number: InternetProtocolVersion4AddressOctetNumber>(remaining_bytes: &[u8], octet_byte1: u8, octet_byte2: u8) -> Result<(u8, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		Self::slice_remaining_bytes::<4>(remaining_bytes, Self::parse_octet_100_to_255_without_slicing::<octet_number>(remaining_bytes, octet_byte1, octet_byte2))
	}
	
	#[inline(always)]
	fn parse_octet_0_to_9_without_slicing<const octet_number: InternetProtocolVersion4AddressOctetNumber>(remaining_bytes: &[u8]) -> Result<u8, InternetProtocolVersion4AddressParseError>
	{
		debug_assert!(remaining_bytes.len() >= 1);
		
		use InternetProtocolVersion4AddressOctetLength1ParseError::FirstDigitMustBeBetween0To9Inclusive;
		
		let octet = match Self::get_0(remaining_bytes)
		{
			valid @ _0 ..= _9 => Self::digit_to_value(valid),
			
			invalid @ _ => return Self::invalid_digit::<_, octet_number>(invalid, FirstDigitMustBeBetween0To9Inclusive)
		};
		Ok(octet)
	}
	
	#[inline(always)]
	fn parse_octet_10_to_99_without_slicing<const octet_number: InternetProtocolVersion4AddressOctetNumber>(remaining_bytes: &[u8], octet_byte1: u8) -> Result<u8, InternetProtocolVersion4AddressParseError>
	{
		debug_assert!(remaining_bytes.len() >= 2);
		
		use InternetProtocolVersion4AddressOctetLength2ParseError::*;
		
		let octet = match Self::get_0(remaining_bytes)
		{
			valid_times_10 @ _1 ..= _9 => match octet_byte1
			{
				valid @ _0 ..= _9 => Self::digit_to_value_times_10(valid_times_10) + Self::digit_to_value(valid),
				
				invalid @ _ => return Self::invalid_digit::<_, octet_number>(invalid, SecondDigitMustBeBetween0To9Inclusive)
			}
			
			invalid @ _ => return Self::invalid_digit::<_, octet_number>(invalid, FirstDigitMustBeBetween1To9Inclusive)
		};
		Ok(octet)
	}
	
	#[inline(always)]
	fn parse_octet_100_to_255_without_slicing<const octet_number: InternetProtocolVersion4AddressOctetNumber>(remaining_bytes: &[u8], octet_byte1: u8, octet_byte2: u8) -> Result<u8, InternetProtocolVersion4AddressParseError>
	{
		debug_assert!(remaining_bytes.len() >= 3);
		
		use InternetProtocolVersion4AddressOctetLength3ParseError::*;
		
		let octet = match Self::get_0(remaining_bytes)
		{
			// Parse 100-199.
			_1 => match octet_byte1
			{
				valid_times_10 @ _0 ..= _9 => match octet_byte2
				{
					valid @ _0 ..= _9 => 100 + Self::digit_to_value_times_10(valid_times_10) + Self::digit_to_value(valid),
					
					invalid @ _ => return Self::invalid_digit::<_, octet_number>(invalid, ThirdDigitMustBeBetween0To9Inclusive)
				},
				
				invalid @ _ => return Self::invalid_digit::<_, octet_number>(invalid, SecondDigitMustBeBetween0To9Inclusive)
			},
			
			// Parse 200-255.
			_2 => match octet_byte1
			{
				// Parse 200-249.
				valid_times_10 @ _0 ..= _4 => match octet_byte2
				{
					valid @ _0 ..= _9 => 200 + Self::digit_to_value_times_10(valid_times_10) + Self::digit_to_value(valid),
					
					invalid @ _ => return Self::invalid_digit::<_, octet_number>(invalid, ThirdDigitMustBeBetween0To9Inclusive)
				},
				
				// Parse 250-255.
				_5 => match octet_byte2
				{
					valid @ _0 ..= _5 => 200 + 50 + Self::digit_to_value(valid),
					
					invalid @ _ => return Self::invalid_digit::<_, octet_number>(invalid, ThirdDigitMustBeBetween0To5Inclusive)
				},
				
				invalid @ _ => return Self::invalid_digit::<_, octet_number>(invalid, SecondDigitMustBeBetween0To5Inclusive)
			}
			
			invalid @ _ => return Self::invalid_digit::<_, octet_number>(invalid, FirstDigitMustBe1Or2)
		};
		Ok(octet)
	}
	
	#[inline(always)]
	fn slice_remaining_bytes<const width: u8>(remaining_bytes: &[u8], result: Result<u8, InternetProtocolVersion4AddressParseError>) -> Result<(u8, &[u8]), InternetProtocolVersion4AddressParseError>
	{
		debug_assert!(width >= 2);
		debug_assert!(width <= 4);
		
		result.map(|octet| (octet, remaining_bytes.get_unchecked_range_safe(width .. )))
	}
	
	#[inline(always)]
	pub(super) fn could_be_an_internet_protocol_version_4_address(length: usize) -> bool
	{
		Self::remainder_could_be_an_internet_protocol_version_4_address::<"0.0.0.0">(length)
	}
	
	#[inline(always)]
	fn can_not_be_remaining_three_octets_of_an_internet_protocol_version_4_address(remaining_bytes: &[u8]) -> Result<(), InternetProtocolVersion4AddressParseError>
	{
		let _ = Self::check_remainder_could_be_an_internet_protocol_version_4_address::<_, "0.0.0">(remaining_bytes, InternetProtocolVersion4AddressParseError::RemainingThreeOctetsTooShort)?;
		Ok(())
	}
	
	#[inline(always)]
	fn can_not_be_remaining_two_octets_of_an_internet_protocol_version_4_address(remaining_bytes: &[u8]) -> Result<usize, InternetProtocolVersion4AddressParseError>
	{
		Self::check_remainder_could_be_an_internet_protocol_version_4_address::<_, "0.0">(remaining_bytes, InternetProtocolVersion4AddressParseError::RemainingTwoOctetsTooShort)
	}
	
	#[inline(always)]
	fn check_remainder_could_be_an_internet_protocol_version_4_address<E: FnOnce(usize) -> InternetProtocolVersion4AddressParseError, const minimum: &'static str>(remaining_bytes: &[u8], error: E) -> Result<usize, InternetProtocolVersion4AddressParseError>
	{
		let length = remaining_bytes.len();
		if Self::remainder_could_be_an_internet_protocol_version_4_address::<minimum>(length)
		{
			Ok(length)
		}
		else
		{
			Err(error(length))
		}
	}
	
	#[inline(always)]
	const fn remainder_could_be_an_internet_protocol_version_4_address<const minimum: &'static str>(length: usize) -> bool
	{
		// Rust does not permit these to be const yet.
		let InclusiveMinimumLength = minimum.len();
		
		length >= InclusiveMinimumLength
	}
	
	#[inline(always)]
	pub(super) fn get_byte_and_check_if_it_is_period_index_1(remaining_bytes: &[u8]) -> (u8, bool)
	{
		const Index1: NonZeroU8 = new_non_zero_u8(1);
		Self::get_byte_and_check_if_it_is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name::<Index1>(remaining_bytes)
	}
	
	#[inline(always)]
	pub(super) fn get_byte_and_check_if_it_is_period_index_2(remaining_bytes: &[u8]) -> (u8, bool)
	{
		const Index2: NonZeroU8 = new_non_zero_u8(2);
		Self::get_byte_and_check_if_it_is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name::<Index2>(remaining_bytes)
	}
	
	#[inline(always)]
	fn get_byte_and_check_if_it_is_not_period_index_3(remaining_bytes: &[u8]) -> bool
	{
		!Self::get_byte_and_check_if_it_is_period_index_3(remaining_bytes)
	}
	
	#[inline(always)]
	pub(super) fn get_byte_and_check_if_it_is_period_index_3(remaining_bytes: &[u8]) -> bool
	{
		const Index3: NonZeroU8 = new_non_zero_u8(3);
		Self::get_byte_and_check_if_it_is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name::<Index3>(remaining_bytes).1
	}
	
	#[inline(always)]
	fn get_byte_and_check_if_it_is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name<const index: NonZeroU8>(remaining_bytes: &[u8]) -> (u8, bool)
	{
		debug_assert!(remaining_bytes.len() > index.as_usize());
		
		let byte = remaining_bytes.get_unchecked_value_safe(index);
		(byte, byte == Period)
	}
	
	#[inline(always)]
	fn get_0(remaining_bytes: &[u8]) -> u8
	{
		debug_assert!(remaining_bytes.len() > 0);
		
		remaining_bytes.get_unchecked_value_safe(0)
	}
	
	#[inline(always)]
	const fn digit_to_value(valid: u8) -> u8
	{
		valid - _0
	}
	
	#[inline(always)]
	const fn digit_to_value_times_10(valid_times_10: u8) -> u8
	{
		Self::digit_to_value(valid_times_10) * 10
	}
	
	#[inline(always)]
	const fn invalid_digit<E: error::Error + Into<InternetProtocolVersion4AddressOctetParseError>, const octet_number: InternetProtocolVersion4AddressOctetNumber>(invalid: u8, error: E) -> Result<u8, InternetProtocolVersion4AddressParseError>
	{
		Err(InternetProtocolVersion4AddressParseError::InternetProtocolVersion4AddressOctetParse { octet_number, cause: error.into(), invalid })
	}
}
