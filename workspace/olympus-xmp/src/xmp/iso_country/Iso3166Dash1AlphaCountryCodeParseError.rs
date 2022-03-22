// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Parse error for either an ISO 3166-1 Alpha-2 country code or an ISO 3166-1 Alpha-3 country code.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Iso3166Dash1AlphaCountryCodeParseError
{
	#[allow(missing_docs)]
	LengthIsNot2Or3(String),
	
	#[allow(missing_docs)]
	InvalidAsciiLetter
	{
		letter: u8,
		
		index: u8
	},
	
	#[allow(missing_docs)]
	UnknownIso3166Alpha2CountryCode([u8; 2]),
	
	#[allow(missing_docs)]
	UnknownIso3166Alpha3CountryCode([u8; 3]),
}

impl Display for Iso3166Dash1AlphaCountryCodeParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for Iso3166Dash1AlphaCountryCodeParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use Iso3166Dash1AlphaCountryCodeParseError::*;
		match self
		{
			LengthIsNot2Or3(_) => None,
			
			InvalidAsciiLetter { .. } => None,
			
			UnknownIso3166Alpha2CountryCode(_) => None,
			
			UnknownIso3166Alpha3CountryCode(_) => None,
		}
	}
}
