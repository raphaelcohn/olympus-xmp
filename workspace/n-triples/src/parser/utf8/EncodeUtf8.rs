// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait EncodeUtf8
{
	type R: Sized;
	
	fn push_unchecked<const length: usize>(buffer: &mut Vec<u8>, offset: usize, encoded_utf8_bytes: [u8; length]) -> Self::R;
	
	#[inline(always)]
	fn encode_utf8(buffer: &mut Vec<u8>, character: char, offset: usize) -> Self::R
	{
		const TAG_CONT: u8 = 0b1000_0000;
		const TAG_TWO_B: u8 = 0b1100_0000;
		const TAG_THREE_B: u8 = 0b1110_0000;
		const TAG_FOUR_B: u8 = 0b1111_0000;
		
		#[inline(always)]
		const fn encoded_utf_bytes_1(code: u32) -> [u8; 1]
		{
			[
				code as u8
			]
		}
		
		#[inline(always)]
		const fn encoded_utf_bytes_2(code: u32) -> [u8; 2]
		{
			[
				(code >> 6 & 0x1F) as u8 | TAG_TWO_B,
				(code & 0x3F) as u8 | TAG_CONT
			]
		}
		
		#[inline(always)]
		const fn encoded_utf_bytes_3(code: u32) -> [u8; 3]
		{
			[
				(code >> 12 & 0x0F) as u8 | TAG_THREE_B,
				(code >> 6 & 0x3F) as u8 | TAG_CONT,
				(code & 0x3F) as u8 | TAG_CONT
			]
		}
		
		#[inline(always)]
		const fn encoded_utf_bytes_4(code: u32) -> [u8; 4]
		{
			[
				(code >> 18 & 0x07) as u8 | TAG_FOUR_B,
				(code >> 12 & 0x3F) as u8 | TAG_CONT,
				(code >> 6 & 0x3F) as u8 | TAG_CONT,
				(code & 0x3F) as u8 | TAG_CONT,
			]
		}
		
		const MAX_ONE_B: u32 = 0x80;
		const MAX_TWO_B: u32 = 0x800;
		const MAX_THREE_B: u32 = 0x10000;
		
		let code = character as u32;
		
		if code < MAX_ONE_B
		{
			Self::push_unchecked::<_>(buffer, offset, encoded_utf_bytes_1(code))
		}
		else if code < MAX_TWO_B
		{
			Self::push_unchecked::<_>(buffer, offset, encoded_utf_bytes_2(code))
		}
		else if code < MAX_THREE_B
		{
			Self::push_unchecked::<_>(buffer, offset, encoded_utf_bytes_3(code))
		}
		else
		{
			Self::push_unchecked::<_>(buffer, offset, encoded_utf_bytes_4(code))
		}
	}
}
