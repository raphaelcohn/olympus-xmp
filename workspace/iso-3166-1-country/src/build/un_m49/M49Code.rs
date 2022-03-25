// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct M49Code([u8; 3]);

impl Debug for M49Code
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		let array = self.0;
		write!(f, "{}{}{}", array[0] as char, array[1] as char, array[2] as char)
	}
}

impl Display for M49Code
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl M49Code
{
	#[inline(always)]
	fn is_sark(self) -> bool
	{
		self.0 == [b'6', b'8', b'0']
	}
}
