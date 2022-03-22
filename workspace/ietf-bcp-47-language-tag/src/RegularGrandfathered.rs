// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// BCP 47, 2.1 Syntax: "these tags match the 'langtag' production, but their subtags are not extended language or variant subtags: their meaning is defined by their registration and all of these are deprecated in favor of a more modern subtag or sequence of subtags".
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum RegularGrandfathered
{
	#[allow(missing_docs)]
	art_lojban,
	
	#[allow(missing_docs)]
	cel_gaulish,
	
	#[allow(missing_docs)]
	no_bok,
	
	#[allow(missing_docs)]
	no_nyn,
	
	#[allow(missing_docs)]
	zh_guoyu,
	
	#[allow(missing_docs)]
	zh_hakka,
	
	#[allow(missing_docs)]
	zh_min,
	
	#[allow(missing_docs)]
	zh_min_nan,
	
	#[allow(missing_docs)]
	zh_xiang,
}
