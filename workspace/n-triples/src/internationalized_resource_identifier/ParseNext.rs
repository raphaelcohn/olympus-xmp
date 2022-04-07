// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum ParseNext<'a>
{
	Query
	{
		remaining_utf8_bytes: &'a [u8],
	},
	
	Fragment
	{
		remaining_utf8_bytes: &'a [u8],
		
		query: Option<()>,
	},
	
	End
	{
		query: Option<()>,
		
		fragment: Option<()>,
	},
}

impl<'a> ParseNext<'a>
{
	const NoQueryNoFragment: Self = ParseNext::End { query: none, fragment: None };
	
	#[inline(always)]
	const fn query(remaining_utf8_bytes: &'a [u8]) -> Self
	{
		ParseNext::Query { remaining_utf8_bytes }
	}
	
	#[inline(always)]
	const fn fragment_no_query(remaining_utf8_bytes: &'a [u8]) -> Self
	{
		ParseNext::Fragment { remaining_utf8_bytes, query: None }
	}
}
