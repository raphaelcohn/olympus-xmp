// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Path.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NonEmptyPath<'a, const PathDepth: usize>
{
	/// Will have a length of at least 1.
	pub first_non_empty_path_segment: NonEmptyPathSegment<'a>,
	
	/// Zero or more; each path segment can be empty.
	pub remaining_path_segments: PathSegments<'a, PathDepth>,
}

impl<'a, const PathDepth: usize> const From<NonEmptyPathSegment<'a>> for NonEmptyPath<'a, PathDepth>
{
	#[inline(always)]
	fn from(first_non_empty_path_segment: NonEmptyPathSegment<'a>) -> Self
	{
		Self::new_minimal(first_non_empty_path_segment)
	}
}

impl<'a, const PathDepth: usize> Display for NonEmptyPath<'a, PathDepth>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.first_non_empty_path_segment)?;
		write!(f, "{}", self.remaining_path_segments)
	}
}

impl<'a, const PathDepth: usize> TryToOwnInPlace for NonEmptyPath<'a, PathDepth>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.first_non_empty_path_segment.try_to_own_in_place()?;
		self.remaining_path_segments.try_to_own_in_place()
	}
}

impl<'a, const PathDepth: usize> TryToOwn for NonEmptyPath<'a, PathDepth>
{
	type TryToOwned = NonEmptyPath<'static, PathDepth>;
	
	#[inline(always)]
	fn try_to_own(self) -> Result<Self::TryToOwned, TryReserveError>
	{
		Ok
		(
			NonEmptyPath
			{
				first_non_empty_path_segment: self.first_non_empty_path_segment.try_to_own()?,
				remaining_path_segments: self.remaining_path_segments.try_to_own()?
			}
		)
	}
}

impl<'a, const PathDepth: usize> NonEmptyPath<'a, PathDepth>
{
	/// New instance.
	#[inline(always)]
	pub const fn new_minimal(first_non_empty_path_segment: NonEmptyPathSegment<'a>) -> Self
	{
		Self::new(first_non_empty_path_segment, PathSegments::Empty)
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn new(first_non_empty_path_segment: NonEmptyPathSegment<'a>, remaining_path_segments: PathSegments<'a, PathDepth>) -> Self
	{
		Self
		{
			first_non_empty_path_segment,
		
			remaining_path_segments: PathSegments::from(remaining_path_segments),
		}
	}
	
	/// Appends a path segment.
	///
	/// Can fail with an `Err()` if there is not enough heap memory.
	#[inline(always)]
	pub fn with_path_segment(&mut self, path_segment: PathSegment<'a>) -> Result<(), TryReserveError>
	{
		self.remaining_path_segments.with_path_segment(path_segment)
	}
	
	/// Appends a path segment.
	///
	/// Can fail with an `Err()` if there is not enough memory to allocate on the stack, i.e. the length (`len()`) is already equal to `PathDepth`.
	#[inline(always)]
	pub const fn with_path_segment_const(&mut self, path_segment: PathSegment<'a>) -> Result<(), PathSegment<'a>>
	{
		self.remaining_path_segments.with_path_segment_const(path_segment)
	}
	
	#[inline(always)]
	pub(super) fn parse(constructor: impl FnOnce(Self) -> Hierarchy<'a, PathDepth>, error: impl FnOnce(NonEmptyPathParseError) -> HierarchyParseError, first_character_of_first_path_segment: (bool, Utf8SequenceEnum), remaining_utf8_bytes: &'a str) -> Result<(Hierarchy<'a, PathDepth>, ParseNextAfterHierarchy<'a>), HierarchyParseError>
	{
		NonEmptyPathParseState::new(constructor).parse(first_character_of_first_path_segment, remaining_utf8_bytes).map_err(error)
	}
	
	/// Removes `other`.
	#[inline(always)]
	pub fn remove<'prefix>(&self, prefix: &NonEmptyPath<'prefix, PathDepth>) -> Option<&[PathSegment<'a>]>
	{
		if self.first_non_empty_path_segment != prefix.first_non_empty_path_segment
		{
			return None
		}
		self.remaining_path_segments.remove(&prefix.remaining_path_segments)
	}
}
