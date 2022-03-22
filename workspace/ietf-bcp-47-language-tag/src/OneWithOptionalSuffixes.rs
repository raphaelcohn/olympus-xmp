// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// One item with optional suffixes of a type.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct OneWithOptionalSuffixes<T>
{
	one: T,
	
	suffixes: Vec<T>,
}

impl<T> OneWithOptionalSuffixes<T>
{
	#[inline(always)]
	const fn one(&self) -> T
	where T: Copy
	{
		self.one
	}
	
	#[inline(always)]
	fn suffixes(&self) -> &[T]
	{
		self.suffixes.as_slice()
	}
	
	#[inline(always)]
	fn without_suffixes(one: T) -> Self
	{
		Self
		{
			one,
			
			suffixes: Vec::new(),
		}
	}
}
