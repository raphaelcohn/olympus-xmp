// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::build::un_series_m_nº49::pdf_extracts::validate_twelve_character_abbreviation;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct Names
{
	arabic: &'static str,
	
	chinese: &'static str,
	
	english: &'static str,
	
	french: &'static str,
	
	russian: &'static str,
	
	spanish: &'static str,
	
	twelve_character_abbreviation: &'static [u8],
}

impl AsMut<Names> for Names
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut Self
	{
		self
	}
}

impl Names
{
	#[inline(always)]
	fn english_and_french_only(english: &'static str, french: &'static str, twelve_character_abbreviation: &'static [u8]) -> Self
	{
		validate_twelve_character_abbreviation(twelve_character_abbreviation);
		assert!(!english.is_empty(), "english is empty");
		assert!(!french.is_empty(), "french is empty");
		
		Self
		{
			arabic: "",
			chinese: "",
			english,
			french,
			russian: "",
			spanish: "",
			twelve_character_abbreviation,
		}
	}
	
	#[inline(always)]
	fn new(arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str, twelve_character_abbreviation: &'static [u8]) -> Self
	{
		validate_twelve_character_abbreviation(twelve_character_abbreviation);
		assert!(!arabic.is_empty(), "arabic is empty");
		assert!(!chinese.is_empty(), "chinese is empty");
		assert!(!english.is_empty(), "english is empty");
		assert!(!french.is_empty(), "french is empty");
		assert!(!russian.is_empty(), "russian is empty");
		assert!(!spanish.is_empty(), "spanish is empty");
		
		Self
		{
			arabic,
			chinese,
			english,
			french,
			russian,
			spanish,
			twelve_character_abbreviation,
		}
	}
}
