// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::convert::TryFrom;
use std::str::FromStr;
use crate::xmp::UnknownStringVariantParseError;

/// A country coding; definitions from <https://www.iso.org/obp/ui/#search>.
///
/// See <https://www.davros.org/misc/iso3166.html> for some legacy definitions.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Iso3166Dash1Country
{
	alpha_2_code: Iso3166Dash1Alpha2CountryCode,
	
	alpha_3_code: Iso3166Dash1Alpha3CountryCode,
	
	numeric_code: Iso3166Dash1NumericCountryCode,
	
	short_name: &'static str,
	
	short_name_lower_case: &'static str,
	
	full_name: Option<&'static str>,
}

impl const TryFrom<Iso3166Dash1AlphaCountryCode> for Iso3166Dash1Country
{
	type Error = UnknownIso3166Dash1CodeError;
	
	#[inline(always)]
	fn try_from(country_code: Iso3166Dash1AlphaCountryCode) -> Result<Self, Self::Error>
	{
		match country_code
		{
			Alpha2(country_code) => Self::try_from(country_code),
			
			Alpha3(country_code) => Self::try_from(country_code),
		}
	}
}

impl const TryFrom<Iso3166Dash1Alpha2CountryCode> for Iso3166Dash1Country
{
	type Error = UnknownIso3166Dash1CodeError;
	
	#[inline(always)]
	fn try_from(country_code: Iso3166Dash1Alpha2CountryCode) -> Result<Self, Self::Error>
	{
		let country = match country_code
		{
			AF => Self::AFGHANISTAN,
			
			ES => Self::SPAIN,
			
			FR => Self::FRANCE,
			
			GB => Self::UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND,
			
			GG => Self::GUERNSEY,
			
			IM => Self::ISLE_OF_MAN,
			
			IO => Self::BRITISH_INDIAN_OCEAN_TERRITORY,
			
			JE => Self::JERSEY,
			
			NF => Self::NORFOLK_ISLAND,
			
			// Exceptional reservations.
			
			AC => Self::ASCENSION_ISLAND,
			
			CP => Self::CLIPPERTON_ISLAND,
			
			CQ => Self::ISLAND_OF_SARK,
			
			DG => Self::DIEGO_GARCIA,
			
			EA => Self::CEUTA_MELILLA,
			
			IC => Self::CANARY_ISLANDS,
			
			TA => Self::TRISTAN_DA_CUNHA,
			
			FX => Self::FRANCE_METROPOLITAN,
			
			UK => Self::UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND,
			
			_ => return Err(UnknownIso3166Dash1CodeError),
		};
		Ok(country)
	}
}

impl const TryFrom<Iso3166Dash1Alpha3CountryCode> for Iso3166Dash1Country
{
	type Error = UnknownIso3166Dash1CodeError;
	
	#[inline(always)]
	fn try_from(country_code: Iso3166Dash1Alpha3CountryCode) -> Result<Self, Self::Error>
	{
		let country = match country_code
		{
			AFG => Self::AFGHANISTAN,
			
			ESP => Self::SPAIN,
			
			FRA => Self::FRANCE,
			
			GBR => Self::UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND,
			
			GGY => Self::GUERNSEY,
			
			IMN => Self::ISLE_OF_MAN,
			
			IOT => Self::BRITISH_INDIAN_OCEAN_TERRITORY,
			
			JEY => Self::JERSEY,
			
			NFK => Self::NORFOLK_ISLAND,
			
			// Exceptional reservations.
			
			ASC => Self::ASCENSION_ISLAND,
			
			CPT => Self::CLIPPERTON_ISLAND,
			
			DGA => Self::DIEGO_GARCIA,
			
			TAA => Self::TRISTAN_DA_CUNHA,
			
			FXX => Self::FRANCE_METROPOLITAN,
			
			_ => return Err(UnknownIso3166Dash1CodeError),
		};
		Ok(country)
	}
}

impl const TryFrom<Iso3166Dash1NumericCountryCode> for Iso3166Dash1Country
{
	type Error = UnknownIso3166Dash1CodeError;
	
	#[inline(always)]
	fn try_from(country_code: Iso3166Dash1NumericCountryCode) -> Result<Self, Self::Error>
	{
		let country = match country_code
		{
			_004 => Self::AFGHANISTAN,
			
			_086 => Self::BRITISH_INDIAN_OCEAN_TERRITORY,
			
			_249 => Self::FRANCE_METROPOLITAN,
			
			_250 => Self::FRANCE,
			
			_654 => Self::SAINT_HELENA_ASCENSIAN_AND_TRISTAN_DA_CUNHA,
			
			_574 => Self::NORFOLK_ISLAND,
			
			_724 => Self::SPAIN,
			
			_826 => Self::UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND,
			
			_831 => Self::GUERNSEY,
			
			_832 => Self::JERSEY,
			
			_833 => Self::ISLE_OF_MAN,
			
			_ => return Err(UnknownIso3166Dash1CodeError),
		};
		Ok(country)
	}
}

