// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Hierarchy.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Hierarchy<'a, const PathDepth: usize>
{
	/// If not empty, then starts with `/`.
	AuthorityAndAbsolutePath
	{
		authority: Authority<'a>,
		
		/// Zero or more; each path segment can be empty.
		path_segments: PathSegments<'a, PathDepth>,
	},
	
	/// Starts `/`.
	/// Minimum is `/X`, where `X` is a valid character.
	AbsolutePath(NonEmptyPath<'a, PathDepth>),

	/// Does not start `/`.
	/// Minimum is `X`, where `X` is a valid character.
	RootlessPath(NonEmptyPath<'a, PathDepth>),
	
	/// An empty path.
	EmptyPath,
}

impl<'a, const PathDepth: usize> TryToOwnInPlace for Hierarchy<'a, PathDepth>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		use Hierarchy::*;
		
		match self
		{
			AuthorityAndAbsolutePath { authority, path_segments } =>
			{
				authority.try_to_own_in_place()?;
				path_segments.try_to_own_in_place()
			}
			
			AbsolutePath(non_empty_path) => non_empty_path.try_to_own_in_place(),
			
			RootlessPath(non_empty_path) => non_empty_path.try_to_own_in_place(),
			
			EmptyPath => Ok(())
		}
	}
}

impl<'a, const PathDepth: usize> TryToOwn for Hierarchy<'a, PathDepth>
{
	type TryToOwned = Hierarchy<'static, PathDepth>;
	
	#[inline(always)]
	fn try_to_own(self) -> Result<Self::TryToOwned, TryReserveError>
	{
		use Hierarchy::*;
		
		Ok
		(
			match self
			{
				AuthorityAndAbsolutePath { authority, path_segments } => AuthorityAndAbsolutePath { authority: authority.try_to_own()?, path_segments: path_segments.try_to_own()? },
				
				AbsolutePath(non_empty_path) => AbsolutePath(non_empty_path.try_to_own()?),
				
				RootlessPath(non_empty_path) => RootlessPath(non_empty_path.try_to_own()?),
				
				EmptyPath => EmptyPath
			}
		)
	}
}

