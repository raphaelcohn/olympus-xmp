// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


macro_rules! impl_xmp_attribute_value_parse_transmute
{
	($attribute: ty, $error_enum: ident, $error_type: ident, $integer: ty, $range: pat) =>
	{
		impl<'a> XmpAttributeValue<'a> for $attribute
		{
			type Error = $crate::xmp::attribute_parse_errors::$error_type;
			
			#[allow(unused_parens)]
			#[inline(always)]
			fn parse(value: &'a str) -> Result<Self, Self::Error>
			{
				let value = <$integer>::from_str(value)?;
				match value
				{
					$range => Ok(unsafe { transmute(value) }),
					
					_ => Err($crate::xmp::attribute_parse_errors::$error_type::InvalidValue(value)),
				}
			}
			
			#[inline(always)]
			fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
			{
				XmpAttributeValueParseError::$error_enum(error)
			}
		}
	}
}
