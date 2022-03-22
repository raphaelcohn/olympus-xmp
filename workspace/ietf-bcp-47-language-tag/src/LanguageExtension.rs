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
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn language_extension_0(&self) -> IanaRegisteredIso639Alpha3Code
	{
		self.language_extension_0
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn permanently_reserved(&self) -> &[IanaRegisteredIso639Alpha3Code]
	{
		self.permanently_reserved.as_slice()
	}
	
	/// ```bash
	/// extlang = 3ALPHA          ; Selected ISO 639 codes.
	///           *2("-" 3ALPHA)  ; Permanently reserved.
	/// ```
	fn parse<'a>(subtags: &mut Subtags<'a>, iana_registered_iso_639_code: IanaRegisteredIso639Code) -> Result<Either<(Language, NextSubtagAfterLanguageExtension<'a>), IetfBcp47LanguageTag>, LanguageExtensionSubtagParseError>
	{
		use NextSubtagAfterLanguageExtension::*;
		use RegularGrandfathered::*;
		
		const no: IanaRegisteredIso639Code = IanaRegisteredIso639Code::from(b"no");
		const zh: IanaRegisteredIso639Code = IanaRegisteredIso639Code::from(b"zh");
		const bok: IanaRegisteredIso639Alpha3Code = IanaRegisteredIso639Alpha3Code::from(b"bok");
		const nyn: IanaRegisteredIso639Alpha3Code = IanaRegisteredIso639Alpha3Code::from(b"nyn");
		const min: IanaRegisteredIso639Alpha3Code = IanaRegisteredIso639Alpha3Code::from(b"min");
		const nan: IanaRegisteredIso639Alpha3Code = IanaRegisteredIso639Alpha3Code::from(b"nan");
		const PendingImpossibleMessage: &'static str = "Pending never returned from parse_extended_language_subtag";
		
		type Output<'a> = Either<(Language, NextSubtagAfterLanguageExtension<'a>), IetfBcp47LanguageTag>;
		
		#[inline(always)]
		const fn left(iana_registered_iso_639_code: IanaRegisteredIso639Code, language_extension: Option<LanguageExtension>, next_subtag: NextSubtagAfterLanguageExtension) -> Output
		{
			Left
			(
				(
					Language::Ordinary
					(
						OrdinaryLanguage
						{
							iana_registered_iso_639_code,
							
							language_extension,
						}
					),
					next_subtag
				)
			)
		}
		
		#[inline(always)]
		const fn left_none(iana_registered_iso_639_code: IanaRegisteredIso639Code, next_subtag: NextSubtagAfterLanguageExtension) -> Output
		{
			left(iana_registered_iso_639_code, None, next_subtag)
		}
		
		#[inline(always)]
		const fn left_some(iana_registered_iso_639_code: IanaRegisteredIso639Code, language_extension: LanguageExtension, next_subtag: NextSubtagAfterLanguageExtension) -> Output
		{
			left(iana_registered_iso_639_code, Some(language_extension), next_subtag)
		}
		
		#[inline(always)]
		const fn right<'a>(regular_grandfathered: RegularGrandfathered) -> Output<'a>
		{
			Right(IetfBcp47LanguageTag::regular_grandfathered(regular_grandfathered))
		}
		
		#[inline(always)]
		fn unreachable_code_because_pending_impossible() -> !
		{
			unreachable_code(format_args!("{}", PendingImpossibleMessage))
		}
		
		macro_rules! parse_extended_language_subtag
		{
			($subtags: ident, $next_subtag: ident, $ok: block) =>
			{
				match Self::parse_extended_language_subtag(subtags)?
				{
					Right(language_extension) => language_extension,
					
					Left($next_subtag) => return Ok($ok),
				}
			}
		}
		
		let language_extension_0 = parse_extended_language_subtag!(subtags, next_subtag,
		{
			debug_assert_ne!(next_subtag, Pending, "{}", PendingImpossibleMessage);
			left_none(iana_registered_iso_639_code, next_subtag)
		});
		
		let language_extension_1 = parse_extended_language_subtag!(subtags, next_subtag,
		{
			match (next_subtag, iana_registered_iso_639_code, language_extension_0)
			{
				(Exhausted, no, bok) => right(no_bok),
				
				(Exhausted, no, nyn) => right(no_nyn),
				
				(Exhausted, zh, min) => right(zh_min),
				
				(Pending, _, _) => unreachable_code_because_pending_impossible(),
				
				(next_subtag @ _, _, _) => left_some(iana_registered_iso_639_code, Self::new_language_extension_0(language_extension_0), next_subtag),
			}
		});
		
		let language_extension_2 = parse_extended_language_subtag!(subtags, next_subtag,
		{
			match (next_subtag, iana_registered_iso_639_code, language_extension_0, language_extension_1)
			{
				(Exhausted, zh, min, nan) => right(zh_min_nan),
				
				(Pending, _, _, _) => unreachable_code_because_pending_impossible(),
				
				(next_subtag @ _, _, _, _) => left_some(iana_registered_iso_639_code, Self::new_language_extension_1(language_extension_0, language_extension_1), next_subtag),
			}
		});
		
		Ok(left_some(iana_registered_iso_639_code, Self::new_language_extension_2(language_extension_0, language_extension_1, language_extension_2), Pending))
	}
	
	#[inline(always)]
	fn parse_extended_language_subtag<'a>(subtags: &mut Subtags<'a>) -> Result<Either<NextSubtagAfterLanguageExtension<'a>, IanaRegisteredIso639Alpha3Code>, LanguageExtensionSubtagParseError>
	{
		use NextSubtagAfterLanguageExtension::*;
		
		#[inline(always)]
		const fn next_subtag<R>(next_subtag: NextSubtagAfterLanguageExtension) -> Result<Either<NextSubtagAfterLanguageExtension, R>, LanguageExtensionSubtagParseError>
		{
			Ok(Left(next_subtag))
		}
		
		let potential_extended_language_subtag = return_next!(subtags, next_subtag(Exhausted));
		
		if potential_extended_language_subtag.len() != Self::length
		{
			return next_subtag(Next(potential_extended_language_subtag))
		}
		
		match Self::validate_as_either_iana_registered_iso_639_alpha_3_code_or_iana_registered_un_m49_region_code(potential_extended_language_subtag)?
		{
			Left(iana_registered_un_m49_region_code) => next_subtag(IanaRegisteredUnM49RegionCode(iana_registered_un_m49_region_code)),
			
			Right(iana_registered_iso_639_alpha_3_code) => Ok(Right(iana_registered_iso_639_alpha_3_code)),
		}
	}
	
	#[inline(always)]
	fn validate_as_either_iana_registered_iso_639_alpha_3_code_or_iana_registered_un_m49_region_code(subtag: &[u8]) -> Result<Either<IanaRegisteredUnM49RegionCode, IanaRegisteredIso639Alpha3Code>, LanguageExtensionSubtagParseError>
	{
		match subtag.byte_0()
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
			|index, byte| LanguageExtensionSubtagParseError::InvalidIanaRegisteredUnM49RegionCode(Digit::error::<{Self::length}>(index, byte))
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
			|index, byte| LanguageExtensionSubtagParseError::InvalidIanaRegisteredIso639Alpha3Code(Alpha::error::<{Self::length}>(index, byte))
		)
	}
	
	#[unroll_for_loops]
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
	
	#[inline(always)]
	const fn new_language_extension_0(language_extension_0: IanaRegisteredIso639Alpha3Code) -> Self
	{
		Self
		{
			language_extension_0,
			
			permanently_reserved: ArrayVec::new_const(),
		}
	}
	
	#[inline(always)]
	fn new_language_extension_1(language_extension_0: IanaRegisteredIso639Alpha3Code, language_extension_1: IanaRegisteredIso639Alpha3Code) -> Self
	{
		Self::new_language_extension_::<_, 1>(language_extension_0, language_extension_1, |_| {})
	}
	
	#[inline(always)]
	fn new_language_extension_2(language_extension_0: IanaRegisteredIso639Alpha3Code, language_extension_1: IanaRegisteredIso639Alpha3Code, language_extension_2: IanaRegisteredIso639Alpha3Code) -> Self
	{
		Self::new_language_extension_::<_, 2>(language_extension_0, language_extension_1, |mut_ptr| unsafe { mut_ptr.add(1).write(language_extension_2) })
	}
	
	#[inline(always)]
	fn new_language_extension_<LE3: FnOnce(*mut IanaRegisteredIso639Alpha3Code), const length: usize>(language_extension_0: IanaRegisteredIso639Alpha3Code, language_extension_1: IanaRegisteredIso639Alpha3Code, language_extension_3: LE3) -> Self
	{
		let mut permanently_reserved: ArrayVec<_, { Self::PermanentlyReservedCount }> = ArrayVec::new_const();
		let mut_ptr: *mut IanaRegisteredIso639Alpha3Code = permanently_reserved.as_mut_ptr();
		unsafe { mut_ptr.write(language_extension_1) };
		language_extension_3(mut_ptr);
		unsafe { permanently_reserved.set_len(length) };
		
		Self
		{
			language_extension_0,
			
			permanently_reserved,
		}
	}
}
