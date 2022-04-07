// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A (potentially empty) path segment.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PathSegment<'a>(Cow<'a, str>);

impl<'a> PathSegment<'a>
{
	#[inline(always)]
	fn decode_percent_encoded_path_segment(percent_encoded_path_segment_utf8_bytes: &'a [u8]) -> Result<Self, HierarchyParseError>
	{
		let remaining_percent_encoded_path_segment_utf8_bytes = &mut percent_encoded_path_segment_utf8_bytes;
		let mut string = StringSoFar::new_stack(remaining_percent_encoded_path_segment_utf8_bytes);
		Self::decode_percent_encoded_path_segment_common(string, remaining_percent_encoded_path_segment_utf8_bytes, Self)
	}
	
	/// [RFC 3987, Section 2.2](https://datatracker.ietf.org/doc/html/rfc3987#section-2.2).
	///
	/// `isegment    = *ipchar`.
	/// `isegment-nz = 1*ipchar`.
	#[inline(always)]
	fn decode_percent_encoded_path_segment_common<R>(mut string: StringSoFar, remaining_percent_encoded_path_segment_utf8_bytes: &mut &'a [u8], constructor: impl FnOnce(Cow<'a, str>) -> R) -> Result<R, HierarchyParseError>
	{
		use Utf8CharacterLength::*;
		
		loop
		{
			match Hierarchy::decode_next_utf8_validity_already_checked(remaining_percent_encoded_path_segment_utf8_bytes)
			{
				None => break,
				
				Some(character) =>
				{
					match character
					{
						ipchar_iunreserved_without_ucschar!() => string.push(character, One),
						ipchar_iunreserved_with_ucschar_2!()  => string.push(character, Two),
						ipchar_iunreserved_with_ucschar_3!()  => string.push(character, Three),
						ipchar_iunreserved_with_ucschar_4!()  => string.push(character, Four),
						ipchar_pct_encoded!()                 => string.push_forcing_heap_percent_encoded(remaining_percent_encoded_path_segment_utf8_bytes),
						ipchar_sub_delims!()                  => string.push(character, One),
						ipchar_other!()                       => string.push(character, One),
						
						_ => Err(HierarchyParseError::InvalidCharacter(character)),
					}
				}
			}
		}
		
		Ok(constructor(string.to_cow()))
	}
}
