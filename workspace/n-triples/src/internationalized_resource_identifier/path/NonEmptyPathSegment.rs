// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Can not be empty.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NonEmptyPathSegment<'a>(Cow<'a, str>);

impl<'a> const Into<Cow<'a, str>> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn into(self) -> Cow<'a, str>
	{
		self.0
	}
}

impl<'a> const Borrow<str> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn borrow(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const AsRef<str> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn as_ref(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const Deref for NonEmptyPathSegment<'a>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'a> NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn decode_percent_encoded_path_segment_remainder(first_character_of_first_path_segment: (bool, char, Utf8CharacterLength), remaining_percent_encoded_path_segment_utf8_bytes: &'a [u8]) -> Result<NonEmptyPathSegment, PathSegmentParseError>
	{
		let (was_percent_encoded, character, utf8_character_length) = first_character_of_first_path_segment;
		
		let mut string = if was_percent_encoded
		{
			StringSoFar::new_heap(character, utf8_character_length)?
		}
		else
		{
			StringSoFar::new_stack_rewind_buffer(remaining_percent_encoded_path_segment_utf8_bytes, utf8_character_length)
		};
		
		PathSegment::decode_percent_encoded_path_segment_common(string, &mut remaining_percent_encoded_path_segment_utf8_bytes, NonEmptyPathSegment)
	}
}
