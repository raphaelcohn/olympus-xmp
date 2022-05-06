// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// UTF-8 sequence of 4 bytes.
pub type Utf8Sequence4 = [u8; 4];

impl Utf8Sequence for Utf8Sequence4
{
	const Length: Utf8CharacterLength = Four;
	
	type Remainder = (u8, u8, u8);
	
	#[inline(always)]
	fn construct(first: u8, remainder: Self::Remainder) -> Self
	{
		let (second, third, fourth) = remainder;
		[first, second, third, fourth]
	}
	
	#[inline(always)]
	fn is(first: u8) -> bool
	{
		first & 0xF8 == xF0
	}
	
	#[inline(always)]
	fn into_raw_unicode_code_point(self) -> u32
	{
		let first = self[0];
		let second = self[1];
		let third = self[2];
		let fourth = self[3];
		((first as u32) & x07) << Shift18 | (second as u32) << Shift12 | (third as u32) << Shift6 | (fourth as u32)
	}
	
	#[inline(always)]
	fn try_into_char(self) -> Result<char, CharTryFromError>
	{
		char::try_from(self.into_raw_unicode_code_point())
	}
	
	#[inline(always)]
	unsafe fn unchecked_into_char(self) -> char
	{
		char::from_u32_unchecked(self.into_raw_unicode_code_point())
	}
	
	#[inline(always)]
	fn encode_character(character: char) -> Self
	{
		Self::encode_u32(character as u32)
	}
	
	#[inline(always)]
	fn encode_u32(code: u32) -> Self
	{
		[
			(code >> Shift18 & x07) as u8 | TAG_FOUR_B,
			(code >> Shift12 & x3F) as u8 | TAG_CONT,
			(code >> Shift6 & x3F) as u8 | TAG_CONT,
			(code & x3F) as u8 | TAG_CONT,
		]
	}
	
	#[inline(always)]
	fn write_unchecked(self, to: NonNull<u8>)
	{
		let pointer = to.as_ptr().cast::<Self>();
		unsafe { pointer.write(self) }
	}
}

impl const Utf8SequenceCrate for Utf8Sequence4
{
	#[inline(always)]
	fn slice_length<BP: ByteProvider>() -> NonZeroUsize
	{
		BP::FourSliceLength
	}
}

impl Utf8SequenceNonConst for Utf8Sequence4
{
	#[inline(always)]
	fn parse<BP: ByteProvider>(bytes: &[u8]) -> Result<<Self as Utf8Sequence>::Remainder, BP::Error>
	{
		BP::four(bytes)
	}
}
