// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A rating.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(i8)]
pub enum XmpRating
{
	#[allow(missing_docs)]
	Rejected = -1,

	#[allow(missing_docs)]
	Unrated = 0,

	#[allow(missing_docs)]
	_1 = 1,

	#[allow(missing_docs)]
	_2 = 2,

	#[allow(missing_docs)]
	_3 = 3,

	#[allow(missing_docs)]
	_4 = 4,
	
	#[allow(missing_docs)]
	_5 = 5,
}

impl_xmp_attribute_value_parse_transmute_i8!(XmpRating, XmpRating, -1 ..= 5);
