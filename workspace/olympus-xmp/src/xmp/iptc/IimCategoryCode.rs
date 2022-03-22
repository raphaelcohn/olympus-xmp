// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Obsolete as of IIM version 4 (extant as "Datasets 2:15 Category in IIM version 3).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IimCategoryCode
{
	#[allow(missing_docs)]
	ACE,
	
	#[allow(missing_docs)]
	CLJ,
	
	#[allow(missing_docs)]
	DIS,
	
	#[allow(missing_docs)]
	EDU,
	
	#[allow(missing_docs)]
	EVN,
	
	#[allow(missing_docs)]
	FIN,
	
	#[allow(missing_docs)]
	HTH,
	
	#[allow(missing_docs)]
	HUM,
	
	#[allow(missing_docs)]
	LAB,
	
	#[allow(missing_docs)]
	LIF,
	
	#[allow(missing_docs)]
	POL,
	
	#[allow(missing_docs)]
	REL,
	
	#[allow(missing_docs)]
	SCI,
	
	#[allow(missing_docs)]
	SOI,
	
	#[allow(missing_docs)]
	SPO,
	
	#[allow(missing_docs)]
	WAR,
	
	#[allow(missing_docs)]
	WEA,
}

impl<'a> XmpAttributeValue<'a> for IimCategoryCode
{
	type Error = IimCategoryCodeParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		use IimCategoryCodeParseError::*;
		
		if raw.len() != 3
		{
			return Err(LengthIsNotThree)
		}
		
		use IimCategoryCode::*;
		let bytes = raw.as_bytes();
		match bytes.get_unchecked_value_safe(0)
		{
			A => Self::parse_last_two_alpha_one_choice(bytes, (ACE, b"CE")),
			
			C => Self::parse_last_two_alpha_one_choice(bytes, (CLJ, b"LJ")),
			
			E => Self::parse_last_two_alpha_two_choices(bytes, (EDU, b"DU"), (EVN, b"VN")),
			
			H => Self::parse_last_two_alpha_two_choices(bytes, (HTH, b"TH"), (HUM, b"UM")),
			
			L => Self::parse_last_two_alpha_two_choices(bytes, (LAB, b"AB"), (LIF, b"IF")),
			
			P => Self::parse_last_two_alpha_one_choice(bytes, (POL, b"OL")),
			
			R => Self::parse_last_two_alpha_one_choice(bytes, (REL, b"EL")),
			
			S => Self::parse_last_two_alpha_three_choices(bytes, (SCI, b"CI"), (SOI, b"OI"), (SPO, b"PO")),
			
			W => Self::parse_last_two_alpha_two_choices(bytes, (WAR, b"AR"), (WEA, b"EA")),
			
			_ => Self::error(bytes),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::IimCategoryCode(error)
	}
}

impl IimCategoryCode
{
	const Length: usize = 3;
	
	const LastTwoLength: usize = Self::Length - 1;
	
	#[inline(always)]
	const fn parse_last_two_alpha_one_choice(bytes: &[u8], choice_1: (IimCategoryCode, Choice)) -> Result<Self, IimCategoryCodeParseError>
	{
		let u16_bytes = Self::to_u16_bytes(bytes);
		if u16_bytes == IimCategoryCode::to_u16_last_two(choice_1.1)
		{
			return Ok(choice_1.0)
		}
		else
		{
			Self::error(bytes)
		}
	}
	
	#[inline(always)]
	const fn parse_last_two_alpha_two_choices(bytes: &[u8], choice_1: (IimCategoryCode, Choice), choice_2: (IimCategoryCode, Choice)) -> Result<Self, IimCategoryCodeParseError>
	{
		let u16_bytes = Self::to_u16_bytes(bytes);
		if u16_bytes == IimCategoryCode::to_u16_last_two(choice_1.1)
		{
			return Ok(choice_1.0)
		}
		else if u16_bytes == IimCategoryCode::to_u16_last_two(choice_2.1)
		{
			return Ok(choice_2.0)
		}
		else
		{
			Self::error(bytes)
		}
	}
	
	#[inline(always)]
	const fn parse_last_two_alpha_three_choices(bytes: &[u8], choice_1: (IimCategoryCode, Choice), choice_2: (IimCategoryCode, Choice), choice_3: (IimCategoryCode, Choice)) -> Result<Self, IimCategoryCodeParseError>
	{
		let u16_bytes = Self::to_u16_bytes(bytes);
		if u16_bytes == IimCategoryCode::to_u16_last_two(choice_1.1)
		{
			return Ok(choice_1.0)
		}
		else if u16_bytes == IimCategoryCode::to_u16_last_two(choice_2.1)
		{
			return Ok(choice_2.0)
		}
		else if u16_bytes == IimCategoryCode::to_u16_last_two(choice_3.1)
		{
			return Ok(choice_3.0)
		}
		else
		{
			Self::error(bytes)
		}
	}
	
	#[inline(always)]
	const fn error(bytes: &[u8]) -> Result<Self, IimCategoryCodeParseError>
	{
		let pointer = bytes.as_ptr().cast::<[u8; Self::Length]>();
		Err(IimCategoryCodeParseError::UnknownThreeLetterCode(unsafe { pointer.read() }))
	}
	
	#[inline(always)]
	const fn to_u16_bytes(bytes: &[u8]) -> u16
	{
		let pointer = bytes.as_ptr();
		Self::to_u16(unsafe { pointer.add(1) })
	}
	
	#[inline(always)]
	const fn to_u16_last_two(last_two_choice: Choice) -> u16
	{
		Self::to_u16(last_two_choice.as_ptr())
	}
	
	#[inline(always)]
	const fn to_u16(pointer: *const u8) -> u16
	{
		let u16_pointer = pointer.cast::<u16>();
		unsafe { u16_pointer.read() }
	}
}
