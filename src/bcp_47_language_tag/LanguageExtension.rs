// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// BCP 47 `extlang`.
#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LanguageExtension
{
	/// Selected ISO 639 codes.
	first_extended_language_subtag: IanaRegisteredIso639Alpha3Code,
	
	/// Will always be empty.
	permanently_reserved: ArrayVec<IanaRegisteredIso639Alpha3Code, { LanguageExtension::PermanentlyReservedCount }>,
}

impl LanguageExtension
{
	const PermanentlyReservedCount: usize = 2;
	
	const length: usize = 3;
	
	/// ```bash
	/// extlang = 3ALPHA          ; Selected ISO 639 codes.
	///           *2("-" 3ALPHA)  ; Permanently reserved.
	/// ```
	fn parse<'a>(subtags: &mut MemchrIterator<'a, Hyphen>) -> Result<(Option<Self>, NextSubtag<'a>), LanguageExtensionSubtagParseError>
	{
		macro_rules! parse_extended_language_subtag
		{
			($on_success: expr, $returns: expr) =>
			{
				match Self::parse_extended_language_subtag(subtags, $on_success)?
				{
					Left(next_subtag) => return Ok(($returns, next_subtag)),
					
					Right(on_success) => on_success,
				}
			}
		}
		
		let mut language_extension = parse_extended_language_subtag!(Self::new, None);
		parse_extended_language_subtag!(|extended_language_subtag| language_extension.push_unchecked::<0>(extended_language_subtag), Some(language_extension));
		parse_extended_language_subtag!(|extended_language_subtag| language_extension.push_unchecked::<1>(extended_language_subtag), Some(language_extension));
		
		Ok((Some(language_extension), NextSubtag::Pending))
	}
	
	#[inline(always)]
	const fn new(first_extended_language_subtag: IanaRegisteredIso639Alpha3Code) -> Self
	{
		Self
		{
			first_extended_language_subtag,
			
			permanently_reserved: ArrayVec::new_const(),
		}
	}
	
	/// More efficient than `ArrayVec::push_unchecked()` as it allows the use of const addition and loop unrolling.
	#[inline(always)]
	fn push_unchecked<const index: usize>(&mut self, extended_language_subtag: IanaRegisteredIso639Alpha3Code)
	{
		let pointer = self.permanently_reserved.as_mut_ptr();
		unsafe
		{
			pointer.add(index).write(extended_language_subtag);
			self.permanently_reserved.set_len(index + 1);
		}
	}
	
	#[inline(always)]
	fn parse_extended_language_subtag<'a, R>(subtags: &mut MemchrIterator<'a, Hyphen>, mut on_success: impl FnMut(IanaRegisteredIso639Alpha3Code) -> R) -> Result<Either<NextSubtag<'a>, R>, LanguageExtensionSubtagParseError>
	{
		use NextSubtag::*;
		
		#[inline(always)]
		const fn next_subtag<R>(next_subtag: NextSubtag) -> Result<Either<NextSubtag, R>, LanguageExtensionSubtagParseError>
		{
			Ok(Left(next_subtag))
		}
		
		let first_potential_extended_language_subtag = match subtags.next()
		{
			None => return next_subtag(Exhausted),
			
			Some(first_potential_extended_language_subtag) => first_potential_extended_language_subtag,
		};
		
		if first_potential_extended_language_subtag.len() != Self::length
		{
			return next_subtag(Next(first_potential_extended_language_subtag))
		}
		
		match Self::validate_as_either_iana_registered_iso_639_alpha_3_code_or_iana_registered_un_m49_region_code(first_potential_extended_language_subtag)?
		{
			Left(iana_registered_un_m49_region_code) => next_subtag(IanaRegisteredUnM49RegionCode(iana_registered_un_m49_region_code)),
			
			Right(iana_registered_iso_639_alpha_3_code) => Ok(Right(on_success(iana_registered_iso_639_alpha_3_code))),
		}
	}
	
	#[inline(always)]
	fn validate_as_either_iana_registered_iso_639_alpha_3_code_or_iana_registered_un_m49_region_code(subtag: &[u8]) -> Result<Either<IanaRegisteredUnM49RegionCode, IanaRegisteredIso639Alpha3Code>, LanguageExtensionSubtagParseError>
	{
		match subtag.get_unchecked_value_safe(0usize)
		{
			zeroth_byte @ _0 ..= _9 => Self::validate_as_iana_registered_un_m49_region_code(subtag, zeroth_byte),
			
			upper_case_zeroth_byte @ A ..= Z => Self::validate_as_iana_registered_iso_639_alpha_3_code(subtag, to_lower_case(upper_case_zeroth_byte)),
			
			zeroth_byte @ a ..= z => Self::validate_as_iana_registered_iso_639_alpha_3_code(subtag, zeroth_byte),
			
			_ => Err(LanguageExtensionSubtagParseError::Invalid(array_vec_u8::<{ Self::length }>(subtag)))
		}
	}
	
	#[inline(always)]
	fn validate_as_iana_registered_un_m49_region_code(subtag: &[u8], zeroth_byte: u8) -> Result<Either<IanaRegisteredUnM49RegionCode, IanaRegisteredIso639Alpha3Code>, LanguageExtensionSubtagParseError>
	{
		Self::validate_as_iana_registered::<_, Digit, _, _, _, _>
		(
			subtag,
			zeroth_byte,
			|byte| if byte >= _0 && byte <= _9
			{
				Some(byte)
			}
			else
			{
				None
			},
			IanaRegisteredUnM49RegionCode,
			Left,
			|index, byte| LanguageExtensionSubtagParseError::InvalidIanaRegisteredUnM49RegionCode(InvalidDigitError { length: Self::length, index, byte })
		)
	}
	
	#[inline(always)]
	fn validate_as_iana_registered_iso_639_alpha_3_code(subtag: &[u8], zeroth_byte: u8) -> Result<Either<IanaRegisteredUnM49RegionCode, IanaRegisteredIso639Alpha3Code>, LanguageExtensionSubtagParseError>
	{
		Self::validate_as_iana_registered::<_, Alpha, _, _, _, _>
		(
			subtag,
			zeroth_byte,
			|byte|
			{
				let lower_case_byte = to_lower_case(byte);
				if lower_case_byte >= a && lower_case_byte <= z
				{
					Some(lower_case_byte)
				}
				else
				{
					None
				}
			},
			IanaRegisteredIso639Alpha3Code,
			Right,
			|index, byte| LanguageExtensionSubtagParseError::InvalidIanaRegisteredIso639Alpha3Code(InvalidAlphaError { length: Self::length, index, byte })
		)
	}
	
	#[inline(always)]
	fn validate_as_iana_registered<IR, RB: RestrictedByte, Validate: Copy + FnOnce(u8) -> Option<u8>, Constructor: FnOnce([RB; Self::length]) -> IR, EitherConstructor: FnOnce(IR) -> Either<IanaRegisteredUnM49RegionCode, IanaRegisteredIso639Alpha3Code>, Error: FnOnce(usize, u8) -> LanguageExtensionSubtagParseError>(subtag: &[u8], zeroth_byte: u8, validate: Validate, constructor: Constructor, either: EitherConstructor, error: Error) -> Result<Either<IanaRegisteredUnM49RegionCode, IanaRegisteredIso639Alpha3Code>, LanguageExtensionSubtagParseError>
	{
		const length: usize = LanguageExtension::length;
		let mut converted = UninitialisedArray::<RB, length>::default();
		converted.convert(zeroth_byte, 0);
		for index in 1 .. Self::length
		{
			let byte = subtag.get_unchecked_value_safe(index);
			
			if let Some(validate_byte) = validate(byte)
			{
				converted.convert(validate_byte, index)
			}
			else
			{
				return Err(error(index, byte))
			}
		}
		Ok(either(constructor(converted.initialise())))
	}
}
