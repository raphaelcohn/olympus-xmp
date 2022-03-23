// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// PLUS licensor telephone type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum PlusLicensorTelephoneType
{
	/// Work.
	///
	/// `http://ns.useplus.org/ldf/vocab/work`.
	work,
	
	/// Not Applicable.
	///
	/// `http://ns.useplus.org/ldf/vocab/cell`.
	cell,
	
	/// Unlimited Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/fax`.
	fax,
	
	/// Unlimited Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/home`.
	home,
	
	/// Limited or Incomplete Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/pager`.
	pager,
}

impl_xmp_attribute_value_parse_str_prefix!
(
	PlusLicensorTelephoneType, PlusLicensorTelephoneType, "http://ns.useplus.org/ldf/vocab/",
	
	"work" => work,
	
	"cell" => cell,
	
	"fax" => fax,
	
	"home" => home,
	
	"pager" => pager,
);
