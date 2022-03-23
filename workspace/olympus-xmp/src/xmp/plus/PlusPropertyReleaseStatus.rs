// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// PLUS property release status.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum PlusPropertyReleaseStatus
{
	/// None.
	///
	/// `http://ns.useplus.org/ldf/vocab/PR-NON`.
	NoReleases,
	
	/// Not Applicable.
	///
	/// `http://ns.useplus.org/ldf/vocab/PR-NAP`.
	NotApplicable,
	
	/// Unlimited Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/PR-UPR`.
	Unlimited,
	
	/// Limited or Incomplete Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/PR-LPR`.
	LimitedOrIncomplete,
}

impl_xmp_attribute_value_parse_str_prefix!
(
	PlusPropertyReleaseStatus, PlusPropertyReleaseStatus, "http://ns.useplus.org/ldf/vocab/PR-",
	
	"NON" => NoReleases,
	
	"NAP" => NotApplicable,
	
	"UPR" => Unlimited,
	
	"LPR" => LimitedOrIncomplete,
);
