// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug)]
struct NonEmptyPathParseState<'a, F: FnOnce(NonEmptyPath<'a, PathDepth>) -> Hierarchy<'a, PathDepth>, const PathDepth: usize>
{
	constructor: F,
	
	remaining_path_segments: PathSegments<'a, PathDepth>,
}

impl<'a, F: FnOnce(NonEmptyPath<'a, PathDepth>) -> Hierarchy<'a, PathDepth>, const PathDepth: usize> NonEmptyPathParseState<'a, F, PathDepth>
{
	#[inline(always)]
	fn new(constructor: F) -> Self
	{
		Self
		{
			constructor,
			remaining_path_segments: PathSegments::default(),
		}
	}
	
	#[inline(always)]
	fn parse(mut self, first_character_of_first_path_segment: (bool, Utf8SequenceEnum), remaining: &'a str) -> Result<(Hierarchy<'a, PathDepth>, ParseNextAfterHierarchy<'a>), NonEmptyPathParseError>
	{
		let (first_non_empty_path_segment, remaining_utf8_bytes) = match remaining.memchr3(QuestionMark, Hash, Slash)
		{
			// everything from `character` to the end is the first path segment.
			None =>
			{
				let remaining_percent_encoded_path_segment = remaining;
				let first_non_empty_path_segment = NonEmptyPathSegment::decode_percent_encoded_path_segment_remainder(first_character_of_first_path_segment, remaining_percent_encoded_path_segment)?;
				return self.finish(first_non_empty_path_segment, ParseNextAfterHierarchy::NoQueryNoFragment)
			}
			
			// everything from `character` to the index is the first path segment.
			Some(index) =>
			{
				let remaining_percent_encoded_path_segment = remaining.before_index(index);
				let first_non_empty_path_segment = NonEmptyPathSegment::decode_percent_encoded_path_segment_remainder(first_character_of_first_path_segment, remaining_percent_encoded_path_segment)?;
				let after_first_non_empty_path_segment_bytes = remaining.after_index(index);
				
				match remaining.get_unchecked_value_safe(index)
				{
					QuestionMark => return self.finish(first_non_empty_path_segment, ParseNextAfterHierarchy::query(after_first_non_empty_path_segment_bytes)),
					
					Hash => return self.finish(first_non_empty_path_segment, ParseNextAfterHierarchy::fragment_no_query(after_first_non_empty_path_segment_bytes)),
					
					Slash => (first_non_empty_path_segment, after_first_non_empty_path_segment_bytes),
					
					_ => unreachable_code_const("memchr3"),
				}
			}
		};
		
		let parse_next = self.remaining_path_segments.parse(remaining_utf8_bytes)?;
		self.finish(first_non_empty_path_segment, parse_next)
	}
	
	#[inline(always)]
	fn finish(self, first_non_empty_path_segment: NonEmptyPathSegment<'a>, parse_next: ParseNextAfterHierarchy<'a>) -> Result<(Hierarchy<'a, PathDepth>, ParseNextAfterHierarchy<'a>), NonEmptyPathParseError>
	{
		Ok(((self.constructor)(NonEmptyPath { first_non_empty_path_segment, remaining_path_segments: self.remaining_path_segments }), parse_next))
	}
}
