// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::bcp_47_language_tag::parser::ScriptParseError::InvalidAlpha;

/// As defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt).
/// And RFC 5646.
/// <https://en.wikipedia.org/wiki/IETF_language_tag> is helpful.
/// <https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry> is official.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Bcp47LanguageTag
{
	#[allow(missing_docs)]
	Normal(Normal),
	
	#[allow(missing_docs)]
	PrivateUse(PrivateUse),

	#[allow(missing_docs)]
	Grandfathered(Grandfathered),
}

impl const From<Normal> for Bcp47LanguageTag
{
	#[inline(always)]
	fn from(normal: Normal) -> Self
	{
		Self::Normal(normal)
	}
}

impl const From<Grandfathered> for Bcp47LanguageTag
{
	#[inline(always)]
	fn from(grandfathered: Grandfathered) -> Self
	{
		Self::Grandfathered(grandfathered)
	}
}

impl const From<PrivateUse> for Bcp47LanguageTag
{
	#[inline(always)]
	fn from(private_use: PrivateUse) -> Self
	{
		Self::PrivateUse(private_use)
	}
}

impl Bcp47LanguageTag
{
	#[allow(missing_docs)]
	pub fn parse(language_tag: &str) -> Result<Self, Bcp47LanguageTagParseError>
	{
		let subtags = MemchrIterator::from_str(language_tag);
		Self::parse_bcp47_language_subtag(subtags)
	}
	
	#[inline(always)]
	fn parse_from_language(mut subtags: MemchrIterator<Hyphen>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		let (language, next_subtag) =
		{
			use LanguageFirstSubtagParseError::*;
			
			let first_subtag = subtags.next_first();
			match first_subtag.len()
			{
				0 => return_error!(FirstSubtagLengthIsZero),
				
				1 => return Self::parse_x_or_i_subtag(first_subtag, subtags),
				
				2 => parse_ordinary_language_subtag!(first_subtag, subtags, parse_2),
				
				3 => parse_ordinary_language_subtag!(first_subtag, subtags, parse_3),
				
				4 => parse_reserved_language_subtag!(first_subtag),
				
				5 => parse_registered_language_subtag!(first_subtag, 5, Alpha5),
				
				6 => parse_registered_language_subtag!(first_subtag, 6, Alpha6),
				
				7 => parse_registered_language_subtag!(first_subtag, 7, Alpha7),
				
				8 => parse_registered_language_subtag!(first_subtag, 8, Alpha8),
				
				_ => return_error!(FirstSubtagLengthIsMoreThanEight),
			}
		};
		
		use NextSubtag::*;
		match next_subtag
		{
			Exhausted => Self::from_language_ok(language),
			
			Next(subtag) => parse_from_script(subtag, subtags, language),
			
			IanaRegisteredUnM49RegionCode(iana_registered_un_m49_region_code) => Self::parse_from_variant(subtags, language, None, Some(IanaRegisteredRegionCode::IanaRegisteredUnM49RegionCode(iana_registered_un_m49_region_code)))
			
			Pending => match subtags.next()
			{
				None => Self::from_language_ok(language),
				
				Some(subtag) => parse_from_script(subtag, subtags, language),
			},
		}
	}

