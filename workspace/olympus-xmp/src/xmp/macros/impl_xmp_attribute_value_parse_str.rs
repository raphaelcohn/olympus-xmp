// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


macro_rules! impl_xmp_attribute_value_parse_str
{
	($attribute: ty, $error_enum: ident, $($text: pat => $enum_member: ident, )*) =>
	{
		impl<'a> XmpAttributeValue<'a> for $attribute
		{
			type Error = UnknownStringVariantParseError;
			
			#[inline(always)]
			fn parse(raw: &'a str) -> Result<Self, Self::Error>
			{
				match raw
				{
					$(
						$text => Ok(<$attribute>::$enum_member),
					)*
					
					_ => Err(UnknownStringVariantParseError::from(raw)),
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
