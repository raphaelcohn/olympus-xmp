// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) trait EncodeUtf8
{
	type R: Sized;
	
	fn push_unchecked<const length: usize>(self, encoded_utf8_bytes: [u8; length]) -> Self::R;
	
	#[inline(always)]
	fn encode_utf8(self, character: char) -> Self::R
	{
		let code = character as u32;
		
		if code < MAX_ONE_B
		{
			self.push_unchecked::<_>(encode_utf8_bytes_1(code))
		}
		else
		{
			self.encode_utf8_of_two_or_more_bytes(code)
		}
	}
	
	#[inline(always)]
	fn encode_utf8_of_two_or_more_bytes(self, code: u32) -> Self::R
	{
		debug_assert!(character_occupies_more_than_one_byte_as_utf8(code as char));
		
		if code < MAX_TWO_B
		{
			self.push_unchecked::<_>(encode_utf8_bytes_2(code))
		}
		else if code < MAX_THREE_B
		{
			self.push_unchecked::<_>(encode_utf8_bytes_3(code))
		}
		else
		{
			self.push_unchecked::<_>(encode_utf8_bytes_4(code))
		}
	}
}
