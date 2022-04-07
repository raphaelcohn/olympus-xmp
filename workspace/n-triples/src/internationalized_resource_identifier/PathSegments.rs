// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Path segments.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PathSegments<'a>(Vec<PathSegment<'a>>);

impl<'a> PathSegments<'a>
{
	/// Assumes that on input `remaining_utf8_bytes` is positioned just to the right of any `Slash`, i.e. `Slash` would be at `remaining_utf8_bytes[-1]`.
	#[inline(always)]
	fn parse(&mut self, mut remaining_utf8_bytes: &'a [u8]) -> Result<ParseNext<'a>, HierarchyParseError>
	{
		let parse_next = loop
		{
			match memchr3(QuestionMark, Hash, Slash, remaining_utf8_bytes)
			{
				None =>
				{
					parse_state.decode_percent_encoded_path_segment(remaining_utf8_bytes)?;
					break ParseNext::NoQueryNoFragment
				}
				
				Some(index) =>
				{
					parse_state.decode_percent_encoded_path_segment(remaining_utf8_bytes.before_index(index))?;
					let after_path_segment_bytes = remaining_utf8_bytes.after_index(index);
					
					match remaining_utf8_bytes.get(index)
					{
						QuestionMark => break ParseNext::query(after_path_segment_bytes),
						
						Hash => break ParseNext::fragment_no_query(after_path_segment_bytes),
						
						Slash =>
						{
							remaining_utf8_bytes = after_path_segment_bytes;
						}
						
						_ => unreachable_code_const("memchr3"),
					}
				}
			}
		};
		Ok(parse_next)
	}
	
	#[inline(always)]
	fn decode_percent_encoded_path_segment(&mut self, percent_encoded_path_segment_utf8_bytes: &[u8]) -> Result<(), HierarchyParseError>
	{
		let path_segment = PathSegment::decode_percent_encoded_path_segment(percent_encoded_path_segment_utf8_bytes)?;
		self.0.try_reserve(1)?;
		self.0.push_unchecked(path_segment);
		Ok(())
	}
}
