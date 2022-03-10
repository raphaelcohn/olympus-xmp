// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A year.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Year(NonZeroU16);

impl const From<NonZeroU16> for Year
{
	#[inline(always)]
	fn from(value: NonZeroU16) -> Self
	{
		Self(value)
	}
}

impl const Into<NonZeroU16> for Year
{
	#[inline(always)]
	fn into(self) -> NonZeroU16
	{
		self.0
	}
}

impl Year
{
	/// Is this a leap year?
	#[inline(always)]
	pub fn is_leap_year(self) -> bool
	{
		let year = self.0.get();
		
		if (year % 400) == 0
		{
			true
		}
		else if (year % 100) == 0
		{
			false
		}
		else
		{
			year % 4 == 0
		}
	}
}
