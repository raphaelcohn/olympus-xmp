// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// IPTC world region.
///
/// One of the values listed in <https://cv.iptc.org/newscodes/worldregion/>.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum IptcWorldRegion
{
	#[allow(missing_docs)]
	World,
	
	#[allow(missing_docs)]
	Africa,
	
	#[allow(missing_docs)]
	SouthAmerica,
	
	#[allow(missing_docs)]
	Oceania,
	
	#[allow(missing_docs)]
	NorthAmerica,
	
	#[allow(missing_docs)]
	Asia,
	
	#[allow(missing_docs)]
	Europe,
	
	#[allow(missing_docs)]
	Antarctica,
}

impl_xmp_attribute_value_parse_str!
(
	IptcWorldRegion, IptcWorldRegion,

	"World" => World,
	
	"Africa" => Africa,
	
	"South America" => SouthAmerica,
	
	"Oceania" => Oceania,
	
	"North America" => NorthAmerica,
	
	"Asia" => Asia,
	
	"Europe" => Europe,
	
	"Antarctica" => Antarctica,
);
