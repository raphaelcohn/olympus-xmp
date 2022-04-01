// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Status of a country.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Developing
{
	#[allow(missing_docs)]
	pub least_developed_countries: bool,
	
	#[allow(missing_docs)]
	pub land_locked_developing_countries: bool,
	
	#[allow(missing_docs)]
	pub small_island_developing_states: bool,
}

impl const Default for Developing
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			least_developed_countries: false,
			land_locked_developing_countries: false,
			small_island_developing_states: false
		}
	}
}
