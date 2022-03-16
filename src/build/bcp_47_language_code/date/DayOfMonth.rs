// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A day of a month.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
enum DayOfMonth
{
	_1 = 1,
	
	_2 = 2,
	
	_3 = 3,
	
	_4 = 4,
	
	_5 = 5,
	
	_6 = 6,
	
	_7 = 7,
	
	_8 = 8,
	
	_9 = 9,
	
	_10 = 10,
	
	_11 = 11,
	
	_12 = 12,
	
	_13 = 13,
	
	_14 = 14,
	
	_15 = 15,
	
	_16 = 16,
	
	_17 = 17,
	
	_18 = 18,
	
	_19 = 19,
	
	_20 = 20,
	
	_21 = 21,
	
	_22 = 22,
	
	_23 = 23,
	
	_24 = 24,
	
	_25 = 25,
	
	_26 = 26,
	
	_27 = 27,
	
	_28 = 28,
	
	_29 = 29,
	
	_30 = 30,
	
	_31 = 31,
}

impl DayOfMonth
{
	fn from_str(s: &str, year: Year, month: Month) -> Result<Self, DateParseError>
	{
		use DateParseError::*;
		use DayOfMonth::*;
		use Month::*;
		
		match s
		{
			"01" => Ok(_1),
			
			"02" => Ok(_2),
			
			"03" => Ok(_3),
			
			"04" => Ok(_4),
			
			"05" => Ok(_5),
			
			"06" => Ok(_6),
			
			"07" => Ok(_7),
			
			"08" => Ok(_8),
			
			"09" => Ok(_9),
			
			"10" => Ok(_10),
			
			"11" => Ok(_11),
			
			"12" => Ok(_12),
			
			"13" => Ok(_13),
			
			"14" => Ok(_14),
			
			"15" => Ok(_15),
			
			"16" => Ok(_16),
			
			"17" => Ok(_17),
			
			"18" => Ok(_18),
			
			"19" => Ok(_19),
			
			"20" => Ok(_20),
			
			"21" => Ok(_21),
			
			"22" => Ok(_22),
			
			"23" => Ok(_23),
			
			"24" => Ok(_24),
			
			"25" => Ok(_25),
			
			"26" => Ok(_26),
			
			"27" => Ok(_27),
			
			"28" => Ok(_28),
			
			"29" => if month == Month::February
			{
				if year.is_leap_year()
				{
					Ok(_29)
				}
				else
				{
					Err(TooManyDaysForMonth)
				}
			}
			else
			{
				Ok(_29)
			},
			
			"30" => if month == February
			{
				Err(TooManyDaysForMonth)
			}
			else
			{
				Ok(_30)
			},
			
			"31" => match month
			{
				January | March | May | July | August | October | December => Ok(_31),
				
				_ => Err(TooManyDaysForMonth)
			},
			
			_ => if s.len() == 2
			{
				Err(DayOfMonthStringInvalid)
			}
			else
			{
				Err(DayOfMonthStringLengthNot2)
			}
		}
	}
}
