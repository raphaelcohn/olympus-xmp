// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) type Utf8Sequence3 = [u8; 3];

impl Utf8Sequence for Utf8Sequence3
{
	const Length: Utf8CharacterLength = Three;
	
	type Remainder = (u8, u8);
	
	#[inline(always)]
	fn construct(first: u8, remainder: Self::Remainder) -> Self
	{
		let (second, third) = remainder;
		[first, second, third]
	}
	
	#[inline(always)]
	fn is(first: u8) -> bool
	{
		first & xF0 == xE0
	}
	
	#[inline(always)]
	fn into_raw_unicode_code_point(self) -> u32
	{
		let first = self[0];
		let second = self[1];
		let third = self[2];
		((first as u32) & x0F) << Shift12 | (second as u32) << Shift6 | (third as u32)
	}
	
	#[inline(always)]
	fn slice_length<BP: ByteProvider>() -> NonZeroUsize
	{
		BP::ThreeSliceLength
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
}

impl Utf8SequenceNonConst for Utf8Sequence3
{
	#[inline(always)]
	fn parse<BP: ByteProvider>(bytes: &[u8]) -> Result<<Self as Utf8Sequence>::Remainder, BP::Error>
	{
		BP::three(bytes)
	}
}
