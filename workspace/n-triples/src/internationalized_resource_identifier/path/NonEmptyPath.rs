// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Path.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NonEmptyPath<'a>
{
	/// Will have a length of at least 1.
	pub first_non_empty_path_segment: NonEmptyPathSegment<'a>,
	
	/// Zero or more; each path segment can be empty.
	pub remaining_path_segments: PathSegments<'a>,
}

impl<'a> NonEmptyPath<'a>
{
	#[inline(always)]
	pub(super) fn parse(constructor: impl FnOnce(Self) -> Hierarchy, error: impl FnOnce(NonEmptyPathParseError) -> HierarchyParseError, first_character_of_first_path_segment: (bool, char, Utf8CharacterLength), mut remaining_utf8_bytes: &'a [u8]) -> Result<(Hierarchy, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		NonEmptyPathParseState::new(constructor).parse(remaining_utf8_bytes).map_err(error)
	}
}