	#[inline(always)]
	fn parse_from_script<'a>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language) -> Result<Self, Bcp47LanguageTagParseError>
	{
		use ScriptParseError::*;
		
		match subtag.len()
		{
			0 => return_error!(SubtagOfZero),
			
			1 => match subtag.get_unchecked_value_safe(0)
			{
				X | x => Self::parse_from_private_use(subtags, language, None, None, Default::default(), Default::default()),
				
				validated_extension_code @ (_0 ..= _9 | A ..= W | Y ..= Z | a ..= w | y ..= z) => Self::parse_from_extension(validated_extension_code, subtags, language, None, None, Default::default()),
				
				invalid_extension_code @ _ => return_error!(InvalidExtensionSingleton(invalid_extension_code))
			}
			
			2 => Self::parse_from_region_2(subtag, subtags, language, None),
			
			3 => Self::parse_from_region_3(subtag, subtags, language, None),
			
			4 => match subtag.get_unchecked_value_safe(0)
			{
				digit @ _0 ..= _9 => Self::parse_from_variant_4_first_digit(subtag, subtags, language, None, None, Digit::construct(digit)),
				
				alpha @ (A ..= Z | a ..= z) =>
				{
					let initial = UpperCaseAlpha::construct(to_upper_case(alpha));
					let script = Alpha::validate_and_convert_array(subtag.get_unchecked_range_safe(1 .. ), |alpha_array| IanaRegisteredIso15924ScriptCode(initial, alpha_array), InvalidAlpha)?;
					Self::parse_from_region(subtags, language, Some(script))
				}
				
				byte @ _ => return_error!(InvalidAlphanumeric(InvalidAlphanumericError { length: 4, index: 0, byte })),
			}
			
			5 => Self::parse_from_variant_5(subtag, subtags, language, None, None),
			
			6 => Self::parse_from_variant_6(subtag, subtags, language, None, None),
			
			7 => Self::parse_from_variant_7(subtag, subtags, language, None, None),
			
			8 => Self::parse_from_variant_8(subtag, subtags, language, None, None),
			
			length @ _ => return_error!(SubtagMoreThanEight { length }),
		}
	}
	
