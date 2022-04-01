// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Sorted in ascending numeric order set of M49 codes.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Constituents(IndexSet<M49Code>);

impl Deref for Constituents
{
	type Target = IndexSet<M49Code>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl From<&'static [M49Code]> for Constituents
{
	#[inline(always)]
	fn from(constituents: StaticConstituents) -> Self
	{
		let length = constituents.len();
		let mut index_set = IndexSet::with_capacity(length);
		
		match length
		{
			0 => (),
			
			1 =>
			{
				let _ = index_set.insert(constituents.get_unchecked_value_safe(0));
			}
			
			_ =>
			{
				let first = constituents.get_unchecked_value_safe(0);
				let _ = index_set.insert(first);
				
				let mut previous = first;
				for index in 1 .. length
				{
					let subsequent = constituents.get_unchecked_value_safe(index);
					assert!(subsequent <= previous);
					
					let was = index_set.insert(subsequent);
					debug_assert!(was.is_none());
					
					previous = subsequent;
				}
			}
		}
		Self(index_set)
	}
}