impl<'a> TryFrom<&'a str> for Iso3166Dash1Country
{
	type Error = UnknownStringVariantParseError;
	
	#[inline(always)]
	fn try_from(raw: &'a str) -> Result<Self, Self::Error>
	{
		Self::from_str(raw)
	}
}

impl FromStr for Iso3166Dash1Country
{
	type Err = UnknownStringVariantParseError;
	
	#[inline(always)]
	fn from_str(raw: &str) -> Result<Self, Self::Err>
	{
		match raw
		{
			"AFGHANISTAN" | "Afghanistan" | "the Islamic Republic of Afghanistan" => Ok(Self::AFGHANISTAN),
			
			"BRITISH INDIAN OCEAN TERRITORY" | "British Indian Ocean Territory (the)" => Ok(Self::BRITISH_INDIAN_OCEAN_TERRITORY),
			
			"FRANCE" | "France" | "the French Republic" => Ok(Self::FRANCE),
			
			"SPAIN" | "Spain" | "the Kingdom of Spain" => Ok(Self::SPAIN),
			
			"GUERNSEY" | "Guernsey" => Ok(Self::GUERNSEY),
			
			"ISLE OF MAN" | "Isle of Man" => Ok(Self::ISLE_OF_MAN),
			
			"JERSEY" | "Jersey" => Ok(Self::JERSEY),
			
			"NORFOLK ISLAND" | "Norfolk Island" => Ok(Self::NORFOLK_ISLAND),
			
			"UNITED KINGDOM OF GREAT BRITAIN AND NORTHERN IRELAND" | "United Kingdom of Great Britain and Northern Ireland (the)" | "the United Kingdom of Great Britain and Northern Ireland" => Ok(Self::UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND),
			
			// Exception reservations; names are guesses.
			
			"ASCENSION ISLAND" | "Ascension Island" => Ok(Self::ASCENSION_ISLAND),
			
			"CANARY ISLANDS" | "Canary Islands" => Ok(Self::CANARY_ISLANDS),
			
			"CEUTA, MELILLA" | "Ceuta, Melilla" => Ok(Self::CEUTA_MELILLA),
			
			"CLIPPERTON ISLAND" | "Clipperton Island" => Ok(Self::CLIPPERTON_ISLAND),
			
			"DIEGO GARCIA" | "Diego Garcia" => Ok(Self::DIEGO_GARCIA),
			
			"ISLAND OF SARK" | "Island of Sark" => Ok(Self::ISLAND_OF_SARK),
			
			"FRANCE, METROPOLITAN" | "France, Metropolitan" => Ok(Self::FRANCE_METROPOLITAN),
			
			"TRISTAN DA CUNHA" | "Tristan da Cunha" => Ok(Self::TRISTAN_DA_CUNHA),
			
			_ => Err(UnknownStringVariantParseError(raw.to_string())),
		}
	}
}

impl<'a> XmpAttributeValue<'a> for Iso3166Dash1Country
{
	type Error = UnknownStringVariantParseError;

	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		Self::from_str(raw)
	}

	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::Iso3166Dash1Country(error)
	}
}

impl Iso3166Dash1Country
{
	/// Afghanistan.
	pub const AFGHANISTAN: Self = Self::with_full_name(AF, AFG, _004, "AFGHANISTAN", "Afghanistan", "the Islamic Republic of Afghanistan");
	
	/// British Indian Ocean Territory (the).
	pub const BRITISH_INDIAN_OCEAN_TERRITORY: Self = Self::without_full_name(IO, IOT, _086, "BRITISH INDIAN OCEAN TERRITORY", "British Indian Ocean Territory (the)");
	
	/// France.
	pub const FRANCE: Self = Self::with_full_name(FR, FRA, _250, "FRANCE", "France", "the French Republic");
	
	/// Guernsey.
	pub const GUERNSEY: Self = Self::without_full_name(GG, GGY, _833, "GUERNSEY", "Guernsey");
	
	/// Isle of Man.
	pub const ISLE_OF_MAN: Self = Self::without_full_name(IM, IMN, _833, "ISLE OF MAN", "Isle of Man");
	
	/// Jersey.
	pub const JERSEY: Self = Self::without_full_name(JE, JEY, _832, "JERSEY", "Jersey");
	
	/// Saint Helena, Ascension and Tristan da Cunha.
	pub const SAINT_HELENA_ASCENSIAN_AND_TRISTAN_DA_CUNHA: Self = Self::without_full_name(SH, SHN, _654, "SAINT HELENA, ASCENSION AND TRISTAN DA CUNHA", "Saint Helena, Ascension and Tristan da Cunha");
	
