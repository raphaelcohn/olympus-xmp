// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait Utf8SequenceParserUchar
{
	fn push_forcing_heap_UCHAR4<E: error::Error, UCP: FnOnce(UCHARParseError) -> E, OOM: FnOnce(TryReserveError) -> E>(&mut self, remaining_bytes: &mut &[u8], uchar_parse_error: UCP, out_of_memory: OOM) -> Result<(), E>;
	
	fn push_forcing_heap_UCHAR8<E: error::Error, UCP: FnOnce(UCHARParseError) -> E, OOM: FnOnce(TryReserveError) -> E>(&mut self, remaining_bytes: &mut &[u8], uchar_parse_error: UCP, out_of_memory: OOM) -> Result<(), E>;
	
	#[doc(hidden)]
	fn push_forcing_heap_UCHAR<E: error::Error, UCP: FnOnce(UCHARParseError) -> E, OOM: FnOnce(TryReserveError) -> E, PU: FnOnce(&mut &[u8]) -> Result<char, UCHARParseError>>(&mut self, remaining_bytes: &mut &[u8], uchar_parse_error: UCP, out_of_memory: OOM, parse_UCHAR: PU) -> Result<(), E>;
}

impl<'a> Utf8SequenceParserUchar for Utf8SequencesParser<'a>
{
	#[inline(always)]
	fn push_forcing_heap_UCHAR4<E: error::Error, UCP: FnOnce(UCHARParseError) -> E, OOM: FnOnce(TryReserveError) -> E>(&mut self, remaining_bytes: &mut &[u8], uchar_parse_error: UCP, out_of_memory: OOM) -> Result<(), E>
	{
		self.push_forcing_heap_UCHAR(remaining_bytes, uchar_parse_error, out_of_memory, UCHARParser::parse_UCHAR4)
	}
	
	#[inline(always)]
	fn push_forcing_heap_UCHAR8<E: error::Error, UCP: FnOnce(UCHARParseError) -> E, OOM: FnOnce(TryReserveError) -> E>(&mut self, remaining_bytes: &mut &[u8], uchar_parse_error: UCP, out_of_memory: OOM) -> Result<(), E>
	{
		self.push_forcing_heap_UCHAR(remaining_bytes, uchar_parse_error, out_of_memory, UCHARParser::parse_UCHAR8)
	}
	
	#[inline(always)]
	fn push_forcing_heap_UCHAR<E: error::Error, UCP: FnOnce(UCHARParseError) -> E, OOM: FnOnce(TryReserveError) -> E, PU: FnOnce(&mut &[u8]) -> Result<char, UCHARParseError>>(&mut self, remaining_bytes: &mut &[u8], uchar_parse_error: UCP, out_of_memory: OOM, parse_UCHAR: PU) -> Result<(), E>
	{
		let character = parse_UCHAR(remaining_bytes).map_err(uchar_parse_error)?;
		self.push_forcing_heap(character).map_err(out_of_memory)
	}
}
