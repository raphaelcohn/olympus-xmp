// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Either an ISO 3166-1 Alpha-2 country code or an ISO 3166-1 Alpha-3 country code.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Iso3166Dash1AlphaCountryCode
{
	/// An ISO 3166-1 Alpha-2 country code.
	Alpha2(Iso3166Dash1Alpha2CountryCode),
	
	/// An ISO 3166-1 Alpha-3 country code.
	Alpha3(Iso3166Dash1Alpha3CountryCode),
}

impl Iso3166Dash1AlphaCountryCode
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn letter_to_number<const index: u8>(bytes: &[u8]) -> Result<u16, Iso3166Dash1AlphaCountryCodeParseError>
	{
		let letter = bytes.get_unchecked_value_safe(index);
		match letter
		{
			A ..= Z => Ok(Self::letter_to_number_scaled::<index>(letter)),
			
			_ => return Err(Iso3166Dash1AlphaCountryCodeParseError::InvalidAsciiLetter { letter, index })
		}
	}
	
	#[inline(always)]
	const fn letter_to_number_unchecked<const index: u8, const length: usize>(bytes: &[u8; length]) -> u16
	{
		if cfg!(debug_assertions)
		{
			if index as usize >= length
			{
				panic!("index must be less than length")
			}
		}
		
		if cfg!(debug_assertions)
		{
			if length != 2 && length != 3
			{
				panic!("length must be 2 or 3")
			}
		}
		
		let letter = bytes[index as usize];
		if cfg!(debug_assertions)
		{
			if letter < A || letter > Z
			{
				panic!("letter must be A to Z inclusive")
			}
		}
		
		Self::letter_to_number_scaled::<index>(letter)
	}
	
	#[inline(always)]
	const fn letter_to_number_scaled<const index: u8>(letter: u8) -> u16
	{
		const Scalar: u8 = Z - A + 1;
		((letter - A) as u16) * (Scalar as u16).pow(index as u32)
	}
}
