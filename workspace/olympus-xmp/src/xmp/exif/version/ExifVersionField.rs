// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A field's value.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ExifVersionField
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

impl Default for ExifVersionField
{
	#[inline(always)]
	fn default() -> Self
	{
		_0
	}
}

impl ExifVersionField
{
	#[inline(always)]
	fn parse<const field_index: u8>(bytes: &[u8]) -> Result<Self, ExifVersionFieldParseError>
	{
		use ExifVersionFieldParseError::*;
		
		const BeforeDigit0: u8 = Digit0 - 1;
		const Digit0: u8 = 0x30;
		const Digit9: u8 = 0x39;
		const AfterDigit9: u8 = Digit9 + 1;
		let byte = bytes.get_unchecked_value_safe(field_index);
		match byte
		{
			0x00 ..= BeforeDigit0 => Err(FieldIsNotDecimalDigit(byte)),
			
			Digit0..= Digit9 => Ok(unsafe { transmute(byte - Digit0) }),
			
			AfterDigit9 ..= 0x7F => Err(FieldIsNotDecimalDigit(byte)),
			
			_ => Err(FieldIsNotAscii(byte)),
		}
	}
}
