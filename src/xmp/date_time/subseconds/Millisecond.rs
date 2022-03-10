// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Number of milliseconds (10⁻³ seconds) (zero based).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Millisecond(u16);

impl TryFrom<u16> for Millisecond
{
	type Error = TooLargeError<u16>;
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value > 999
		{
			Err(TooLargeError(value))
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}
