// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait EncodeUtf8
{
	type R: Sized;
	
	fn push_unchecked<const length: usize>(buffer: &mut Vec<u8>, offset: usize, encoded_utf8_bytes: [u8; length]) -> Self::R;
	
	#[inline(always)]
	fn encode_utf8(buffer: &mut Vec<u8>, character: char, offset: usize) -> Self::R
	{
		const MAX_ONE_B: u32 = 0x80;
		const MAX_TWO_B: u32 = 0x800;
		const MAX_THREE_B: u32 = 0x10000;
		
		let code = character as u32;
		
		if code < MAX_ONE_B
		{
			Self::push_unchecked::<_>(buffer, offset, encode_utf8_bytes_1(code))
		}
		else if code < MAX_TWO_B
		{
			Self::push_unchecked::<_>(buffer, offset, encode_utf8_bytes_2(code))
		}
		else if code < MAX_THREE_B
		{
			Self::push_unchecked::<_>(buffer, offset, encode_utf8_bytes_3(code))
		}
		else
		{
			Self::push_unchecked::<_>(buffer, offset, encode_utf8_bytes_4(code))
		}
	}
}
