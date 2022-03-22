// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Number of deciseconds (10⁻¹ seconds) (zero based).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Decisecond
{
	#[allow(missing_docs)]
	_0 = 0,
	
	#[allow(missing_docs)]
	_1 = 1,
	
	#[allow(missing_docs)]
	_2 = 2,
	
	#[allow(missing_docs)]
	_3 = 3,
	
	#[allow(missing_docs)]
	_4 = 4,
	
	#[allow(missing_docs)]
	_5 = 5,
	
	#[allow(missing_docs)]
	_6 = 6,
	
	#[allow(missing_docs)]
	_7 = 7,
	
	#[allow(missing_docs)]
	_8 = 8,
	
	#[allow(missing_docs)]
	_9 = 9,
}

impl TryFrom<u8> for Decisecond
{
	type Error = TooLargeError<u8>;
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value > 9
		{
			Err(TooLargeError(value))
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}
