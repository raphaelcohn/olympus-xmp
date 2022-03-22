// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// As defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt).
/// And RFC 5646.
/// <https://en.wikipedia.org/wiki/IETF_language_tag> is helpful.
/// <https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry> is official.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IetfBcp47LanguageTag
{
	#[allow(missing_docs)]
	Normal(Normal),
	
	#[allow(missing_docs)]
	PrivateUse(PrivateUse),

	#[allow(missing_docs)]
	Grandfathered(Grandfathered),
}

impl const From<Normal> for IetfBcp47LanguageTag
{
	#[inline(always)]
	fn from(normal: Normal) -> Self
	{
		Self::Normal(normal)
	}
}

impl const From<Grandfathered> for IetfBcp47LanguageTag
{
	#[inline(always)]
	fn from(grandfathered: Grandfathered) -> Self
	{
		Self::Grandfathered(grandfathered)
	}
}

impl const From<PrivateUse> for IetfBcp47LanguageTag
{
	#[inline(always)]
	fn from(private_use: PrivateUse) -> Self
	{
		Self::PrivateUse(private_use)
	}
}

impl IetfBcp47LanguageTag
{
	#[allow(missing_docs)]
	pub fn parse(language_tag: &str) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		let subtags = MemchrIterator::from_str(language_tag);
		Self::parse_from_language(subtags)
	}
	
	#[inline(always)]
	fn parse_from_language(mut subtags: Subtags) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		let (language, next_subtag) =
		{
			use LanguageSubtagParseError::*;
			
			let subtag = subtags.next_first();
			match_subtag_length!
			{
				subtag,
				
				return Self::parse_x_or_i_subtag(subtag, subtags),
				
				parse_ordinary_language_subtag!(subtag, subtags, parse_2),
				
				parse_ordinary_language_subtag!(subtag, subtags, parse_3),
				
				parse_reserved_language_subtag!(subtag),
				
				parse_registered_language_subtag!(subtag, 5, Alpha5),
				
				parse_registered_language_subtag!(subtag, 6, Alpha6),
				
				parse_registered_language_subtag!(subtag, 7, Alpha7),
				
				parse_registered_language_subtag!(subtag, 8, Alpha8)
			}
		};
		
		use NextSubtagAfterLanguageExtension::*;
		match next_subtag
		{
			Exhausted => Self::from_language_ok(language),
			
			Next(subtag) => Self::parse_from_script(subtag, subtags, language),
			
			IanaRegisteredUnM49RegionCode(iana_registered_un_m49_region_code) => Self::parse_from_variant(subtags, language, None, Some(IanaRegisteredRegionCode::IanaRegisteredUnM49RegionCode(iana_registered_un_m49_region_code))),
			
			Pending => match subtags.next()
			{
				None => Self::from_language_ok(language),
				
				Some(subtag) => Self::parse_from_script(subtag, subtags, language),
			},
		}
	}

	#[inline(always)]
	fn parse_from_script<'a>(subtag: &'a [u8], subtags: Subtags<'a>, language: Language) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		use ScriptParseError::*;
		
		match_subtag_length!
		{
			subtag,
			validated_extension_code,
			
			Self::parse_from_private_use(subtags, language, None, None, Default::default(), Default::default()),
			
			Self::parse_from_extension(validated_extension_code, subtags, language, None, None, Default::default()),
			
			Self::parse_from_region_2(subtag, subtags, language, None),
			
			Self::parse_from_region_3(subtag, subtags, language, None),
			
			match subtag.byte_0()
			{
				digit @ _0 ..= _9 => Self::parse_from_variant_4(subtag, subtags, language, None, None, Digit::construct(digit)),
				
				alpha @ (A ..= Z | a ..= z) =>
				{
					let initial = UpperCaseAlpha::construct(to_upper_case(alpha));
					let script = Alpha::validate_and_convert_array(subtag.slice_less_first_byte(), |alpha_array| IanaRegisteredIso15924ScriptCode(initial, alpha_array), InvalidAlpha)?;
					Self::parse_from_region(subtags, language, Some(script))
				}
				
				byte @ _ => return_error!(InvalidAlphanumeric(Alphanumeric::error::<4>(0, byte))),
			},
			
			Self::parse_from_variant_5(subtag, subtags, language, None, None),
			
			Self::parse_from_variant_6(subtag, subtags, language, None, None),
			
			Self::parse_from_variant_7(subtag, subtags, language, None, None),
			
			Self::parse_from_variant_8(subtag, subtags, language, None, None)
		}
	}
	
	#[inline(always)]
	fn parse_from_region_2<'a>(subtag: &'a [u8], subtags: Subtags<'a>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		const length: usize = 2;
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_region_n::<UpperCaseAlpha, IanaRegisteredIso3166Dash1Alpha2CountryCode, _, _, length>(subtag, subtags, language, script, IanaRegisteredIso3166Dash1Alpha2CountryCode, RegionParseError::InvalidUpperCaseAlpha)
	}
	
	#[inline(always)]
	fn parse_from_region_3<'a>(subtag: &'a [u8], subtags: Subtags<'a>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		const length: usize = 3;
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_region_n::<Digit, IanaRegisteredUnM49RegionCode, _, _, length>(subtag, subtags, language, script, IanaRegisteredUnM49RegionCode, RegionParseError::InvalidDigit)
	}
	
	#[inline(always)]
	fn parse_from_region_n<'a, RB: RestrictedByte, F: Into<IanaRegisteredRegionCode>, OkConstructor: FnOnce([RB; length]) -> F, ErrorConstructor: Copy + FnOnce(RB::Error) -> RegionParseError, const length: usize>(subtag: &'a [u8], subtags: Subtags<'a>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, constructor: OkConstructor, error: ErrorConstructor) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		debug_assert_eq!(subtag.len(), length);
		let code = RB::validate_and_convert_array::<_, _, _, _, length>(subtag, constructor, error)?;
		Self::parse_from_variant(subtags, language, script, Some(code.into()))
	}
	
	#[inline(always)]
	fn parse_from_region(mut subtags: Subtags, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		use RegionParseError::*;
		
		let subtag = return_next!(subtags, Self::from_script_ok(language, script));
		
		#[inline(always)]
		fn parse_region<RB: RestrictedByte, Error: Copy + FnOnce(RB::Error) -> RegionParseError, const length: usize>(subtags: Subtags, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, subtag: &[u8], error: Error) -> Result<IetfBcp47LanguageTag, IetfBcp47LanguageTagParseError>
		where IanaRegisteredRegionCode: From<[RB; length]>
		{
			let region = RB::validate_and_convert_array::<_, _, _, _, length>(subtag, |array| IanaRegisteredRegionCode::from(array), error)?;
			IetfBcp47LanguageTag::parse_from_variant(subtags, language, script, Some(region))
		}
		
		match_subtag_length!
		{
			subtag,
			validated_extension_code,
			
			Self::parse_from_private_use(subtags, language, script, None, Default::default(), Default::default()),
			
			Self::parse_from_extension(validated_extension_code, subtags, language, script, None, Default::default()),
			
			parse_region::<UpperCaseAlpha, _, 2>(subtags, language, script, subtag, InvalidUpperCaseAlpha),
			
			parse_region::<Digit, _, 3>(subtags, language, script, subtag, InvalidDigit),
			
			match subtag.byte_0()
			{
				digit @ _0 ..= _9 => Self::parse_from_variant_4(subtag, subtags, language, script, None, Digit::construct(digit)),
				
				byte @ _ => return_error!(InvalidAlphanumeric(Alphanumeric::error::<4>(0, byte))),
			},
			
			Self::parse_from_variant_5(subtag, subtags, language, script, None),
			
			Self::parse_from_variant_6(subtag, subtags, language, script, None),
			
			Self::parse_from_variant_7(subtag, subtags, language, script, None),
			
			Self::parse_from_variant_8(subtag, subtags, language, script, None)
		}
		
	}
	
	#[inline(always)]
	fn parse_from_variant(mut subtags: Subtags, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		use VariantParseError::*;
		
		let subtag = return_next!(subtags, Self::from_region_ok(language, script, region));
		
		match_subtag_length!
		{
			subtag,
			validated_extension_code,
			
			Self::parse_from_private_use(subtags, language, script, region, Default::default(), Default::default()),
			
			Self::parse_from_extension(validated_extension_code, subtags, language, script, region, Default::default()),
			
			Self::parse_from_variant_irregular_grandfathered_second_region(subtags, subtag, language, region),
			
			Self::parse_from_variant_irregular_grandfathered_oed(subtags, subtag, language, region),
			
			match subtag.byte_0()
			{
				digit @ _0 ..= _9 => Self::parse_from_variant_4(subtag, subtags, language, None, None, Digit::construct(digit)),
				
				byte @ _ => return_error!(InvalidDigit(Digit::error::<4>(0, byte))),
			},
			
			Self::parse_from_variant_5(subtag, subtags, language, script, region),
			
			Self::parse_from_variant_6(subtag, subtags, language, script, region),
			
			Self::parse_from_variant_7(subtag, subtags, language, script, region),
			
			Self::parse_from_variant_8(subtag, subtags, language, script, region)
		}
	}
	
	#[inline(always)]
	fn parse_from_variant_irregular_grandfathered_second_region(subtags: Subtags, subtag: &[u8], language: Language, region: Option<IanaRegisteredRegionCode>) -> Result<IetfBcp47LanguageTag, IetfBcp47LanguageTagParseError>
	{
		use IrregularGrandfathered::*;
		
		const SecondRegionLength: usize = 2;
		const sgn: Language = Language::from(b"sgn");
		const BE: Option<IanaRegisteredRegionCode> = Some(IanaRegisteredRegionCode::from(b"BE"));
		const CH: Option<IanaRegisteredRegionCode> = Some(IanaRegisteredRegionCode::from(b"CH"));
		const FR: [UpperCaseAlpha; SecondRegionLength] = UpperCaseAlpha::new_array_unchecked(b"FR");
		const NL: [UpperCaseAlpha; SecondRegionLength] = UpperCaseAlpha::new_array_unchecked(b"NL");
		const DE: [UpperCaseAlpha; SecondRegionLength] = UpperCaseAlpha::new_array_unchecked(b"DE");
		
		if subtags.is_empty() && language == sgn
		{
			#[inline(always)]
			fn second_region(subtag: &[u8]) -> Result<[UpperCaseAlpha; SecondRegionLength], VariantParseError>
			{
				UpperCaseAlpha::validate_and_convert_array::<_, _, _, _, SecondRegionLength>(subtag, |upper_case_alpha_array| upper_case_alpha_array, VariantParseError::InvalidUpperCaseAlpha)
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
		Err(IetfBcp47LanguageTagParseError::from(VariantParseError::IsTwo))
	}
	
	#[inline(always)]
	fn parse_from_variant_irregular_grandfathered_oed(subtags: Subtags, subtag: &[u8], language: Language, region: Option<IanaRegisteredRegionCode>) -> Result<IetfBcp47LanguageTag, IetfBcp47LanguageTagParseError>
	{
		use IrregularGrandfathered::en_GB_oed;
		
		const OedLength: usize = 3;
		const en: Language = Language::from(b"en");
		const GB: Option<IanaRegisteredRegionCode> = Some(IanaRegisteredRegionCode::from(b"GB"));
		const oed: [Alpha; OedLength] = Alpha::new_array_unchecked(b"oed");
		
		if subtags.is_empty() && language == en && region == GB
		{
			let possibly_oed = Alpha::validate_and_convert_array::<_, _, _, _, OedLength>(subtag, |upper_case_alpha_array| upper_case_alpha_array, VariantParseError::InvalidAlpha)?;
			if possibly_oed == oed
			{
				return Self::from_irregular_granfathered_ok(en_GB_oed)
			}
		}
		Err(IetfBcp47LanguageTagParseError::from(VariantParseError::IsThree))
	}
	
	#[inline(always)]
	fn parse_from_variant_4<'a>(subtag: &'a [u8], subtags: Subtags<'a>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, first_digit: Digit) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		const length: usize = 4;
		const NumberOfDigits: usize = 1;
		const alphanumeric_array_length: usize = length - NumberOfDigits;
		debug_assert_eq!(subtag.len(), length);
		
		let alphanumeric_array = Alphanumeric::validate_and_convert_array_identity_constructor_variant::<alphanumeric_array_length>(subtag.get_unchecked_range_safe(NumberOfDigits .. ))?;
		Self::parse_from_variant_4_to_8_common::<_, alphanumeric_array_length>(subtags, language, script, region, alphanumeric_array, |alphanumeric_array| Variant::DigitAlphanumeric3(first_digit, alphanumeric_array))
	}
	
	#[inline(always)]
	fn parse_from_variant_5<'a>(subtag: &'a [u8], subtags: Subtags<'a>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		use RegularGrandfathered::*;
		
		const length: usize = 5;
		const zh: Language = Language::from(b"zh");
		const guoyu: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"guoyu");
		const hakka: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"hakka");
		const xiang: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"xiang");
		
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_variant_5_to_7::<_, _, length>(zh, subtag, subtags, language, script, region, Variant::Alphanumeric5, |validated_subtag| match validated_subtag
		{
			guoyu => Some(zh_guoyu),
			
			hakka => Some(zh_hakka),
			
			xiang => Some(zh_xiang),
			
			_ => None,
		})
	}
	
	#[inline(always)]
	fn parse_from_variant_6<'a>(subtag: &'a [u8], subtags: Subtags<'a>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		use RegularGrandfathered::art_lojban;
		
		const length: usize = 6;
		const art: Language = Language::from(b"art");
		const lojban: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"lojban");
		
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_variant_5_to_7::<_, _, length>(art, subtag, subtags, language, script, region, Variant::Alphanumeric6, |validated_subtag| match validated_subtag
		{
			lojban => Some(art_lojban),
			
			_ => None,
		})
	}
	
	#[inline(always)]
	fn parse_from_variant_7<'a>(subtag: &'a [u8], subtags: Subtags<'a>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		use RegularGrandfathered::cel_gaulish;
		
		const length: usize = 7;
		const cel: Language = Language::from(b"cel");
		const gaulish: [Alphanumeric; length] = Alphanumeric::new_array_unchecked(b"gaulish");
		
		debug_assert_eq!(subtag.len(), length);
		Self::parse_from_variant_5_to_7::<_, _, length>(cel, subtag, subtags, language, script, region, Variant::Alphanumeric7,  |validated_subtag| match validated_subtag
		{
			gaulish => Some(cel_gaulish),
			
			_ => None,
		})
	}
	
	#[inline(always)]
	fn parse_from_variant_8<'a>(subtag: &'a [u8], subtags: Subtags<'a>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		const length: usize = 8;
		
		debug_assert_eq!(subtag.len(), length);
		let validated_subtag = Alphanumeric::validate_and_convert_array_identity_constructor_variant::<length>(subtag)?;
		Self::parse_from_variant_4_to_8_common::<_, length>(subtags, language, script, region, validated_subtag, Variant::Alphanumeric8)
	}

	#[inline(always)]
	fn parse_from_variant_5_to_7<'a, IsGrandfatheredRegularSubtag: FnOnce([Alphanumeric; length]) -> Option<RegularGrandfathered>, Constructor: FnOnce([Alphanumeric; length]) -> Variant, const length: usize>(GrandfatheredRegularLanguage: Language, subtag: &'a [u8], subtags: Subtags<'a>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, constructor: Constructor, is_grandfathered_regular_subtag: IsGrandfatheredRegularSubtag) -> Result<Self, IetfBcp47LanguageTagParseError>
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
	fn parse_from_variant_4_to_8_common<Constructor: FnOnce([Alphanumeric; length]) -> Variant, const length: usize>(subtags: Subtags, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, alphanumeric_array: [Alphanumeric; length], constructor: Constructor) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		let mut variants = HashSet::with_capacity(1);
		let _ = variants.insert(constructor(alphanumeric_array));
		Self::parse_remaining_variants(variants, subtags, language, script, region)
	}

	#[inline(always)]
	fn parse_remaining_variants(mut variants: HashSet<Variant>, mut subtags: Subtags, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
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
			
			let variant = match_subtag_length!
			{
				subtag,
				validated_extension_code,
				
				Self::parse_from_private_use(subtags, language, script, region, variants, Default::default()),
				
				return Self::parse_from_extension(validated_extension_code, subtags, language, script, region, variants),
				
				return_error!(IsTwo),
				
				return_error!(IsThree),
				
				{
					const length: usize = 4;
					const length_alphanumeric: usize = length - 1;
					let digit = Digit::construct(Digit::validate_and_convert_byte::<_, _, length>(subtag, InvalidDigit, 0)?);
					Alphanumeric::validate_and_convert_array::<_, _, _, _, length_alphanumeric>(subtag.slice_less_first_byte(), |alphanumeric_array| DigitAlphanumeric3(digit, alphanumeric_array), InvalidAlphanumeric)?
				},
				
				parse_alphanumeric_variant!(subtag, 5, Alphanumeric5),
				
				parse_alphanumeric_variant!(subtag, 6, Alphanumeric6),
				
				parse_alphanumeric_variant!(subtag, 7, Alphanumeric7),
				
				parse_alphanumeric_variant!(subtag, 8, Alphanumeric8)
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
	fn parse_from_extension(validated_extension_code: u8, mut subtags: Subtags, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>) -> Result<IetfBcp47LanguageTag, IetfBcp47LanguageTagParseError>
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
			extension_entry(extensions, validated_extension_code, |occupied| DuplicateExtension(*occupied.key()))
		}
		
		#[inline(always)]
		fn mandatory_first_subtag(subtags: &mut Subtags) -> Result<Extension, IetfBcp47LanguageTagParseError>
		{
			let mandatory_first_subtag = next_or_error!(subtags, FirstSubtagIsMandatory);
			
			let one = match_subtag_length!
			{
				mandatory_first_subtag,
				
				return_error!(FirstSubtagIsOne),
				
				parse_alphanumeric_variant!(mandatory_first_subtag, 2, Alphanumeric2),
				
				parse_alphanumeric_variant!(mandatory_first_subtag, 3, Alphanumeric3),
				
				parse_alphanumeric_variant!(mandatory_first_subtag, 4, Alphanumeric4),
				
				parse_alphanumeric_variant!(mandatory_first_subtag, 5, Alphanumeric5),
				
				parse_alphanumeric_variant!(mandatory_first_subtag, 6, Alphanumeric6),
				
				parse_alphanumeric_variant!(mandatory_first_subtag, 7, Alphanumeric7),
				
				parse_alphanumeric_variant!(mandatory_first_subtag, 8, Alphanumeric8)
			};
			Ok(Extension::new(one))
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
					match_subtag_length!
					{
						subsequent_subtag,
						validated_extension_code,
						
						Self::parse_from_private_use(subtags, language, script, region, variants, extensions),
						
						break extension_entry_subsequent(&mut extensions, validated_extension_code)?,
						
						parse_alphanumeric_variant!(subsequent_subtag, 2, Alphanumeric2),
						
						parse_alphanumeric_variant!(subsequent_subtag, 3, Alphanumeric3),
						
						parse_alphanumeric_variant!(subsequent_subtag, 4, Alphanumeric4),
						
						parse_alphanumeric_variant!(subsequent_subtag, 5, Alphanumeric5),
						
						parse_alphanumeric_variant!(subsequent_subtag, 6, Alphanumeric6),
						
						parse_alphanumeric_variant!(subsequent_subtag, 7, Alphanumeric7),
						
						parse_alphanumeric_variant!(subsequent_subtag, 8, Alphanumeric8)
					}
				)
			}
		}
	}

	#[inline(always)]
	fn parse_from_private_use(subtags: Subtags, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>, extensions: HashMap<Singleton, Extension>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		Self::from_private_use_ok(language, script, region, variants, extensions, Some(PrivateUse::parse(subtags)?))
	}
	
	#[inline(always)]
	fn parse_x_or_i_subtag<'a>(subtag: &'a [u8], subtags: Subtags<'a>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		match to_lower_case(subtag.byte_0())
		{
			i => Ok(Self::irregular_grandfathered(IrregularGrandfathered::parse_irregular_i(subtags)?)),
			
			x => Ok(PrivateUse::parse(subtags)?.into()),
			
			byte @ _ => return_error!(LanguageSubtagParseError::FirstSubtagLengthIsOneAndIsNotPrivateUseOrGrandfatheredIrregularTag(byte)),
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
	fn from_regular_granfathered_ok(regular_grandfathered: RegularGrandfathered) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		Ok(Self::regular_grandfathered(regular_grandfathered))
	}
	
	#[inline(always)]
	fn from_irregular_granfathered_ok(irregular_grandfathered: IrregularGrandfathered) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		Ok(Self::irregular_grandfathered(irregular_grandfathered))
	}
	
	#[inline(always)]
	fn from_language_ok(language: Language) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		Self::from_script_ok(language, None)
	}
	
	#[inline(always)]
	fn from_script_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		Self::from_region_ok(language, script, None)
	}
	
	#[inline(always)]
	fn from_region_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		Self::from_variants_ok(language, script, region, Default::default())
	}
	
	#[inline(always)]
	fn from_variants_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		Self::from_extensions_ok(language, script, region, variants, Default::default())
	}
	
	#[inline(always)]
	const fn from_extensions_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>, extensions: HashMap<Singleton, Extension>) -> Result<Self, IetfBcp47LanguageTagParseError>
	{
		Self::from_private_use_ok(language, script, region, variants, extensions, None)
	}
	
	#[inline(always)]
	const fn from_private_use_ok(language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>, variants: HashSet<Variant>, extensions: HashMap<Singleton, Extension>, private_use: Option<PrivateUse>) -> Result<Self, IetfBcp47LanguageTagParseError>
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
