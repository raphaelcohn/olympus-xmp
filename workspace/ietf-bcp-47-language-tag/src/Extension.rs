// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Start with `0-9`, `a-y` or `y-z`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Extension(OneWithOptionalSuffixes<ExtensionPortion>);

impl Extension
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn first(&self) -> ExtensionPortion
	{
		self.0.one()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn subsequent(&self) -> &[ExtensionPortion]
	{
		self.0.suffixes()
	}
	
	#[inline(always)]
	fn new(one: ExtensionPortion) -> Self
	{
		Self
		(
			OneWithOptionalSuffixes
			{
				one,
				
				suffixes: vec![]
			}
		)
	}
	
	#[inline(always)]
	fn push(&mut self, subsequent: ExtensionPortion)
	{
		self.0.suffixes.push(subsequent)
	}
}