impl<'a, const PathDepth: usize> Hierarchy<'a, PathDepth>
{
	/// New instance.
	///
	/// Prefer `Authority.with()` over this method.
	#[inline(always)]
	pub const fn new_authority_and_absolute_path<const M: usize>(authority: Authority<'a>, path_segments: [PathSegment<'a>; M]) -> Self
	{
		authority.with(path_segments)
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn new_absolute_path<const M: usize>(first_non_empty_path_segment: NonEmptyPathSegment<'a>, remaining_path_segments: [PathSegment<'a>; M]) -> Self
	{
		Hierarchy::AbsolutePath(NonEmptyPath::new(first_non_empty_path_segment, remaining_path_segments))
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn new_rootless_path<const M: usize>(first_non_empty_path_segment: NonEmptyPathSegment<'a>, remaining_path_segments: [PathSegment<'a>; M]) -> Self
	{
		Hierarchy::RootlessPath(NonEmptyPath::new(first_non_empty_path_segment, remaining_path_segments))
	}
	
	/// `ihier-part = "//" iauthority ipath-abempty / ipath-absolute / ipath-rootless / ipath-empty`.
	/// `ipath-abempty  = *( "/" isegment )`.
	/// `ipath-absolute = "/" [ isegment-nz *( "/" isegment ) ]`.
	/// `ipath-rootless = isegment-nz *( "/" isegment )`.
	/// `ipath-empty    = 0<ipchar>`.
	/// `isegment       = *ipchar`.
	/// `isegment-nz    = 1*ipchar`.
	#[inline(always)]
	fn parse(mut remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		use Utf8CharacterLength::*;
		use Hierarchy::*;
		use HierarchyParseError::*;
		
		let character = StringSoFar::decode_next_utf8_validity_already_checked_mandatory(&mut remaining_utf8_bytes, DidNotExpectEndParsingFirstCharacter)?;
		match character
		{
			QuestionMarkChar => Ok((EmptyPath, ParseNextAfterHierarchy::query(remaining_utf8_bytes))),
			
			HashChar => Ok((EmptyPath, ParseNextAfterHierarchy::fragment_no_query(remaining_utf8_bytes))),
			
			SlashChar => Self::parse_iauthority_ipath_abempty_or_ipath_absolute(remaining_utf8_bytes),
			
			ipchar_iunreserved_without_ucschar!() => Self::parse_ipath_rootless(Self::decoded(character, One), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_2!()  => Self::parse_ipath_rootless(Self::decoded(character, Two), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_3!()  => Self::parse_ipath_rootless(Self::decoded(character, Three), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_4!()  => Self::parse_ipath_rootless(Self::decoded(character, Four), remaining_utf8_bytes),
			ipchar_pct_encoded!()                 => Self::parse_ipath_rootless(Self::decode_percent_encoded(&mut remaining_utf8_bytes, InvalidPercentEncodedUtf8ParseFirstCharacter)?, remaining_utf8_bytes),
			ipchar_sub_delims!()                  => Self::parse_ipath_rootless(Self::decoded(character, One), remaining_utf8_bytes),
			ipchar_other!()                       => Self::parse_ipath_rootless(Self::decoded(character, One), remaining_utf8_bytes),
			
			_ => Err(InvalidFirstCharacter(character)),
		}
	}
	
	#[inline(always)]
	fn parse_iauthority_ipath_abempty_or_ipath_absolute(mut remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		use Utf8CharacterLength::*;
		use HierarchyParseError::*;
		
		let character = StringSoFar::decode_next_utf8_validity_already_checked_mandatory(&mut remaining_utf8_bytes, DidNotExpectEndParsingSecondCharacter)?;
		match character
		{
			SlashChar => Self::parse_iauthority_ipath_abempty(remaining_utf8_bytes),
			
			ipchar_iunreserved_without_ucschar!() => Self::parse_ipath_absolute(Self::decoded(character, One), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_2!()  => Self::parse_ipath_absolute(Self::decoded(character, Two), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_3!()  => Self::parse_ipath_absolute(Self::decoded(character, Three), remaining_utf8_bytes),
			ipchar_iunreserved_with_ucschar_4!()  => Self::parse_ipath_absolute(Self::decoded(character, Four), remaining_utf8_bytes),
			ipchar_pct_encoded!()                 => Self::parse_ipath_absolute(Self::decode_percent_encoded(&mut remaining_utf8_bytes, InvalidPercentEncodedUtf8ParseSecondCharacter)?, remaining_utf8_bytes),
			ipchar_sub_delims!()                  => Self::parse_ipath_absolute(Self::decoded(character, One), remaining_utf8_bytes),
			ipchar_other!()                       => Self::parse_ipath_absolute(Self::decoded(character, One), remaining_utf8_bytes),
			
			_ => Err(InvalidSecondCharacter(character)),
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
	fn parse_ipath_rootless(first_character_of_first_path_segment: (bool, char, Utf8CharacterLength), remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		Self::parse_non_empty_path(first_character_of_first_path_segment, remaining_utf8_bytes, Hierarchy::RootlessPath, HierarchyParseError::IPathRootlessParse)
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
	fn parse_ipath_absolute(first_character_of_first_path_segment: (bool, char, Utf8CharacterLength), remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		Self::parse_non_empty_path(first_character_of_first_path_segment, remaining_utf8_bytes, Hierarchy::AbsolutePath, HierarchyParseError::IPathAbsoluteParse)
	}
	
	#[inline(always)]
	fn parse_non_empty_path(first_character_of_first_path_segment: (bool, char, Utf8CharacterLength), remaining_utf8_bytes: &'a [u8], constructor: impl FnOnce(NonEmptyPath<'a, PathDepth>) -> Self, error: impl FnOnce(NonEmptyPathParseError) -> HierarchyParseError) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		NonEmptyPath::parse(constructor, error, first_character_of_first_path_segment, remaining_utf8_bytes)
	}
	
	/// `.              = iauthority ipath-abempty`.
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
	fn parse_iauthority_ipath_abempty(remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		let mut path_segments = PathSegments::default();
		
		let (authority, parse_next) = match memchr3(QuestionMark, Hash, Slash, remaining_utf8_bytes)
		{
			None =>
			{
				let authority = Authority::parse(remaining_utf8_bytes)?;
				(authority, ParseNextAfterHierarchy::NoQueryNoFragment)
			}
			
			Some(index) =>
			{
				let authority_bytes = remaining_utf8_bytes.before_index(index);
				let authority = Authority::parse(authority_bytes)?;
				
				let after_authority_bytes = remaining_utf8_bytes.after_index(index);
				
				let parse_next_after_hierarchy = match remaining_utf8_bytes.get_unchecked_value_safe(index)
				{
					// means there's just iauthority followed by an empty path then a query
					QuestionMark => ParseNextAfterHierarchy::query(remaining_utf8_bytes),
					
					// means there's just iauthority followed by an empty path then an empty query then a fragment
					Hash => ParseNextAfterHierarchy::fragment_no_query(after_authority_bytes),
					
					// after_authority_bytes is the start of the absolute path.
					Slash => path_segments.parse(after_authority_bytes)?,
					
					_ => unreachable_code_const("memchr3")
				};
				
				(authority, parse_next_after_hierarchy)
			}
		};
		
		Ok((Hierarchy::AuthorityAndAbsolutePath { authority, path_segments }, parse_next))
	}
	
	#[inline(always)]
	const fn decoded(character: char, utf8_character_length: Utf8CharacterLength) -> (bool, char, Utf8CharacterLength)
	{
		(false, character, utf8_character_length)
	}
	
	#[inline(always)]
	fn decode_percent_encoded(remaining_utf8_bytes: &mut &[u8], error: impl FnOnce(InvalidUtf8ParseError<PercentDecodeError>) -> HierarchyParseError) -> Result<(bool, char, Utf8CharacterLength), HierarchyParseError>
	{
		let (character, utf8_character_length) = decode_next_percent_encoded_utf8(remaining_utf8_bytes).map_err(error)?;
		Ok((true, character, utf8_character_length))
	}
}
