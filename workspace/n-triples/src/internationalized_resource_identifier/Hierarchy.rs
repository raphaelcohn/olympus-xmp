// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Hierarchy.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Hierarchy<'a>
{
	/// If not empty, then starts with `/`.
	AuthorityAndAbsolutePath
	{
		authority: Authority<'a>,
		
		/// Zero or more; each path segment can be empty.
		path_segments: PathSegments<'a>,
	},
	
	/// Starts `/`.
	/// Minimum is `/X`, where `X` is a valid character.
	AbsolutePath(NonEmptyPath<'a>),

	/// Does not start `/`.
	/// Minimum is `X`, where `X` is a valid character.
	RootlessPath(NonEmptyPath<'a>),
	
	/// An empty path.
	EmptyPath,
}

impl<'a> Hierarchy<'a>
{
	/// `ihier-part = "//" iauthority ipath-abempty / ipath-absolute / ipath-rootless / ipath-empty`.
	/// `ipath-abempty  = *( "/" isegment )`.
	/// `ipath-absolute = "/" [ isegment-nz *( "/" isegment ) ]`.
	/// `ipath-rootless = isegment-nz *( "/" isegment )`.
	/// `ipath-empty    = 0<ipchar>`.
	/// `isegment       = *ipchar`.
	/// `isegment-nz    = 1*ipchar`.
	#[inline(always)]
	fn parse(mut remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNext<'a>), HierarchyParseError>
	{
		use Utf8CharacterLength::*;
		use Hierarchy::*;
		use HierarchyParseError::*;
		use ParseNext::*;
		
		match Self::decode_next_utf8_validity_already_checked_mandatory(&mut remaining_utf8_bytes)?
		{
			QuestionMarkChar => Ok((EmptyPath, ParseNext::query(remaining_utf8_bytes))),
			
			HashChar => Ok((EmptyPath, ParseNext::fragment_no_query(remaining_utf8_bytes))),
			
			SlashChar => Self::parse_iauthority_ipath_abempty_or_ipath_absolute(remaining_utf8_bytes),
			
			ipchar_iunreserved_without_ucschar!() => Self::parse_ipath_rootless(Self::decoded(character, One), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_2!()  => Self::parse_ipath_rootless(Self::decoded(character, Two), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_3!()  => Self::parse_ipath_rootless(Self::decoded(character, Three), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_4!()  => Self::parse_ipath_rootless(Self::decoded(character, Four), remaining_utf8_bytes),
			ipchar_pct_encoded!()                 => Self::parse_ipath_rootless(Self::decode_percent_encoded(&mut remaining_utf8_bytes)?, remaining_utf8_bytes),
			ipchar_sub_delims!()                  => Self::parse_ipath_rootless(Self::decoded(character, One), remaining_utf8_bytes),
			ipchar_other!()                       => Self::parse_ipath_rootless(Self::decoded(character, One), remaining_utf8_bytes),
			
			_ => Err(InvalidCharacter(character)),
		}
	}
	
	#[inline(always)]
	fn parse_iauthority_ipath_abempty_or_ipath_absolute(mut remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNext<'a>), HierarchyParseError>
	{
		use Utf8CharacterLength::*;
		use Hierarchy::*;
		use HierarchyParseError::*;
		use ParseNext::*;
		
		match Self::decode_next_utf8_validity_already_checked_mandatory(&mut remaining_utf8_bytes)?
		{
			SlashChar => Self::parse_iauthority_ipath_abempty(remaining_utf8_bytes),
			
			ipchar_iunreserved_without_ucschar!() => Self::parse_ipath_absolute(Self::decoded(character, One), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_2!()  => Self::parse_ipath_absolute(Self::decoded(character, Two), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_3!()  => Self::parse_ipath_absolute(Self::decoded(character, Three), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_4!()  => Self::parse_ipath_absolute(Self::decoded(character, Four), remaining_utf8_bytes),
			ipchar_pct_encoded!()                 => Self::parse_ipath_absolute(Self::decode_percent_encoded(&mut remaining_utf8_bytes)?, remaining_utf8_bytes),
			ipchar_sub_delims!()                  => Self::parse_ipath_absolute(Self::decoded(character, One), remaining_utf8_bytes),
			ipchar_other!()                       => Self::parse_ipath_absolute(Self::decoded(character, One), remaining_utf8_bytes),
			
			_ => Err(InvalidCharacter(character)),
		}
	}
	
	/// First segment can not be empty, and must start `<something>`.
	/// `character` is the first character of the first segment.
	/// `ipath-rootless` examples:-
	/// * "x"
	/// * "x/"
	/// * "x///"
	/// * "x///segment"
	/// * "x///segment//x/"
	#[inline(always)]
	fn parse_ipath_rootless(first_character_of_first_path_segment: (bool, char, Utf8CharacterLength), mut remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNext<'a>), HierarchyParseError>
	{
		NonEmptyPath::parse(Hierarchy::RootlessPath, first_character_of_first_path_segment, remaining_utf8_bytes)
	}
	
	/// First segment can not be empty, and must start `/<something>`.
	/// `character` is the first character of the first segment.
	/// `ipath-absolute` examples:-
	/// * "/x"
	/// * "/x/"
	/// * "/x///"
	/// * "/x/segment"
	/// * "/x/segment///"
	#[inline(always)]
	fn parse_ipath_absolute(first_character_of_first_path_segment: (bool, char, Utf8CharacterLength), mut remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNext<'a>), HierarchyParseError>
	{
		NonEmptyPath::parse(Hierarchy::AbsolutePath, first_character_of_first_path_segment, remaining_utf8_bytes)
	}
	
	/// `.              = iauthority ipath-abempty`.
	/// `iauthority     = [ iuserinfo "@" ] ihost [ ":" port ]`
	/// `iuserinfo      = *( iunreserved / pct-encoded / sub-delims / ":" )`
	/// `ihost          = IP-literal / IPv4address / ireg-name`
	/// `ireg-name      = *( iunreserved / pct-encoded / sub-delims )`
	/// `IP-literal     = "[" ( IPv6address / IPvFuture  ) "]"` ;Exclude as starts with `[`.
	/// `IPv4address    = dec-octet "." dec-octet "." dec-octet "." dec-octet` ;Exclude as starts with a `DIGIT`.
	/// `iunreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~" / ucschar`.
	/// `ipath-abempty  = *( "/" isegment )`.
	/// `isegment       = *ipchar`.
	/// `ipath-abempty` examples:-
	/// * ""
	/// * "/segment"
	/// * "/segment/segment"
	/// * "/"
	/// * "//"
	/// * "//segment"
	/// * "//segment/"
	/// * "//segment////segment"
	#[inline(always)]
	fn parse_iauthority_ipath_abempty(remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNext<'a>), HierarchyParseError>
	{
		use ParseNext::*;
		use Hierarchy::AuthorityAndAbsolutePath;
		
		let mut path_segments = PathSegments::default();
		
		let (authority, parse_next) = match memchr3(QuestionMark, Hash, Slash, remaining_utf8_bytes)
		{
			None =>
			{
				let authority = Authority::parse(remaining_utf8_bytes)?;
				(authority, ParseNext::NoQueryNoFragment)
			}
			
			Some(index) =>
			{
				let authority_bytes = remaining_utf8_bytes.before_index(index);
				let authority = Authority::parse(authority_bytes)?;
				
				let after_authority_bytes = remaining_utf8_bytes.after_index(index);
				
				let parse_next = match remaining_utf8_bytes.get_unchecked_value_safe(index)
				{
					// means there's just iauthority followed by an empty path then a query
					QuestionMark => ParseNext::query(remaining_utf8_bytes),
					
					// means there's just iauthority followed by an empty path then an empty query then a fragment
					Hash => ParseNext::fragment_no_query(after_authority_bytes),
					
					// after_authority_bytes is the start of the absolute path.
					Slash => path_segments.parse(after_authority_bytes)?,
					
					_ => unreachable_code_const("memchr3")
				};
				
				(authority, parse_next)
			}
		};
		
		Ok((AuthorityAndAbsolutePath { authority, path_segments }, parse_next))
	}
	
	#[inline(always)]
	fn decode_next_utf8_validity_already_checked_mandatory(remaining_utf8_bytes: &mut &'a [u8]) -> Result<char, HierarchyParseError>
	{
		Self::decode_next_utf8_validity_already_checked(&mut remaining_utf8_bytes).ok_or(HierarchyParseError::DidNotExpectEndParsingCharacter)
	}
	
	#[inline(always)]
	fn decode_next_utf8_validity_already_checked(remaining_utf8_bytes: &mut &[u8]) -> Option<char>
	{
		decode_next_utf8_validity_already_checked(remaining_utf8_bytes).map(|(character, utf8_character_length)| character)
	}
	
	#[inline(always)]
	const fn decoded(character: char, utf8_character_length: Utf8CharacterLength) -> (bool, char, Utf8CharacterLength)
	{
		(false, character, utf8_character_length)
	}
	
	#[inline(always)]
	fn decode_percent_encoded(remaining_utf8_bytes: &mut &[u8]) -> Result<(bool, char, Utf8CharacterLength), InvalidUtf8ParseError<PercentDecodeError>>
	{
		let (character, utf8_character_length) = decode_next_percent_encoded_utf8(remaining_utf8_bytes)?;
		Ok((true, character, utf8_character_length))
	}
}
