// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// PLUS minor model age disclosure.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum PlusMinorModelAgeDisclosure
{
	/// Age unknown.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-UNK`.
	AgeUnknown,
	
	/// Age 25 or over.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A25`.
	Age25OrOver,
	
	/// Age 24.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A24`.
	Age24,
	
	/// Age 23.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A22`.
	Age23,
	
	/// Age 22.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A22`.
	Age22,
	
	/// Age 21.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A21`.
	Age21,
	
	/// Age 20.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A20`.
	Age20,
	
	/// Age 19.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A19`.
	Age19,
	
	/// Age 18.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A18`.
	Age18,
	
	/// Age 17.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A17`.
	Age17,
	
	/// Age 16.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A16`.
	Age16,
	
	/// Age 15.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-A15`.
	Age15,
	
	/// Age 14 or under.
	///
	/// `http://ns.useplus.org/ldf/vocab/AG-U14`.
	Age14OrUnder,
}

impl_xmp_attribute_value_parse_str_prefix!
(
	PlusMinorModelAgeDisclosure, PlusMinorModelAgeDisclosure, "http://ns.useplus.org/ldf/vocab/AG-",
	
	"UNK" => AgeUnknown,
	
	"A25" => Age25OrOver,
	
	"A24" => Age24,
	
	"A23" => Age23,
	
	"A22" => Age22,
	
	"A21" => Age21,
	
	"A20" => Age20,
	
	"A19" => Age19,
	
	"A18" => Age18,
	
	"A17" => Age17,
	
	"A16" => Age16,
	
	"A15" => Age15,
	
	"U14" => Age14OrUnder,
);