	#[inline(always)]
	fn parse_from_region_2<'a>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		const length: usize = 2;
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_region_n::<UpperCaseAlpha, IanaRegisteredIso3166Dash1Alpha2CountryCode, _, _, length>(subtag, subtags, language, script, IanaRegisteredIso3166Dash1Alpha2CountryCode, RegionParseError::InvalidDigit)
	}
	
	#[inline(always)]
	fn parse_from_region_3<'a>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		const length: usize = 3;
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_region_n::<Digit, IanaRegisteredUnM49RegionCode, _, _, length>(subtag, subtags, language, script, IanaRegisteredUnM49RegionCode, RegionParseError::InvalidDigit)
	}
	
	#[inline(always)]
	fn parse_from_region_n<'a, RB: RestrictedByte, F: Into<IanaRegisteredRegionCode>, OkConstructor: FnOnce([RB; length]) -> F, ErrorConstructor: FnOnce(RB::Error) -> RegionParseError, const length: usize>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, constructor: OkConstructor, error: ErrorConstructor) -> Result<Self, Bcp47LanguageTagParseError>
	{
		debug_assert_eq!(subtag.len(), length);
		let code = RB::validate_and_convert_array::<_, _, _, _, length>(subtag, constructor, error)?;
		Self::parse_from_variant(subtags, language, script, Some(code.into()))
	}
	
	#[inline(always)]
	fn parse_from_region<'a>(mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		
		/*
			TODO Remember regular & irregular variants
		 */
		xxxx;
		
	}
	
	#[inline(always)]
	fn parse_from_variant(mut subtags: MemchrIterator<Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		use IrregularGrandfathered::*;
		use VariantParseError::*;
		
		const SecondRegionLength: usize = 2;
		const OedLength: usize = 3;
		const en: Language = Language::from("en");
		const sgn: Language = Language::from("sgn");
		const GB: Option<IanaRegisteredRegionCode> = Some(IanaRegisteredRegionCode::from(b"GB"));
		const BE: Option<IanaRegisteredRegionCode> = Some(IanaRegisteredRegionCode::from(b"BE"));
		const CH: Option<IanaRegisteredRegionCode> = Some(IanaRegisteredRegionCode::from(b"CH"));
		const FR: [UpperCaseAlpha; SecondRegionLength] = UpperCaseAlpha::new_array_unchecked(b"FR");
		const NL: [UpperCaseAlpha; SecondRegionLength] = UpperCaseAlpha::new_array_unchecked(b"NL");
		const DE: [UpperCaseAlpha; SecondRegionLength] = UpperCaseAlpha::new_array_unchecked(b"DE");
		const oed: [Alpha; OedLength] = Alpha::new_array_unchecked(b"oed");
		
		let subtag: &[u8] = return_next!(subtags, Self::from_region_ok(language, script, region));
		
		match subtag.len()
		{
			0 => return_error!(VariantOfZero),
			
			1 => match subtag.get_unchecked_value_safe(0)
			{
				x | X => Self::parse_from_private_use(subtags, language, script, region, Default::default(), Default::default()),
				
				validated_extension_code @ (_0 ..= _9 | A ..= W | Y ..= Z | a ..= w | y ..= z) => Self::parse_from_extension(validated_extension_code, subtags, language, script, region, Default::default()),
				
				invalid_extension_code @ _ => return_error!(InvalidExtensionSingleton(invalid_extension_code))
			},
			
			SecondRegionLength =>
			{
				if subtags.is_empty() && language == sgn
				{
					#[inline(always)]
					fn second_region(subtag: &[u8]) -> Result<[UpperCaseAlpha; SecondRegionLength], VariantParseError>
					{
						UpperCaseAlpha::validate_and_convert_array::<_, _, _, _, SecondRegionLength>(subtag, |upper_case_alpha_array| upper_case_alpha_array, InvalidUpperCaseAlpha)
					}
					match region
					{
						BE => match second_region(subtag)?
						{
							FR => return Self::from_irregular_granfathered_ok(sgn_BE_FR),
							
							NL => return Self::from_irregular_granfathered_ok(sgn_BE_NL),
							
							_ => (),
						}
						
						CH => if second_region(subtag)? == DE
						{
							return Self::from_irregular_granfathered_ok(sgn_CH_DE)
						},
						
						_ => (),
					}
				}
				return_error!(VariantOfTwo)
			}
			
			OedLength =>
			{
				if subtags.is_empty() && language == en && region == GB
				{
					let possibly_oed = Alpha::validate_and_convert_array::<_, _, _, _, OedLength>(subtag, |upper_case_alpha_array| upper_case_alpha_array, InvalidAlpha)?;
					if possibly_oed == oed
					{
						return Self::from_irregular_granfathered_ok(en_GB_oed)
					}
				}
				return_error!(VariantOfThree)
			}
			
			4 => match subtag.get_unchecked_value_safe(0)
			{
				digit @ _0 ..= _9 => Self::parse_from_variant_4_first_digit(subtag, subtags, language, None, None, Digit::construct(digit)),
				
				byte @ _ => return_error!(InvalidDigit(InvalidDigitError { length: 4, index: 0, byte })),
			},
			
			5 => Self::parse_from_variant_5(subtag, subtags, language, script, region),
			
			6 => Self::parse_from_variant_6(subtag, subtags, language, script, region),
			
			7 => Self::parse_from_variant_7(subtag, subtags, language, script, region),
			
			8 => Self::parse_from_variant_8(subtag, subtags, language, script, region),
			
			length @ _ => return_error!(VariantMoreThanEight { length }),
		}
		
		Self::parse_remaining_variants(HashSet::new(), subtags, language, script, region)
	}
	
	#[inline(always)]
	fn parse_from_variant_4_first_digit<'a>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, first_digit: Digit) -> Result<Self, Bcp47LanguageTagParseError>
	{
		const length: usize = 4;
		const NumberOfDigits: usize = 1;
		const alphanumeric_array_length: usize = length - NumberOfDigits;
		debug_assert_eq!(subtag.len(), length);
		
		let alphanumeric_array = Alphanumeric::validate_and_convert_array_identity_constructor_variant::<alphanumeric_array_length>(subtag.get_unchecked_range_safe(NumberOfDigits .. ))?;
		Self::parse_from_variant_4_to_8_common::<_, alphanumeric_array_length>(subtags, language, script, region, alphanumeric_array, |alphanumeric_array| Variant::DigitAlphanumeric3(first_digit, alphanumeric_array))
	}
	
	#[inline(always)]
	fn parse_from_variant_5<'a>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		use RegularGrandfathered::*;
		
		const length: usize = 5;
		const zh: Language = Language::from(b"zh");
		const guoyu: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"guoyu");
		const hakka: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"hakka");
		const xiang: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"xiang");
		
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_variant_5_to_7::<_, _, length, zh>(subtag, subtags, language, script, region, Variant::Alphanumeric5, |validated_subtag| match validated_subtag
		{
			guoyu => Some(zh_guoyu),
			
			hakka => Some(zh_hakka),
			
			xiang => Some(zh_xiang),
			
			_ => None,
		})
	}
	
	#[inline(always)]
	fn parse_from_variant_6<'a>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		use RegularGrandfathered::art_lojban;
		
		const length: usize = 6;
		const art: Language = Language::from(b"art");
		const lojban: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"lojban");
		
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_variant_5_to_7::<_, _, length, art>(subtag, subtags, language, script, region, Variant::Alphanumeric6, |validated_subtag| match validated_subtag
		{
			lojban => Some(art_lojban),
			
			_ => None,
		})
	}
	
	#[inline(always)]
	fn parse_from_variant_7<'a>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		use RegularGrandfathered::cel_gaulish;
		
		const length: usize = 7;
		const cel: Language = Language::from(b"cel");
		const gaulish: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"gaulish");
		
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_variant_5_to_7::<_, _, length, cel>(subtag, subtags, language, script, region, Variant::Alphanumeric7, |validated_subtag| match validated_subtag
		{
			gaulish => Some(cel_gaulish),
			
			_ => None,
		})
	}
	
	#[inline(always)]
	fn parse_from_variant_8<'a>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		const length: usize = 8;
		
		debug_assert_eq!(subtag.len(), length);
		let validated_subtag = Alphanumeric::validate_and_convert_array_identity_constructor_variant::<length>(subtag)?;
		Self::parse_from_variant_4_to_8_common::<_, length>(subtags, language, script, region, validated_subtag, Variant::Alphanumeric8)
	}

	#[inline(always)]
	fn parse_from_variant_5_to_7<'a, IsGrandfatheredRegularSubtag: FnOnce([Alphanumeric; length]) -> Option<RegularGrandfathered>, Constructor: FnOnce([Alphanumeric; length]) -> Variant, const length: usize, const GrandfatheredRegularLanguage: Language>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, constructor: Constructor, is_grandfathered_regular_subtag: IsGrandfatheredRegularSubtag) -> Result<Self, Bcp47LanguageTagParseError>
	{
		debug_assert_eq!(subtag.len(), length);
		let validated_subtag = Alphanumeric::validate_and_convert_array_identity_constructor_variant::<length>(subtag)?;
		if subtags.is_empty() && language == GrandfatheredRegularLanguage
		{
			if let Some(regular_grandfathered) = is_grandfathered_regular_subtag(validated_subtag)
			{
				return Self::from_regular_granfathered_ok(regular_grandfathered)
			}
		}
		Self::parse_from_variant_4_to_8_common::<_, length>(subtags, language, script, region, validated_subtag, constructor)
	}
	
	#[inline(always)]
	fn parse_from_variant_4_to_8_common<Constructor: FnOnce([Alphanumeric; length]) -> Variant, const length: usize>(subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, alphanumeric_array: [Alphanumeric; length], constructor: Constructor) -> Result<Self, Bcp47LanguageTagParseError>
	{
		let mut variants = HashSet::with_capacity(1);
		variants.insert(constructor(alphanumeric_array));
		Self::parse_remaining_variants(variants, subtags, language, script, region)
	}

	#[inline(always)]
	fn parse_remaining_variants(mut variants: HashSet<Variant>, mut subtags: MemchrIterator<Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		loop
		{
			let subtag = match subtags.next()
			{
				None => break,
				
				Some(subtag) => subtag,
			};
			
			use Variant::*;
			use VariantParseError::*;
			
			let variant = match subtag.len()
			{
				0 => return_error!(VariantOfZero),
				
				1 => match subtag.get_unchecked_value_safe(0)
				{
					X | x => return Self::parse_from_private_use(subtags, language, script, region, variants, Default::default()),
					
					validated_extension_code @ (_0 ..= _9 | A ..= W | Y ..= Z | a ..= w | y ..= z) => return Self::parse_from_extension(validated_extension_code, subtags, language, script, region, variants),
					
					_ => return_error!(InvalidExtensionSingleton(extension))
				},
				
				2 => return_error!(VariantOfTwo),
				
				3 => return_error!(VariantOfThree),
				
				4 => parse_digit_alphanumeric_variant!(subtag),
				
				5 => parse_alphanumeric_variant!(subtag, 5, Alphanumeric5),
				
				6 => parse_alphanumeric_variant!(subtag, 6, Alphanumeric6),
				
				7 => parse_alphanumeric_variant!(subtag, 7, Alphanumeric7),
				
				8 => parse_alphanumeric_variant!(subtag, 8, Alphanumeric8),
				
				length @ _ => return_error!(VariantMoreThanEight { length })
			};
			
			let inserted = variants.insert(variant);
			if !inserted
			{
				return_error!(DuplicateVariant(variant))
			}
		}
		
		Self::from_variants_ok(language, script, region, variants)
	}

	/// Known extensions are:-
	/// * T (Transformed Content, RFC 6497).
	/// 	* Has an ordered BCP 47 compliant language tag less extensions, private use and presumably the historic 'i-' irregular grandfathered tags with optional additional suffixed tags describing the transformation.
	/// * U (Common Locale Data Repository (CLDR) RFC 6067).
	/// 	* Is about encode locale information (timezones, calendars, date time formatting, currency codes, line break handling, measurement systems, etc).
	/// 	* See also <https://cldr.unicode.org/index/bcp47-extension>.
	/// 	* Provides a way to encode 'LDML' attributes, such as collation order.
	/// 	* See <https://unicode-org.github.io/cldr/ldml/tr35.html#Key_And_Type_Definitions_> for a historic list of keys.
	/// Known extensions require a full blown parser in their own right.
	#[inline(always)]
	fn parse_from_extension(validated_extension_code: u8, subtags: MemchrIterator<Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>) -> Result<Bcp47LanguageTag, Bcp47LanguageTagParseError>
	{
		use ExtensionParseError::*;
		use ExtensionPortion::*;
		
		#[inline(always)]
		fn extension_entry(extensions: &mut HashMap<Singleton, Extension>, validated_extension_code: u8, error: impl FnOnce(OccupiedEntry<Singleton, Extension>) -> ExtensionParseError) -> Result<VacantEntry<Singleton, Extension>, ExtensionParseError>
		{
			use Entry::*;
			match extensions.entry(Singleton::construct(validated_extension_code))
			{
				Vacant(vacant) => Ok(vacant),
				
				Occupied(occupied) => Err(error(occupied)),
			}
		}
		
		#[inline(always)]
		fn extension_entry_first(extensions: &mut HashMap<Singleton, Extension>, validated_extension_code: u8) -> Result<VacantEntry<Singleton, Extension>, ExtensionParseError>
		{
			extension_entry(extensions, validated_extension_code, |_| unreachable_code(format_args!("Empty HashMap")))
		}
		
		#[inline(always)]
		fn extension_entry_subsequent(extensions: &mut HashMap<Singleton, Extension>, validated_extension_code: u8) -> Result<VacantEntry<Singleton, Extension>, ExtensionParseError>
		{
			extension_entry(extensions, validated_extension_code, |_| DuplicateExtension(*occupied.key()))
		}
		
		#[inline(always)]
		fn mandatory_first_subtag(subtags: &mut MemchrIterator<Hyphen>) -> Result<Extension, Bcp47LanguageTagParseError>
		{
			let mandatory_first_subtag = next_or_error!(subtags, FirstSubtagIsMandatory);
			let one = match mandatory_first_subtag.len()
			{
				0 => return_error!(SubtagOfZero),
				
				1 => return_error!(FirstSubtagIsOne),
				
				2 => parse_alphanumeric_variant!(mandatory_first_subtag, 2, Alphanumeric2),
				
				3 => parse_alphanumeric_variant!(mandatory_first_subtag, 3, Alphanumeric3),
				
				4 => parse_alphanumeric_variant!(mandatory_first_subtag, 4, Alphanumeric4),
				
				5 => parse_alphanumeric_variant!(mandatory_first_subtag, 5, Alphanumeric5),
				
				6 => parse_alphanumeric_variant!(mandatory_first_subtag, 6, Alphanumeric6),
				
				7 => parse_alphanumeric_variant!(mandatory_first_subtag, 7, Alphanumeric7),
				
				8 => parse_alphanumeric_variant!(mandatory_first_subtag, 8, Alphanumeric8),
				
				length @ _ => return_error!(SubtagMoreThanEight { length })
			};
			extension_entry.insert(Extension::new(one))
		}
		
		let mut extensions = HashMap::new();
		
		let mut extension_entry = extension_entry_first(&mut extensions, validated_extension_code)?;
		loop
		{
			let extension = extension_entry.insert(mandatory_first_subtag(&mut subtags)?);
			
			extension_entry = loop
			{
				let subsequent_subtag = return_next!(subtags, Self::from_extensions_ok(language, script, region, variants, extensions));
				
				extension.push
				(
					match subsequent_subtag.len()
					{
						0 => return_error!(SubtagOfZero),
						
						1 => match subtag.get_unchecked_value_safe(0)
						{
							X | x => return Self::parse_from_private_use(subtags, language, script, region, variants, extensions),
							
							validated_extension_code @ (_0 ..= _9 | A ..= W | Y ..= Z | a ..= w | y ..= z) => break extension_entry_subsequent(&mut extensions, extension_code)?,
							
							_ => return_error!(InvalidExtensionSingleton(extension))
						},
						
						2 => parse_alphanumeric_variant!(subtag, 2, Alphanumeric2),
						
						3 => parse_alphanumeric_variant!(subtag, 3, Alphanumeric3),
						
						4 => parse_alphanumeric_variant!(subtag, 4, Alphanumeric4),
						
						5 => parse_alphanumeric_variant!(subtag, 5, Alphanumeric5),
						
						6 => parse_alphanumeric_variant!(subtag, 6, Alphanumeric6),
						
						7 => parse_alphanumeric_variant!(subtag, 7, Alphanumeric7),
						
						8 => parse_alphanumeric_variant!(subtag, 8, Alphanumeric8),
						
						length @ _ => return_error!(SubtagMoreThanEight { length })
					}
				)
			}
		}
	}

	#[inline(always)]
	fn parse_from_private_use(subtags: MemchrIterator<Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>, extensions: HashMap<Singleton, Extension>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		Self::from_private_use_ok(language, script, region, variants, extensions, Some(PrivateUse::parse(subtags)?))
	}
	
	#[inline(always)]
	fn parse_x_or_i_subtag<'a>(first_subtag: &'a [u8], subtags: MemchrIterator<'a, Hyphen>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		match to_lower_case(first_subtag.get_unchecked_value_safe(0usize))
		{
			i => Ok(Self::irregular_grandfathered(IrregularGrandfathered::parse_irregular_i(subtags)?)),
			
			x => Ok(PrivateUse::parse(subtags)?.into()),
			
			byte @ _ => return_error!(LanguageFirstSubtagParseError::FirstSubtagLengthIsOneAndIsNotPrivateUseOrGrandfatheredIrregularTag(byte)),
		}
	}
	
	#[inline(always)]
	const fn regular_grandfathered(regular_grandfathered: RegularGrandfathered) -> Self
	{
		Grandfathered::Regular(regular_grandfathered).into()
	}
	
	#[inline(always)]
	const fn irregular_grandfathered(irregular_grandfathered: IrregularGrandfathered) -> Self
	{
		Grandfathered::Irregular(irregular_grandfathered).into()
	}
	
	#[inline(always)]
	fn from_regular_granfathered_ok(regular_grandfathered: RegularGrandfathered) -> Result<Self, Bcp47LanguageTagParseError>
	{
		Ok(Self::regular_grandfathered(regular_grandfathered))
	}
	
	#[inline(always)]
	fn from_irregular_granfathered_ok(irregular_grandfathered: IrregularGrandfathered) -> Result<Self, Bcp47LanguageTagParseError>
	{
		Ok(Self::irregular_grandfathered(irregular_grandfathered))
	}
	
	#[inline(always)]
	fn from_language_ok(language: Language) -> Result<Self, Bcp47LanguageTagParseError>
	{
		Self::from_script_ok(language, None)
	}
	
	#[inline(always)]
	fn from_script_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		Self::from_region_ok(language, script, None)
	}
	
	#[inline(always)]
	fn from_region_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		Self::from_variants_ok(language, script, region, HashSet::default())
	}
	
	#[inline(always)]
	fn from_variants_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		Self::from_extensions_ok(language, script, region, variants, HashMap::default())
	}
	
	#[inline(always)]
	const fn from_extensions_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>, extensions: HashMap<Singleton, Extension>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		Self::from_private_use_ok(language, script, region, variants, extensions, None)
	}
	
	#[inline(always)]
	const fn from_private_use_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>, extensions: HashMap<Singleton, Extension>, private_use: Option<PrivateUse>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		let normal = Normal
		{
			language,
			script,
			region,
			variants,
			extensions,
			private_use,
		};
		Ok(normal.into())
	}
}