	/// Norfolk Island.
	pub const NORFOLK_ISLAND: Self = Self::without_full_name(NF, NFK, _574, "NORFOLK ISLAND", "Norfolk Island");
	
	/// Spain.
	pub const SPAIN: Self = Self::with_full_name(ES, ESP, _724, "SPAIN", "Spain", "the Kingdom of Spain");
	
	/// United Kingdom of Great Britain and Northern Ireland (the).
	pub const UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND: Self = Self::with_full_name(GB, GBR, _826, "UNITED KINGDOM OF GREAT BRITAIN AND NORTHERN IRELAND", "United Kingdom of Great Britain and Northern Ireland (the)", "the United Kingdom of Great Britain and Northern Ireland");
	
	// Exceptional reservations.
	
	/// Ascension Island.
	///
	/// A territory of `SAINT_HELENA_ASCENSIAN_AND_TRISTAN_DA_CUNHA`.
	/// Only has an Alpha-2 code, `AC` and an Alpha-3 code `ASC`.
	pub const ASCENSION_ISLAND: Self = Self::SAINT_HELENA_ASCENSIAN_AND_TRISTAN_DA_CUNHA;
	
	/// Canary Islands.
	///
	/// A territory of `SPAIN`.
	/// Only has an Alpha-2 code, `IC`.
	/// Unofficially, Switzerland uses the Alpha-2 code, `XA`.
	pub const CANARY_ISLANDS: Self = Self::SPAIN;
	
	/// Ceuta, Melilla.
	///
	/// A territory of `SPAIN`.
	/// Only has an Alpha-2 code, `EA`.
	pub const CEUTA_MELILLA: Self = Self::SPAIN;
	
	/// Clipperton Island.
	///
	/// A territory of `FRANCE`.
	/// Only has an Alpha-2 code, `CP` and an Alpha-3 code `CPT`.
	pub const CLIPPERTON_ISLAND: Self = Self::FRANCE;
	
	/// Diego Garcia.
	///
	/// A territory of `BRITISH_INDIAN_OCEAN_TERRITORY`.
	/// Only has an Alpha-2 code, `DG` and an Alpha-3 code `DGA`.
	pub const DIEGO_GARCIA: Self = Self::BRITISH_INDIAN_OCEAN_TERRITORY;
	
	/// Island of Sark.
	///
	/// A territory of `GUERNSEY`.
	/// Only has an Alpha-2 code, `CQ`.
	pub const ISLAND_OF_SARK: Self = Self::GUERNSEY;
	
	/// This deleted reference is converted to France.
	pub const FRANCE_METROPOLITAN: Self = Self::FRANCE;
	
	/// Tristan da Cunha.
	///
	/// TA, TAA
	///
	/// A territory of `SAINT_HELENA_ASCENSIAN_AND_TRISTAN_DA_CUNHA`.
	/// Only has an Alpha-2 code, `TA` and an Alpha-3 code `TAA`.
	pub const TRISTAN_DA_CUNHA: Self = Self::SAINT_HELENA_ASCENSIAN_AND_TRISTAN_DA_CUNHA;
	
	#[allow(missing_docs)]
	const fn without_full_name(alpha_2_code: Iso3166Dash1Alpha2CountryCode, alpha_3_code: Iso3166Dash1Alpha3CountryCode, numeric_code: Iso3166Dash1NumericCountryCode, short_name: &'static str, short_name_lower_case: &'static str) -> Self
	{
		Self::new(alpha_2_code, alpha_3_code, numeric_code, short_name, short_name_lower_case, None)
	}
	
	#[allow(missing_docs)]
	const fn with_full_name(alpha_2_code: Iso3166Dash1Alpha2CountryCode, alpha_3_code: Iso3166Dash1Alpha3CountryCode, numeric_code: Iso3166Dash1NumericCountryCode, short_name: &'static str, short_name_lower_case: &'static str, full_name: &'static str) -> Self
	{
		Self::new(alpha_2_code, alpha_3_code, numeric_code, short_name, short_name_lower_case, Some(full_name))
	}
	
	#[inline(always)]
	const fn new(alpha_2_code: Iso3166Dash1Alpha2CountryCode, alpha_3_code: Iso3166Dash1Alpha3CountryCode, numeric_code: Iso3166Dash1NumericCountryCode, short_name: &'static str, short_name_lower_case: &'static str, full_name: Option<&'static str>) -> Self
	{
		Self
		{
			alpha_2_code,
			alpha_3_code,
			numeric_code,
			short_name,
			short_name_lower_case,
			full_name
		}
	}
}
