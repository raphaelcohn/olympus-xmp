// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct NameAndM49Code
{
	pub(super) name: &'static str,
	
	pub(super) m49_code: M49Code
}

impl NameAndM49Code
{
	#[inline(always)]
	pub(super) fn new(name: &'static str, m49_code: M49Code) -> Self
	{
		assert!(!name.is_empty(), "Name is empty");
		
		Self
		{
			name,
		
			m49_code,
		}
	}
	
	#[inline(always)]
	pub(super) fn is_sark(self) -> bool
	{
		self.m49_code.is_sark()
	}
	
	#[inline(always)]
	pub(super) fn non_empty_name(self) -> &'static str
	{
		let name = self.name;
		assert!(!name.is_empty(), "Name is empty");
		name
	}
}
