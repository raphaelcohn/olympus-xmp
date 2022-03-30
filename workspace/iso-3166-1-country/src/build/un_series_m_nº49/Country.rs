// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct Country
{
	names: Names,
	
	abbreviations: Abbreviations,
	
	iso_3166_1_alpha_code: Iso3166Dash1AlphaCode,
	
	developing: Developing,
	
	replacements: &'static [M49Code],
}

impl AsRef<Names> for Country
{
	#[inline(always)]
	fn as_ref(&self) -> &Names
	{
		&self.names
	}
}

impl AsMut<Names> for Country
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut Names
	{
		&mut self.names
	}
}

impl Country
{
	const NoReplacements: &'static [M49Code] = &[];
	
	#[inline(always)]
	fn extant(names: Names, abbreviations: Option<TwelveCharacterAbbreviation>, iso_3166_1_alpha2_code: Iso3166Dash1Alpha2Code, iso_3166_1_alpha3_code: Option<Iso3166Dash1Alpha3Code>, developing: Developing) -> Self
	{
		use Abbreviations::*;
		use Iso3166Dash1AlphaCode::*;
		
		Self
		{
			names,
			
			abbreviations: match abbreviations
			{
				None => AbsentRevision3Onwards,
				
				Some(english_twelve_character_abbreviation) => Revision3Onwards
				{
					english_twelve_character_abbreviation
				},
			},
			
			iso_3166_1_alpha_code: match iso_3166_1_alpha3_code
			{
				None => Alpha2Only(iso_3166_1_alpha2_code),
				
				Some(iso_3166_1_alpha3_code) => Both
				{
					alpha2: iso_3166_1_alpha2_code,
					
					alpha3: iso_3166_1_alpha3_code,
				}
			},
			
			developing,
			
			replacements: Self::NoReplacements
		}
	}
	
	#[inline(always)]
	fn statistical(arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str, english_twelve_character_abbreviation: &'static [u8], iso_3166_1_alpha2_code: &[u8; 2], iso_3166_1_alpha3_code: &[u8; 3]) -> Self
	{
		Self
		{
			names: Names::new(arabic, chinese, english, french, russian, spanish),
			
			abbreviations: Abbreviations::Revision3Onwards
			{
				english_twelve_character_abbreviation: TwelveCharacterAbbreviation::new(english_twelve_character_abbreviation)
			},
			
			iso_3166_1_alpha_code: Iso3166Dash1AlphaCode::Both
			{
				alpha2: Iso3166Dash1Alpha2Code::from(iso_3166_1_alpha2_code),
				alpha3: Iso3166Dash1Alpha3Code::from(iso_3166_1_alpha3_code),
			},
			
			developing: Developing::default(),
			
			replacements: Self::NoReplacements,
		}
	}
	
	#[inline(always)]
	fn obsolete(arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str, english_twelve_character_abbreviation: &'static [u8], alpha_2_and_alpha_3: Option<(Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code)>, replacements: &'static [M49Code]) -> Self
	{
		use Iso3166Dash1AlphaCode::*;
		
		Self
		{
			names: if arabic.is_empty() && chinese.is_empty() && russian.is_empty() && spanish.is_empty()
			{
				Names::english_and_french_only(english, french)
			}
			else
			{
				Names::new(arabic, chinese, english, french, russian, spanish)
			},
			
			abbreviations: Abbreviations::Revision3Onwards
			{
				english_twelve_character_abbreviation: TwelveCharacterAbbreviation::new(english_twelve_character_abbreviation)
			},
			
			iso_3166_1_alpha_code: match alpha_2_and_alpha_3
			{
				None => PredatesOrNotApplicable,
				
				Some((alpha2, alpha3)) => Both
				{
					alpha2,
					alpha3,
				}
			},
			
			developing: Developing::default(),
			
			replacements:
			{
				assert!(!replacements.is_empty(), "replacements are empty");
				replacements
			},
		}
	}
	
	#[inline(always)]
	fn country_like_1996_onwards(arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str) -> Self
	{
		Self
		{
			names: Names::new(arabic, chinese, english, french, russian, spanish),
			
			abbreviations: Abbreviations::AbsentRevision3Onwards,
			
			iso_3166_1_alpha_code: Iso3166Dash1AlphaCode::PredatesOrNotApplicable,
			
			developing: Developing::default(),
			
			replacements: Self::NoReplacements,
		}
	}
	
	#[inline(always)]
	fn country_like_1970(arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str, english_twelve_character_abbreviation: &'static [u8], english_legacy_eight_character_abbreviation: &'static [u8], english_legacy_four_character_abbreviation: &'static [u8], french_legacy_twelve_character_abbreviation: &'static [u8], french_legacy_eight_character_abbreviation: &'static [u8], french_legacy_four_character_abbreviation: &'static [u8]) -> Self
	{
		Self
		{
			names: Names::new(arabic, chinese, english, french, russian, spanish),
			
			abbreviations: Abbreviations::revision_0(english_twelve_character_abbreviation, english_legacy_eight_character_abbreviation, english_legacy_four_character_abbreviation, french_legacy_twelve_character_abbreviation, french_legacy_eight_character_abbreviation, french_legacy_four_character_abbreviation),
			
			iso_3166_1_alpha_code: Iso3166Dash1AlphaCode::PredatesOrNotApplicable,
			
			developing: Developing::default(),
			
			replacements: Self::NoReplacements,
		}
	}
}
