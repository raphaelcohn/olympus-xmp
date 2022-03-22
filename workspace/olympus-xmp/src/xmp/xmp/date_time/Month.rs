// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Month (one based).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Month
{
	#[allow(missing_docs)]
	January = 1,
	
	#[allow(missing_docs)]
	February = 2,
	
	#[allow(missing_docs)]
	March = 3,
	
	#[allow(missing_docs)]
	April = 4,
	
	#[allow(missing_docs)]
	May = 5,
	
	#[allow(missing_docs)]
	June = 6,
	
	#[allow(missing_docs)]
	July = 7,
	
	#[allow(missing_docs)]
	August = 8,
	
	#[allow(missing_docs)]
	September = 9,
	
	#[allow(missing_docs)]
	October = 10,
	
	#[allow(missing_docs)]
	November = 11,
	
	#[allow(missing_docs)]
	December = 12,
}

impl TryFrom<NonZeroU8> for Month
{
	type Error = TooLargeError<NonZeroU8>;
	
	#[inline(always)]
	fn try_from(value: NonZeroU8) -> Result<Self, Self::Error>
	{
		const InclusiveMaximum: NonZeroU8 = new_non_zero_u8(12);
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

impl Month
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn number_of_days_in_month(self, is_leap_year: bool) -> NonZeroU8
	{
		use Month::*;
		let value = match self
		{
			January => 31,
			
			February => if is_leap_year
			{
				29
			}
			else
			{
				28
			},
			
			March => 31,
			
			April => 30,
			
			May => 31,
			
			June => 30,
			
			July => 31,
			
			August => 31,
			
			September => 30,
			
			October => 31,
			
			November => 30,
			
			December => 31,
		};
		new_non_zero_u8(value)
	}
}
