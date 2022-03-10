// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A day of a month.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum DayOfMonth
{
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
	
	#[allow(missing_docs)]
	_10 = 10,
	
	#[allow(missing_docs)]
	_11 = 11,
	
	#[allow(missing_docs)]
	_12 = 12,
	
	#[allow(missing_docs)]
	_13 = 13,
	
	#[allow(missing_docs)]
	_14 = 14,
	
	#[allow(missing_docs)]
	_15 = 15,
	
	#[allow(missing_docs)]
	_16 = 16,
	
	#[allow(missing_docs)]
	_17 = 17,
	
	#[allow(missing_docs)]
	_18 = 18,
	
	#[allow(missing_docs)]
	_19 = 19,
	
	#[allow(missing_docs)]
	_20 = 20,
	
	#[allow(missing_docs)]
	_21 = 21,
	
	#[allow(missing_docs)]
	_22 = 22,
	
	#[allow(missing_docs)]
	_23 = 23,
	
	#[allow(missing_docs)]
	_24 = 24,
	
	#[allow(missing_docs)]
	_25 = 25,
	
	#[allow(missing_docs)]
	_26 = 26,
	
	#[allow(missing_docs)]
	_27 = 27,
	
	#[allow(missing_docs)]
	_28 = 28,
	
	#[allow(missing_docs)]
	_29 = 29,
	
	#[allow(missing_docs)]
	_30 = 30,
	
	#[allow(missing_docs)]
	_31 = 31,
}

impl TryFrom<NonZeroU8> for DayOfMonth
{
	type Error = TooLargeError<NonZeroU8>;
	
	#[inline(always)]
	fn try_from(value: NonZeroU8) -> Result<Self, Self::Error>
	{
		const InclusiveMaximum: NonZeroU8 = new_non_zero_u8(31);
		if value > InclusiveMaximum
		{
			Err(TooLargeError(value))
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}

impl DayOfMonth
{
	#[inline(always)]
	fn is_valid_for(self, year_month: YearMonth) -> bool
	{
		let value = new_non_zero_u8(self as u8);
		value <= year_month.number_of_days_in_month()
	}
}
