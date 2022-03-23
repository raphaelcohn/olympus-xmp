// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// PLUS model release status.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum PlusModelReleaseStatus
{
	/// None.
	///
	/// `http://ns.useplus.org/ldf/vocab/MR-NON`.
	NoReleases,
	
	/// Not Applicable.
	///
	/// `http://ns.useplus.org/ldf/vocab/MR-NAP`.
	NotApplicable,
	
	/// Unlimited Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/MR-UMR`.
	Unlimited,
	
	/// Limited or Incomplete Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/MR-LMR`.
	LimitedOrIncomplete,
}

impl_xmp_attribute_value_parse_str_prefix!
(
	PlusModelReleaseStatus, PlusModelReleaseStatus, "http://ns.useplus.org/ldf/vocab/MR-",
	
	"NON" => NoReleases,
	
	"NAP" => NotApplicable,
	
	"UMR" => Unlimited,
	
	"LMR" => LimitedOrIncomplete,
);
