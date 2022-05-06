// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Contains an UTF-8 sequence and its decoded character form, for efficiency.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Utf8SequenceAndCharacter(pub Utf8SequenceEnum, pub char);

impl const Default for Utf8SequenceAndCharacter
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(Utf8SequenceEnum::default(), 0x00 as char)
	}
}

impl const From<char> for Utf8SequenceAndCharacter
{
	#[inline(always)]
	fn from(character: char) -> Self
	{
		Self(Utf8SequenceEnum::from(character), character)
	}
}

impl<U8S: Utf8Sequence> const FromUnchecked<U8S> for Utf8SequenceAndCharacter
{
	#[inline(always)]
	unsafe fn from_unchecked(utf8_sequence: U8S) -> Self
	{
		Self
		(
			Utf8SequenceEnum::from(utf8_sequence),
			unsafe { utf8_sequence.unchecked_into_char() }
		)
	}
}

impl const FromUnchecked<u8> for Utf8SequenceAndCharacter
{
	#[inline(always)]
	unsafe fn from_unchecked(ascii_byte: u8) -> Self
	{
		Self
		(
			Self
			(
				Utf8SequenceEnum::One([ascii_byte]),
				ascii_byte as char
			)
		)
	}
}

impl<U8S: Utf8Sequence> const TryFrom<U8S> for Utf8SequenceAndCharacter
{
	type Error = CharTryFromError;
	
	#[inline(always)]
	fn try_from(utf8_sequence: U8S) -> Result<Self, Self::Error>
	{
		Ok
		(
			Self
			(
				Utf8SequenceEnum::from(utf8_sequence),
				utf8_sequence.try_into_char()?
			)
		)
	}
}
