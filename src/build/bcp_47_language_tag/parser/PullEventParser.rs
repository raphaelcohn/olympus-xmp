// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) struct PullEventParser<'a>(UnfoldedLinesIterator<'a>);

impl<'a> Iterator for PullEventParser<'a>
{
	type Item = Result<Event<'a>, PullEventParserError>;
	
	fn next(&mut self) -> Option<Self::Item>
	{
		self.0.next().map(Self::split_line)
	}
}

impl<'a> PullEventParser<'a>
{
	#[inline(always)]
	pub(super) const fn new(buffer: &'a str) -> Self
	{
		Self(UnfoldedLinesIterator::new(buffer))
	}
	
	/// See RFC 5646, Section 3.1.1.
	fn split_line(line: Cow<'a, str>) -> Result<Event<'a>, PullEventParserError>
	{
		const Hyphen: u8 = b'-';
		const Percent: u8 = b'%';
		const Space: u8 = b' ';
		const Colon: u8 = b':';
		const MinimumLineLength: usize = 2;
		
		use Event::*;
		use PullEventParserError::*;
		
		fn parse_second_percent<'a>(length: usize, line_bytes: &[u8]) -> Result<Event<'a>, PullEventParserError>
		{
			if length != MinimumLineLength
			{
				Err(PercentNotFollowedByPercent)
			}
			else if line_bytes.get_unchecked_value_safe(1) != Percent
			{
				Err(PercentNotFollowedByPercent)
			}
			else
			{
				Ok(NewRecord)
			}
		}
		
		fn check_previous_byte_is_not_hyphen(line_bytes: &[u8], index: usize) -> Result<(), PullEventParserError>
		{
			let previous_byte = line_bytes.get_unchecked_value_safe(index - 1);
			if previous_byte == Hyphen
			{
				Err(FieldNameEndsWithHyphen)
			}
			else
			{
				Ok(())
			}
		}
		
		fn consume_spaces_until_colon(line_bytes: &[u8], index: usize, length: usize) -> Result<usize, PullEventParserError>
		{
			let mut index = index + 1;
			loop
			{
				if index == length
				{
					return Err(LineDoesNotContainColon)
				}
				
				match line_bytes.get_unchecked_value_safe(index)
				{
					Colon => return Ok(index),
					
					Space => index += 1,
					
					_ => return Err(LineDoesNotContainColon),
				}
			}
		}
		
		fn find_colon(line_bytes: &[u8], length: usize) -> Result<(usize, usize), PullEventParserError>
		{
			let mut index = 1;
			loop
			{
				if index == length
				{
					return Err(LineDoesNotContainColon)
				}
				
				match line_bytes.get_unchecked_value_safe(index)
				{
					_0 ..= _9 | A ..= Z | a ..= z | Hyphen =>
					{
						index += 1;
					}
					
					Space =>
					{
						check_previous_byte_is_not_hyphen(line_bytes, index)?;
						let colon_index = consume_spaces_until_colon(line_bytes, index, length)?;
						return Ok((index, colon_index))
					}
					
					Colon => return Ok((index, index)),
					
					_ => return Err(FieldNameContainsSomethingOtherThanAlphaDigitOrHyphen),
				}
			}
		}
		
		fn consume_spaces_after_colon(line_bytes: &[u8], length: usize, colon_index: usize) -> usize
		{
			let mut index = colon_index + 1;
			while index != length
			{
				if line_bytes.get_unchecked_value_safe(index) != Space
				{
					break
				}
				index += 1;
			}
			index
		}
		
		let line_bytes = line.as_ref().as_bytes();
		let length = line_bytes.len();
		
		let (field_name_exclusive_end_index, colon_index) =
		{
			if length < MinimumLineLength
			{
				return Err(LineTooShort)
			}
			
			let byte = line_bytes.get_unchecked_value_safe(0);
			match byte
			{
				Percent => return parse_second_percent(length, line_bytes),
				
				_0 ..= _9 | A ..= Z | a ..= z => find_colon(line_bytes, length)?,
				
				_ => return Err(LineDoesNotStartWithAlphaOrDigitOrPercent),
			}
		};
		
		let field_body_inclusive_start_index = consume_spaces_after_colon(line_bytes, length, colon_index);
		
		Ok
		(
			Field
			(
				FieldEvent
				{
					line,
					field_name_exclusive_end_index,
					field_body_inclusive_start_index,
				}
			)
		)
	}
}
