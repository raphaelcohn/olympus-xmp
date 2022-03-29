// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Language
{
	Arabic,
	
	Chinese,
	
	English,

	French,

	Russian,

	Spanish,
}

impl Language
{
	#[inline(always)]
	fn initial(self, names: &mut Names, non_empty_name: &'static str)
	{
		use Language::*;
		
		let field = match self
		{
			Arabic => &mut names.arabic,
			
			Chinese => &mut names.arabic,
			
			English => &mut names.arabic,
			
			French => &mut names.arabic,
			
			Russian => &mut names.arabic,
			
			Spanish => &mut names.arabic,
		};
		
		let was = *field;
		assert!(was.is_empty(), "Name was not previously empty but was {}", was);
		*field = non_empty_name;
	}
	
	#[inline(always)]
	fn older(self, names: &mut Names)
	{
	
	}
}
