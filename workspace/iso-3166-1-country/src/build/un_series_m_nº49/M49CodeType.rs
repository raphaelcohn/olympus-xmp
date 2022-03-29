// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(super) enum M49CodeType
{
	Global(Names),
	
	Region(Names),
	
	SubRegion(Names),
	
	IntermediateRegion(Names),
	
	Country(Country),

	PrivateUse,
	
	ObsoleteSubRegion
	{
		names: Names,
		
		replacements: &'static [M49Code],
	},
	
	ObsoleteOtherGrouping
	{
		names: Names,
	},
}
