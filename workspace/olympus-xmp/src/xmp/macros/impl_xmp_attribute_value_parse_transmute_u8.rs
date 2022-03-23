// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


macro_rules! impl_xmp_attribute_value_parse_transmute_u8
{
	($attribute: ty, $error_enum: ident, $range: pat) =>
	{
		impl_xmp_attribute_value_parse_transmute!($attribute, $error_enum, U8ParseError, u8, $range);
	}
}
