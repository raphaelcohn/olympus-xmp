// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


struct NonEmptyPathParseState<'a, F: FnOnce(NonEmptyPath) -> Hierarchy>
{
	constructor: F,
	
	remaining_path_segments: PathSegments<'a>,
}

impl<'a, F: FnOnce(NonEmptyPath) -> Hierarchy> NonEmptyPathParseState<'a, F>
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
	fn parse(self, mut remaining_utf8_bytes: &'a [u8]) -> Result<(Hierarchy, ParseNext<'a>), HierarchyParseError>
	{
		let (first_non_empty_path_segment, mut remaining_utf8_bytes) = match memchr3(QuestionMark, Hash, Slash, remaining_utf8_bytes)
		{
			// everything from `character` to the end is the first path segment.
			None =>
			{
				let first_non_empty_path_segment = NonEmptyPathSegment::decode_percent_encoded_path_segment_remainder(first_character_of_first_path_segment, remaining_utf8_bytes)?;
				return self.finish(first_non_empty_path_segment, ParseNext::NoQueryNoFragment)
			}
			
			// everything from `character` to the index is the first path segment.
			Some(index) =>
			{
				let first_non_empty_path_segment = NonEmptyPathSegment::decode_percent_encoded_path_segment_remainder(first_character_of_first_path_segment, remaining_utf8_bytes.before_index(index))?;
				let after_first_non_empty_path_segment_bytes = remaining_utf8_bytes.after_index(index);
				
				match remaining_utf8_bytes.get_unchecked_value_safe(index)
				{
					QuestionMark => return self.finish(first_non_empty_path_segment, ParseNext::query(after_first_non_empty_path_segment_bytes)),
					
					Hash => return self.finish(first_non_empty_path_segment, ParseNext::fragment_no_query(after_first_non_empty_path_segment_bytes)),
					
					Slash => (first_non_empty_path_segment, after_first_non_empty_path_segment_bytes),
					
					_ => unreachable_code_const("memchr3"),
				}
			}
		};
		
		let parse_next = self.remaining_path_segments.parse(remaining_utf8_bytes)?;
		self.finish(first_non_empty_path_segment, parse_next)
	}
	
	#[inline(always)]
	fn finish(self, first_non_empty_path_segment: NonEmptyPathSegment<'a>, parse_next: ParseNext<'a>) -> Result<(Hierarchy, ParseNext<'a>), HierarchyParseError>
	{
		Ok(((self.constructor)(NonEmptyPath { first_non_empty_path_segment, remaining_path_segments: self.remaining_path_segments }), parse_next))
	}
}
