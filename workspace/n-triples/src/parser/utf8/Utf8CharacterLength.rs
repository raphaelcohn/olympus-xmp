// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum Utf8CharacterLength
{
	One = 1,

	Two = 2,

	Three = 3,

	Four = 4,
}

impl const Into<u8> for Utf8CharacterLength
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl const Into<u32> for Utf8CharacterLength
{
	#[inline(always)]
	fn into(self) -> u32
	{
		let into: u8 = self.into();
		into as u32
	}
}

impl const Into<usize> for Utf8CharacterLength
{
	#[inline(always)]
	fn into(self) -> usize
	{
		let into: u8 = self.into();
		into as usize
	}
}
