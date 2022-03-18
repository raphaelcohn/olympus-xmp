// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// BCP 47 `extlang`.
#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LanguageExtension
{
	/// Selected ISO 639 codes.
	language_extension_0: IanaRegisteredIso639Alpha3Code,
	
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
	fn parse<'a>(subtags: &mut MemchrIterator<'a, Hyphen>, iana_registered_iso_639_code: IanaRegisteredIso639Code) -> Result<(Either<Option<Self>, RegularGrandfathered>, NextSubtag<'a>), LanguageExtensionSubtagParseError>
	{
		use NextSubtag::*;
		use RegularGrandfathered::*;
		
		const no: IanaRegisteredIso639Code = IanaRegisteredIso639Code::from(b"no");
		const zh: IanaRegisteredIso639Code = IanaRegisteredIso639Code::from(b"zh");
		const bok: IanaRegisteredIso639Alpha3Code = IanaRegisteredIso639Alpha3Code::from(b"bok");
		const nyn: IanaRegisteredIso639Alpha3Code = IanaRegisteredIso639Alpha3Code::from(b"nyn");
		const min: IanaRegisteredIso639Alpha3Code = IanaRegisteredIso639Alpha3Code::from(b"min");
		const nan: IanaRegisteredIso639Alpha3Code = IanaRegisteredIso639Alpha3Code::from(b"nan");
		
		#[inline(always)]
		const fn right<'a>(regular_grandfathered: RegularGrandfathered) -> (Either<Option<LanguageExtension>, RegularGrandfathered>, NextSubtag<'a>)
		{
			(Right(regular_grandfathered), Exhausted)
		}
		
		#[inline(always)]
		const fn left(language_extension: LanguageExtension, next_subtag: NextSubtag) -> (Either<Option<LanguageExtension>, RegularGrandfathered>, NextSubtag)
		{
			(Left(Some(language_extension)), next_subtag)
		}
		
		// TODO: Create a macro;
		xxx;
		
		let language_extension_0 = match Self::parse_extended_language_subtag(subtags)?
		{
			Right(language_extension_0) => language_extension_0,
			
			Left(next_subtag) =>
			{
				debug_assert_ne!(next_subtag, Pending, "Pending never returned from parse_extended_language_subtag");
				return Ok((Left(None), next_subtag))
			},
		};
		
		let language_extension_1 = match Self::parse_extended_language_subtag(subtags)?
		{
			Right(language_extension_2) => language_extension_2,
			
			Left(next_subtag) => return Ok
			(
				match (next_subtag, iana_registered_iso_639_code, language_extension_0)
				{
					(Exhausted, no, bok) => right(no_bok),
					
					(Exhausted, no, nyn) => right(no_nyn),
					
					(Exhausted, zh, min) => right(zh_min),
					
					(Pending, _, _) => unreachable_code(format_args!("Pending never returned from parse_extended_language_subtag")),
					
					(next_subtag @ _, _, _) => left(Self::language_extension_0(language_extension_0), next_subtag),
				}
			),
		};
		
		match Self::parse_extended_language_subtag(subtags)?
		{
			Right(language_extension_3) => Some(language_extension_3),
			
			Left(next_subtag) => return Ok
			(
				match (next_subtag, iana_registered_iso_639_code, language_extension_0, language_extension_1)
				{
					(Exhausted, zh, min, nan) => right(zh_min_nan),
					
					(Pending, _, _, _) => unreachable_code(format_args!("Pending never returned from parse_extended_language_subtag")),
					
					(next_subtag @ _, _, _, _) => left(Self::language_extension_1(language_extension_0, language_extension_1), next_subtag),
				}
			)
		}
		
		Ok(left(Self::language_extension_2(language_extension_0, language_extension_1, language_extension_2), Pending))
	}
	
	#[inline(always)]
	const fn language_extension_0(language_extension_0: IanaRegisteredIso639Alpha3Code) -> Self
	{
		Self
		{
			language_extension_0,
			
			permanently_reserved: ArrayVec::new_const(),
		}
	}
	
	#[inline(always)]
	fn language_extension_1(language_extension_0: IanaRegisteredIso639Alpha3Code, language_extension_1: IanaRegisteredIso639Alpha3Code) -> Self
	{
		let mut permanently_reserved = ArrayVec::new_const();
		let mut_ptr = permanently_reserved.as_mut_ptr();
		unsafe
		{
			*mut_ptr = language_extension_1;
			permanently_reserved.set_len(1);
		}
		Self
		{
			language_extension_0,
			
			permanently_reserved,
		}
	}
	
	#[inline(always)]
	fn language_extension_2(language_extension_0: IanaRegisteredIso639Alpha3Code, language_extension_1: IanaRegisteredIso639Alpha3Code, language_extension_2: IanaRegisteredIso639Alpha3Code) -> Self
	{
		let mut permanently_reserved = ArrayVec::new_const();
		let mut_ptr = permanently_reserved.as_mut_ptr();
		unsafe
		{
			*mut_ptr = language_extension_1;
			*mut_ptr.add(1) = language_extension_2;
			permanently_reserved.set_len(2);
		}
		Self
		{
			language_extension_0,
			
			permanently_reserved,
		}
	}
	
	#[inline(always)]
	fn parse_extended_language_subtag<'a>(subtags: &mut MemchrIterator<'a, Hyphen>) -> Result<Either<NextSubtag<'a>, IanaRegisteredIso639Alpha3Code>, LanguageExtensionSubtagParseError>
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
			
			Right(iana_registered_iso_639_alpha_3_code) => Ok(Right(iana_registered_iso_639_alpha_3_code)),
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
