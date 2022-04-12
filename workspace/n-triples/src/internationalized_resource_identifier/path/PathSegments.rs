// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Path segments.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PathSegments<'a, const PathDepth: usize>(ConstSmallVec<PathSegment<'a>, PathDepth>);

impl<'a, const PathDepth: usize, const M: usize> const From<[PathSegment<'a>; M]> for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn from(array: [PathSegment<'a>; M]) -> Self
	{
		Self::from(ConstSmallVec::from(array))
	}
}

impl<'a, const PathDepth: usize> const From<ConstSmallVec<PathSegment<'a>, PathDepth>> for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn from(value: ConstSmallVec<PathSegment<'a>, PathDepth>) -> Self
	{
		Self(value)
	}
}

impl<'a, const PathDepth: usize> Borrow<[PathSegment<'a>]> for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn borrow(&self) -> &[PathSegment<'a>]
	{
		self.deref()
	}
}

impl<'a, const PathDepth: usize> AsRef<[PathSegment<'a>]> for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn as_ref(&self) -> &[PathSegment<'a>]
	{
		self.deref()
	}
}

impl<'a, const PathDepth: usize> Deref for PathSegments<'a, PathDepth>
{
	type Target = [PathSegment<'a>];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'a, const PathDepth: usize> const Default for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(ConstSmallVec::default())
	}
}

impl<'a, const PathDepth: usize> PathSegments<'a, PathDepth>
{
	/// Assumes that on input `remaining_utf8_bytes` is positioned just to the right of any `Slash`, i.e. `Slash` would be at `remaining_utf8_bytes[-1]`.
	#[inline(always)]
	pub(super) fn parse(&mut self, mut remaining_utf8_bytes: &'a [u8]) -> Result<ParseNextAfterHierarchy<'a>, PathSegmentsParseError>
	{
		let parse_next_after_hierarchy = loop
		{
			match memchr3(QuestionMark, Hash, Slash, remaining_utf8_bytes)
			{
				None =>
				{
					self.decode_percent_encoded_path_segment(remaining_utf8_bytes)?;
					break ParseNextAfterHierarchy::NoQueryNoFragment
				}
				
				Some(index) =>
				{
					self.decode_percent_encoded_path_segment(remaining_utf8_bytes.before_index(index))?;
					let after_path_segment_bytes = remaining_utf8_bytes.after_index(index);
					
					match remaining_utf8_bytes.get_unchecked_value_safe(index)
					{
						QuestionMark => break ParseNextAfterHierarchy::query(after_path_segment_bytes),
						
						Hash => break ParseNextAfterHierarchy::fragment_no_query(after_path_segment_bytes),
						
						Slash =>
						{
							remaining_utf8_bytes = after_path_segment_bytes;
						}
						
						_ => unreachable_code_const("memchr3"),
					}
				}
			}
		};
		Ok(parse_next_after_hierarchy)
	}
	
	#[inline(always)]
	fn decode_percent_encoded_path_segment(&mut self, percent_encoded_path_segment_utf8_bytes: &'a [u8]) -> Result<(), PathSegmentsParseError>
	{
		use PathSegmentsParseError::*;
		
		let path_segment = PathSegment::decode_percent_encoded_path_segment(percent_encoded_path_segment_utf8_bytes).map_err(PathSegmentParse)?;
		self.0.try_reserve_push(path_segment).map_err(OutOfMemory)
	}
}
