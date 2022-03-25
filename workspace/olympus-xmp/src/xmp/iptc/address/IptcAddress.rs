// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::collections::HashMap;
use iso_3166_1_country::{Iso3166Dash1AlphaCountryCode, Iso3166Dash1Country};
use crate::xml_name;
use crate::xmp::non_empty_str::NonEmptyStr;
use crate::xmp::{XmpElement, XmpValidationError};
use crate::xmp::iptc::IptcWorldRegion;
use super::super::namespaces::Iptc4xmpExt;

pub struct IptcAddress<'a>
{
	sublocation: NonEmptyStr<'a>,
	
	city: NonEmptyStr<'a>,
	
	province_state: NonEmptyStr<'a>,
}

impl<'a> IptcAddress<'a>
{
	// <rdf:li
	// Iptc4xmpExt:Sublocation="Addingham Churchyard"
	// Iptc4xmpExt:City="Addingham"
	// Iptc4xmpExt:ProvinceState="North Yorkshire"	TODO: UK province validator?
	// Iptc4xmpExt:CountryName="United Kingdom of Great Britain and Northern Ireland (the)"	DONE ISO 3166 parser
	// Iptc4xmpExt:CountryCode="GBR" DONE: ISO 3166 parser
	// Iptc4xmpExt:WorldRegion="Europe"/>	DONE: region parser
	pub fn parse<'name, 'namespace, 'local_name>(li_element: &XmpElement<'a, 'name, 'namespace, 'local_name>) -> Result<Self, IptcAddressParseError<'name, 'namespace, 'local_name>>
	{
		use IptcAddressParseError::*;
		
		// if !xmp_element.has_name(xml_name!(Iptc4xmpExt, "li"))
		// {
		// 	return Err(IsNotLiXmlElement)
		// }
		
		let mut attribute_parse_failures = IptcAddressAttributeParseFailures::default();
		
		let sublocation = attribute_parse_failures.parse_attribute::<NonEmptyStr, "Sublocation">(li_element);
		let city = attribute_parse_failures.parse_attribute::<NonEmptyStr, "City">(li_element);
		let province_state = attribute_parse_failures.parse_attribute::<NonEmptyStr, "ProvinceState">(li_element);
		let country_name = attribute_parse_failures.parse_attribute::<Iso3166Dash1Country, "CountryName">(li_element);
		let country_code = attribute_parse_failures.parse_attribute::<Iso3166Dash1AlphaCountryCode, "CountryCode">(li_element);
		let world_region = attribute_parse_failures.parse_attribute::<IptcWorldRegion, "WorldRegion">(li_element);
		
		if attribute_parse_failures.has_failures()
		{
			return Err(IptcAddressParseError::AttributeParseFailures(attribute_parse_failures))
		}
		
		
		Ok
		(
			Self
			{
				sublocation: unsafe { sublocation.unwrap_unchecked() },
				city: unsafe { city.unwrap_unchecked() },
				province_state: unsafe { province_state.unwrap_unchecked() },
			}
		)
	}
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct IptcAddressAttributeParseFailures<'name, 'namespace, 'local_name>(HashMap<&'static str, XmpValidationError<'name, 'namespace, 'local_name>>);

impl<'name, 'namespace, 'local_name> IptcAddressAttributeParseFailures<'name, 'namespace, 'local_name>
{
	#[inline(always)]
	fn has_failures(&self) -> bool
	{
		!self.0.is_empty()
	}
	
	#[inline(always)]
	fn parse_attribute<'a, XAV: XmpAttributeValue<'a>, const attribute_name: &'static str>(&mut self, xmp_element: &XmpElement<'a, 'name, 'namespace, 'local_name>) -> Option<XAV>
	{
		match xmp_element.get_attribute_or_error::<XAV>(xml_name!(Iptc4xmpExt, attribute_name))
		{
			Ok(attribute) => Some(attribute),
			
			Err(error) =>
			{
				let inserted = self.0.insert(attribute_name, error);
				debug_assert_eq!(inserted, true);
				None
			}
		}
		
	}
}
