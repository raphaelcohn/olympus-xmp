// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum ParseNextAfterHierarchy<'a>
{
	Query
	{
		remaining: &'a str,
	},
	
	NoQueryFragment
	{
		remaining: &'a str,
	},
	
	NoQueryNoFragment,
}

impl<'a> ParseNextAfterHierarchy<'a>
{
	#[inline(always)]
	const fn query(remaining: &'a str) -> Self
	{
		ParseNextAfterHierarchy::Query { remaining }
	}
	
	#[inline(always)]
	const fn fragment_no_query(remaining: &'a str) -> Self
	{
		ParseNextAfterHierarchy::NoQueryFragment { remaining }
	}
}
