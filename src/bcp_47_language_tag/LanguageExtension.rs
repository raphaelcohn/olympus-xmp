// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// BCP 47 `extlang`.
#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LanguageExtension
{
	/// Selected ISO 639 codes.
	alpha3: IanaRegisteredIso639Alpha3Code,
	
	/// Will always be empty.
	permanently_reserved: ArrayVec<IanaRegisteredIso639Alpha3Code, 2>,
}

enum Remainder<'a>
{
	Exhausted,

	PushBack(&'a [u8])
}

impl LanguageExtension
{
	/*
		TODO:    Records other than those of type 'extlang' that contain a 'Preferred-
   Value' field MUST also have a 'Deprecated' field.  This field
   contains the date on which the tag or subtag was deprecated in favor
   of the preferred value.
   
   		TODO: 	For records of type 'extlang', the 'Preferred-Value' field appears
   without a corresponding 'Deprecated' field
	 */
	/*
	 extlang       = 3ALPHA              ; selected ISO 639 codes
			  *2("-" 3ALPHA)      ; permanently reserved
	 */
	fn parse<'a>(subtags: &mut MemchrIterator<'a, Hyphen>) -> Result<(Option<Self>, Remainder<'a>), LanguageExtensionTagParseError>
	{
		let potential_extended_language_subtag = match subtags.next()
		{
			None => return Ok((None, Remainder::Exhausted)),
			
			Some(first_extended_language_subtag) => first_extended_language_subtag,
		};
		
		const length: usize = 3;
		
		if potential_extended_language_subtag.len() != length
		{
			return Ok((None, Remainder::PushBack(potential_extended_language_subtag)))
		}
		
		let _ = Self::validate_as_either_iana_registered_iso_639_alpha_3_code_or_iana_registered_un_m49_region_code(potential_extended_language_subtag)?;
		
		unimplemented!();
	}
	
	#[inline(always)]
	fn validate_as_either_iana_registered_iso_639_alpha_3_code_or_iana_registered_un_m49_region_code(subtag: &[u8]) -> Result<Either<IanaRegisteredIso639Alpha3Code, IanaRegisteredUnM49RegionCode>, LanguageExtensionTagParseError>
	{
		unimplemented!();
	}
	
	#[inline(always)]
	fn validate_as_iana_registered_iso_639_alpha_3_code(subtag: &[u8], zeroth_byte: u8) -> Result<IanaRegisteredIso639Alpha3Code, LanguageExtensionTagParseError>
	{
		let mut converted = UninitialisedArray::<_, 3>::default();
		converted.convert(zeroth_byte, 0);
		for index in 1 .. 2
		{
			let byte = subtag.get_unchecked_value_safe(index);
			match to_lower_case(byte)
			{
				lower_case_byte @ a ..= z => converted.convert(lower_case_byte, index),
				
				_ => return Err(LanguageExtensionTagParseError::InvalidIanaRegisteredIso639Alpha3Code(InvalidAlphaError { length: 3, index, byte }))
			}
		}
		Ok(IanaRegisteredIso639Alpha3Code(converted.initialise()))
	}
	
	// TODO: Merge these routines somewhat, and check if other logic elsewhere can be merged.
	
	#[inline(always)]
	fn validate_as_iana_registered_un_m49_region_code(subtag: &[u8], zeroth_byte: u8) -> Result<IanaRegisteredUnM49RegionCode, LanguageExtensionTagParseError>
	{
		let mut converted = UninitialisedArray::<_, 3>::default();
		converted.convert(zeroth_byte, 0);
		for index in 1 .. 2
		{
			let byte = subtag.get_unchecked_value_safe(index);
			match byte
			{
				_0 ..= _9 => converted.convert(byte, index),
				
				_ => return Err(LanguageExtensionTagParseError::InvalidIanaRegisteredUnM49RegionCode(InvalidDigitError { length: 3, index, byte }))
			}
		}
		Ok(IanaRegisteredUnM49RegionCode(converted.initialise()))
	}
}
