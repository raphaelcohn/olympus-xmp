// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) type Utf8Sequence1 = [u8; 1];

impl Utf8Sequence for Utf8Sequence1
{
	const Length: Utf8CharacterLength = One;
	
	type Remainder = ();
	
	#[inline(always)]
	fn construct(first: u8, remainder: Self::Remainder) -> Self
	{
		let () = remainder;
		[first]
	}
	
	#[inline(always)]
	fn is(first: u8) -> bool
	{
		first & 0x80 == 0x00
	}
	
	#[inline(always)]
	fn into_raw_unicode_code_point(self) -> u32
	{
		self[0] as u32
	}
	
	#[inline(always)]
	fn slice_length<BP: ByteProvider>() -> NonZeroUsize
	{
		BP::OneSliceLength
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

impl Utf8SequenceNonConst for Utf8Sequence1
{
	#[inline(always)]
	fn parse<BP: ByteProvider>(bytes: &[u8]) -> Result<<Self as Utf8Sequence>::Remainder, BP::Error>
	{
		Ok(())
	}
}
