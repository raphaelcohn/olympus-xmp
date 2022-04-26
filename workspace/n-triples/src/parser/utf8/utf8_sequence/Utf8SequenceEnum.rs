// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(super) enum Utf8SequenceEnum
{
	One(Utf8Sequence1) = One.into(),
	
	Two(Utf8Sequence2) = Two.into(),
	
	Three(Utf8Sequence3) = Three.into(),
	
	Four(Utf8Sequence4) = Four.into(),
}

impl const From<Utf8Sequence1> for Utf8SequenceEnum
{
	#[inline(always)]
	fn from(utf8_sequence: Utf8Sequence1) -> Self
	{
		Utf8SequenceEnum::One(utf8_sequence)
	}
}

impl const From<Utf8Sequence2> for Utf8SequenceEnum
{
	#[inline(always)]
	fn from(utf8_sequence: Utf8Sequence2) -> Self
	{
		Utf8SequenceEnum::Two(utf8_sequence)
	}
}

impl const From<Utf8Sequence3> for Utf8SequenceEnum
{
	#[inline(always)]
	fn from(utf8_sequence: Utf8Sequence3) -> Self
	{
		Utf8SequenceEnum::Three(utf8_sequence)
	}
}

impl const From<Utf8Sequence4> for Utf8SequenceEnum
{
	#[inline(always)]
	fn from(utf8_sequence: Utf8Sequence4) -> Self
	{
		Utf8SequenceEnum::Four(utf8_sequence)
	}
}

impl Utf8SequenceEnum
{
	#[inline(always)]
	pub(super) const fn occupies_more_than_one_byte(&self) -> bool
	{
		self.utf8_character_length() != One
	}
	
	#[inline(always)]
	const fn utf8_character_length(&self) -> Utf8CharacterLength
	{
		unsafe { transmute(discriminant(self)) }
	}
}
