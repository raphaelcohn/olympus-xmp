// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Hierarchy.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Hierarchy<'a, const PathDepth: usize>
{
	/// If not empty, then starts with `/`.
	///
	/// Only valid option for http and https schemes.
	/// One of two valid options (the other being `AbsolutePath`) for file schemes.
	AuthorityAndAbsolutePath(AuthorityAndAbsolutePath<'a, PathDepth>),
	
	/// Starts `/`.
	/// Minimum is `/X`, where `X` is a valid character.
	///
	/// One of two valid options (the other being `AuthorityAndAbsolutePath`) for file schemes.
	AbsolutePath(NonEmptyPath<'a, PathDepth>),

	/// Does not start `/`.
	/// Minimum is `X`, where `X` is a valid character.
	RootlessPath(NonEmptyPath<'a, PathDepth>),
	
	/// An empty path.
	EmptyPath,
}

impl<'a, const PathDepth: usize> Display for Hierarchy<'a, PathDepth>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		use Hierarchy::*;
		match self
		{
			AuthorityAndAbsolutePath(authority_and_absolute_path) => write!(f, "//{}", authority_and_absolute_path),
			
			AbsolutePath(non_empty_path) => write!(f, "/{}", non_empty_path),
			
			RootlessPath(non_empty_path) => write!(f, "{}", non_empty_path),
			
			EmptyPath => Ok(()),
		}
	}
}

impl<'a, const PathDepth: usize> TryToOwnInPlace for Hierarchy<'a, PathDepth>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		use Hierarchy::*;
		
		match self
		{
			AuthorityAndAbsolutePath(authority_and_absolute_path) => authority_and_absolute_path.try_to_own_in_place(),
			
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
				AuthorityAndAbsolutePath(authority_and_absolute_path) => AuthorityAndAbsolutePath(authority_and_absolute_path.try_to_own()?),
				
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
	pub const fn new_authority_and_absolute_path(authority: Authority<'a>, path_segments: PathSegments<'a, PathDepth>) -> Self
	{
		authority.with(path_segments)
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn new_absolute_path(first_non_empty_path_segment: NonEmptyPathSegment<'a>, remaining_path_segments: PathSegments<'a, PathDepth>) -> Self
	{
		Hierarchy::AbsolutePath(NonEmptyPath::new(first_non_empty_path_segment, remaining_path_segments))
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn new_rootless_path(first_non_empty_path_segment: NonEmptyPathSegment<'a>, remaining_path_segments: PathSegments<'a, PathDepth>) -> Self
	{
		Hierarchy::RootlessPath(NonEmptyPath::new(first_non_empty_path_segment, remaining_path_segments))
	}
	
	/// Appends a path segment.
	///
	/// If the hierarchy is `Hierarchy::EmptyPath`, it is converted according to the argument `convert_empty_path_to_absolute`:-
	///
	/// * If `true`, empty path becomes an absolute path.
	/// * If `false`, empty path becomes a rootless path.
	///
	/// Failure:-
	/// * Can fail with an `Err()` if there is not enough heap memory.
	/// * If the `path_segment` is empty and the hierarchy is `Hierarchy::EmptyPath`.
	#[inline(always)]
	pub fn with_path_segment<const convert_empty_path_to_absolute: bool>(&mut self, path_segment: PathSegment<'a>) -> Result<(), WithPathSegmentError>
	{
		use Hierarchy::*;
		match self
		{
			AuthorityAndAbsolutePath(authority_and_absolute_path) => authority_and_absolute_path.with_path_segment(path_segment)?,
			
			AbsolutePath(non_empty_path) => non_empty_path.with_path_segment(path_segment)?,
			
			RootlessPath(non_empty_path) => non_empty_path.with_path_segment(path_segment)?,
			
			EmptyPath =>
			{
				let first_non_empty_path_segment = NonEmptyPathSegment::try_from(path_segment).map_err(|_| WithPathSegmentError::HierarchyIsEmptyPathAndPathSegmentIsEmpty)?;
				let non_empty_path = NonEmptyPath::new_minimal(first_non_empty_path_segment);
				
				*self = if convert_empty_path_to_absolute
				{
					AbsolutePath(non_empty_path)
				}
				else
				{
					RootlessPath(non_empty_path)
				};
			}
		}
		
		Ok(())
	}
	
	/// Appends a path segment if there is space on the stack.
	///
	/// If the hierarchy is `Hierarchy::EmptyPath`, it is converted according to the argument `convert_empty_path_to_absolute`:-
	///
	/// * If `true`, empty path becomes an absolute path.
	/// * If `false`, empty path becomes a rootless path.
	///
	/// Failure:-
	/// * Can fail with an `Err()` if there is not enough memory on the stack.
	/// * If the `path_segment` is empty and the hierarchy is `Hierarchy::EmptyPath`.
	#[inline(always)]
	pub const fn with_path_segment_const<const convert_empty_path_to_absolute: bool>(&mut self, path_segment: PathSegment<'a>) -> Result<(), PathSegment<'a>>
	{
		use Hierarchy::*;
		match self
		{
			AuthorityAndAbsolutePath(authority_and_absolute_path) => authority_and_absolute_path.with_path_segment_const(path_segment),
			
			AbsolutePath(non_empty_path) => non_empty_path.with_path_segment_const(path_segment),
			
			RootlessPath(non_empty_path) => non_empty_path.with_path_segment_const(path_segment),
			
			EmptyPath =>
			{
				let first_non_empty_path_segment =
				{
					if path_segment.is_empty()
					{
						return Err(path_segment)
					}
					unsafe { NonEmptyPathSegment::from_unchecked(path_segment) }
				};
				
				let non_empty_path = NonEmptyPath::new_minimal(first_non_empty_path_segment);
				
				let hierarchy = if convert_empty_path_to_absolute
				{
					AbsolutePath(non_empty_path)
				}
				else
				{
					RootlessPath(non_empty_path)
				};
				
				// This is safe, as `EmptyPath` does not require a drop.
				let write_pointer = self as *mut Self;
				unsafe { write_pointer.write(hierarchy) }
				
				Ok(())
			}
		}
	}
	
	/// `ihier-part = "//" iauthority ipath-abempty / ipath-absolute / ipath-rootless / ipath-empty`.
	/// `ipath-abempty  = *( "/" isegment )`.
	/// `ipath-absolute = "/" [ isegment-nz *( "/" isegment ) ]`.
	/// `ipath-rootless = isegment-nz *( "/" isegment )`.
	/// `ipath-empty    = 0<ipchar>`.
	/// `isegment       = *ipchar`.
	/// `isegment-nz    = 1*ipchar`.
	#[inline(always)]
	fn parse(scheme_specific_parsing_rule: &SchemeSpecificParsingRule, mut remaining_string: &'a str) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		use Hierarchy::*;
		use HierarchyParseError::*;
		
		let Utf8SequenceAndCharacter(utf8_sequence, character) = Self::decode_next_utf8_validity_already_checked_mandatory(&mut remaining_string, DidNotExpectEndParsingFirstCharacter)?;
		
		if scheme_specific_parsing_rule.hierarchy_starts_with_slash()
		{
			return if character == SlashChar
			{
				Self::parse_iauthority_ipath_abempty_or_ipath_absolute(scheme_specific_parsing_rule, remaining_string)
			}
			else
			{
				Err(InvalidFirstCharacter(character))
			}
		}
		
		#[allow(unused_qualifications)]
		match character
		{
			QuestionMarkChar => Ok((EmptyPath, ParseNextAfterHierarchy::query(remaining_string))),
			
			HashChar => Ok((EmptyPath, ParseNextAfterHierarchy::fragment_no_query(remaining_string))),
			
			SlashChar => Self::parse_iauthority_ipath_abempty_or_ipath_absolute(scheme_specific_parsing_rule, remaining_string),
			
			ipchar_iunreserved_without_ucschar!() => Self::parse_ipath_rootless(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_iunreserved_with_ucschar_2!()  => Self::parse_ipath_rootless(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_iunreserved_with_ucschar_3!()  => Self::parse_ipath_rootless(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_iunreserved_with_ucschar_4!()  => Self::parse_ipath_rootless(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_pct_encoded!()                 => Self::parse_ipath_rootless(Self::decode_percent_encoded(&mut remaining_string, InvalidPercentEncodedUtf8ParseFirstCharacter)?, remaining_string),
			ipchar_sub_delims!()                  => Self::parse_ipath_rootless(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_other!()                       => Self::parse_ipath_rootless(Self::decoded(utf8_sequence, character), remaining_string),
			
			_ => Err(InvalidFirstCharacter(character)),
		}
	}
	
	#[inline(always)]
	fn parse_iauthority_ipath_abempty_or_ipath_absolute(scheme_specific_parsing_rule: &SchemeSpecificParsingRule, mut remaining_string: &'a str) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		use HierarchyParseError::*;
		
		let Utf8SequenceAndCharacter(utf8_sequence, character) = Self::decode_next_utf8_validity_already_checked_mandatory(&mut remaining_string, DidNotExpectEndParsingSecondCharacter)?;
		if scheme_specific_parsing_rule.hierarchy_is_authority_and_absolute_path()
		{
			return if character == SlashChar
			{
				Self::parse_iauthority_ipath_abempty(scheme_specific_parsing_rule, remaining_string)
			}
			else
			{
				Err(InvalidSecondCharacter(character))
			}
		}
		
		#[allow(unused_qualifications)]
		match character
		{
			SlashChar => Self::parse_iauthority_ipath_abempty(scheme_specific_parsing_rule, remaining_string),
			
			ipchar_iunreserved_without_ucschar!() => Self::parse_ipath_absolute(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_iunreserved_with_ucschar_2!()  => Self::parse_ipath_absolute(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_iunreserved_with_ucschar_3!()  => Self::parse_ipath_absolute(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_iunreserved_with_ucschar_4!()  => Self::parse_ipath_absolute(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_pct_encoded!()                 => Self::parse_ipath_absolute(Self::decode_percent_encoded(&mut remaining_string, InvalidPercentEncodedUtf8ParseSecondCharacter)?, remaining_string),
			ipchar_sub_delims!()                  => Self::parse_ipath_absolute(Self::decoded(utf8_sequence, character), remaining_string),
			ipchar_other!()                       => Self::parse_ipath_absolute(Self::decoded(utf8_sequence, character), remaining_string),
			
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
	fn parse_ipath_rootless(first_character_of_first_path_segment: (bool, Utf8SequenceAndCharacter), remaining_utf8_bytes: &'a str) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		Self::parse_non_empty_path(first_character_of_first_path_segment, remaining_utf8_bytes, Hierarchy::RootlessPath, HierarchyParseError::RootlessPathParse)
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
	fn parse_ipath_absolute(first_character_of_first_path_segment: (bool, Utf8SequenceAndCharacter), remaining_utf8_bytes: &'a str) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		Self::parse_non_empty_path(first_character_of_first_path_segment, remaining_utf8_bytes, Hierarchy::AbsolutePath, HierarchyParseError::AbsolutePathParse)
	}
	
	#[inline(always)]
	fn parse_non_empty_path(first_character_of_first_path_segment: (bool, Utf8SequenceAndCharacter), remaining_utf8_bytes: &'a str, constructor: impl FnOnce(NonEmptyPath<'a, PathDepth>) -> Self, error: impl FnOnce(NonEmptyPathParseError) -> HierarchyParseError) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		let (was_percent_encoded, Utf8SequenceAndCharacter(utf8_sequence, _character)) = first_character_of_first_path_segment;
		NonEmptyPath::parse(constructor, error, (was_percent_encoded, utf8_sequence), remaining_utf8_bytes)
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
	fn parse_iauthority_ipath_abempty(scheme_specific_parsing_rule: &SchemeSpecificParsingRule, remaining_utf8_bytes: &'a str) -> Result<(Self, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		let (authority_and_absolute_path, parse_next) = AuthorityAndAbsolutePath::parse(scheme_specific_parsing_rule, remaining_utf8_bytes)?;
		Ok((Hierarchy::AuthorityAndAbsolutePath(authority_and_absolute_path), parse_next))
	}
	
	#[inline(always)]
	const fn decoded(utf8_sequence: Utf8SequenceEnum, character: char) -> (bool, Utf8SequenceAndCharacter)
	{
		(false, Utf8SequenceAndCharacter(utf8_sequence, character))
	}
	
	#[inline(always)]
	fn decode_next_utf8_validity_already_checked_mandatory<E: error::Error>(remaining_string: &mut &'a str, error: E) -> Result<Utf8SequenceAndCharacter, E>
	{
		remaining_string.decode_next_utf8_validity_already_checked().ok_or(error)
	}
	
	#[inline(always)]
	fn decode_percent_encoded(remaining_string: &mut &str, error: impl FnOnce(InvalidUtf8ParseError<PercentDecodeError>) -> HierarchyParseError) -> Result<(bool, Utf8SequenceAndCharacter), HierarchyParseError>
	{
		let utf8_sequence = remaining_string.decode_next_percent_encoded_utf8::<false>().map_err(error)?;
		Ok((true, utf8_sequence))
	}
}
