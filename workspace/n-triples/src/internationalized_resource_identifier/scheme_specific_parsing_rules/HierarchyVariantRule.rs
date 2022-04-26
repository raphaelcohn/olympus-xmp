// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) enum HierarchyVariantRule
{
	AuthorityAndAbsolutePathOnly,
	
	AuthorityAndAbsolutePathOrAbsolutePathOnly,
	
	RootlessOnly,
	
	Unknown,
}

impl HierarchyVariantRule
{
	#[inline(always)]
	const fn hierarchy_starts_with_slash(&self) -> bool
	{
		use HierarchyVariantRule::*;
		
		match self
		{
			AuthorityAndAbsolutePathOnly | AuthorityAndAbsolutePathOrAbsolutePathOnly => true,
			
			Unknown => false,
		}
	}
	
	#[inline(always)]
	const fn hierarchy_is_authority_and_absolute_path(&self) -> bool
	{
		use HierarchyVariantRule::*;
		
		match self
		{
			AuthorityAndAbsolutePathOnly => true,
			
			AuthorityAndAbsolutePathOrAbsolutePathOnly | Unknown => false,
		}
	}
}
