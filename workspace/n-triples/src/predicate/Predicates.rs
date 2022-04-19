// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Convenience.
pub trait Predicates<'a: 'string_literals_map, 'string_literals_map, 'predicate>
{
	#[allow(missing_docs)]
	#[inline(always)]
	fn get_only_one_xml_schema_string(&'string_literals_map self, predicate: Predicate<'predicate>) -> Result<&'string_literals_map str, OnlyOneXmlSchemaStringLiteralError<'predicate>>
	{
		self.get_string_literals_by_absolute_internationalized_resource_identifier(&predicate).xml_schema_string().map_err(|error| OnlyOneXmlSchemaStringLiteralError::String(error, predicate))
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn get_only_one_xml_schema_boolean(&'string_literals_map self, predicate: Predicate<'predicate>) -> Result<bool, OnlyOneXmlSchemaStringLiteralError<'predicate>>
	{
		self.get_string_literals_by_absolute_internationalized_resource_identifier(&predicate).xml_schema_boolean().map_err(|error| OnlyOneXmlSchemaStringLiteralError::Boolean(error, predicate))
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn get_only_one_xml_schema_integer(&'string_literals_map self, predicate: Predicate<'predicate>) -> Result<i64, OnlyOneXmlSchemaStringLiteralError<'predicate>>
	{
		self.get_string_literals_by_absolute_internationalized_resource_identifier(&predicate).xml_schema_integer().map_err(|error| OnlyOneXmlSchemaStringLiteralError::Integer(error, predicate))
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn get_only_one_xml_schema_date_time(&'string_literals_map self, predicate: Predicate<'predicate>) -> Result<DateTime<FixedOffset>, OnlyOneXmlSchemaStringLiteralError<'predicate>>
	{
		self.get_string_literals_by_absolute_internationalized_resource_identifier(&predicate).xml_schema_date_time().map_err(|error| OnlyOneXmlSchemaStringLiteralError::DateTime(error, predicate))
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn get_optional_xml_schema_string(&'string_literals_map self, predicate: Predicate<'predicate>) -> Result<Option<&'string_literals_map str>, OptionalXmlSchemaStringLiteralError<'predicate>>
	{
		self.get_string_literals_by_absolute_internationalized_resource_identifier(&predicate).optional_xml_schema_string().map_err(|error| OptionalXmlSchemaStringLiteralError::String(error, predicate))
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn get_optional_xml_schema_boolean(&'string_literals_map self, predicate: Predicate<'predicate>) -> Result<Option<bool>, OptionalXmlSchemaStringLiteralError<'predicate>>
	{
		self.get_string_literals_by_absolute_internationalized_resource_identifier(&predicate).optional_xml_schema_boolean().map_err(|error| OptionalXmlSchemaStringLiteralError::Boolean(error, predicate))
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn get_optional_xml_schema_integer(&'string_literals_map self, predicate: Predicate<'predicate>) -> Result<Option<i64>, OptionalXmlSchemaStringLiteralError<'predicate>>
	{
		self.get_string_literals_by_absolute_internationalized_resource_identifier(&predicate).optional_xml_schema_integer().map_err(|error| OptionalXmlSchemaStringLiteralError::Integer(error, predicate))
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn get_optional_xml_schema_date_time(&'string_literals_map self, predicate: Predicate<'predicate>) -> Result<Option<DateTime<FixedOffset>>, OptionalXmlSchemaStringLiteralError<'predicate>>
	{
		self.get_string_literals_by_absolute_internationalized_resource_identifier(&predicate).optional_xml_schema_date_time().map_err(|error| OptionalXmlSchemaStringLiteralError::DateTime(error, predicate))
	}
	
	#[allow(missing_docs)]
	fn get_optional_absolute_internationalized_resource_identifier(&'string_literals_map self, predicate: &Predicate<'predicate>) -> Result<Option<&'string_literals_map AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>, MoreThanOneError>;
	
	#[allow(missing_docs)]
	fn get_absolute_internationalized_resource_identifiers(&'string_literals_map self, predicate: &Predicate<'predicate>) -> &[AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>];
	
	#[allow(missing_docs)]
	fn get_blank_nodes(&'string_literals_map self, predicate: &Predicate<'predicate>) -> &'string_literals_map [BlankNodeLabel<'a>];
	
	#[allow(missing_docs)]
	fn get_string_literals_by_language(&'string_literals_map self, predicate: &Predicate<'predicate>) -> &'string_literals_map StringLiteralsMap<'a, NaiveIetfBcp47LanguageTag<'a>>;
	
	#[allow(missing_docs)]
	fn get_string_literals_by_absolute_internationalized_resource_identifier(&'string_literals_map self, predicate: &Predicate<'predicate>) -> &'string_literals_map StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>;
}

impl<'a: 'string_literals_map, 'string_literals_map, 'predicate> Predicates<'a, 'string_literals_map, 'predicate> for HashMap<Predicate<'a>, Objects<'a>>
{
	#[inline(always)]
	fn get_optional_absolute_internationalized_resource_identifier(&'string_literals_map self, predicate: &Predicate<'predicate>) -> Result<Option<&'string_literals_map AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>, MoreThanOneError>
	{
		match self.get(unsafe { transmute(predicate) })
		{
			None => Ok(None),
			
			Some(objects) =>
			{
				let x = objects.absolute_internationalized_resource_identifiers();
				let length = x.len();
				if length == 1
				{
					Ok(Some(x.get_unchecked_safe(0)))
				}
				else
				{
					Err(MoreThanOneError { count: new_non_zero_usize(length) })
				}
			}
		}
	}
	
	#[inline(always)]
	fn get_absolute_internationalized_resource_identifiers(&'string_literals_map self, predicate: &Predicate<'predicate>) -> &'string_literals_map [AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>]
	{
		const Empty: &'static [AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>] = &[];
		
		match self.get(unsafe { transmute(predicate) })
		{
			// This seems like a bug in Rust to do with with const generics.
			None => unsafe { transmute(Empty) },
			
			Some(objects) => objects.absolute_internationalized_resource_identifiers()
		}
	}
	
	#[inline(always)]
	fn get_blank_nodes(&'string_literals_map self, predicate: &Predicate<'predicate>) -> &'string_literals_map [BlankNodeLabel<'a>]
	{
		const Empty: &'static [BlankNodeLabel<'static>] = &[];
		
		match self.get(unsafe { transmute(predicate) })
		{
			None => Empty,
			
			Some(objects) => objects.blank_nodes()
		}
	}
	
	#[inline(always)]
	fn get_string_literals_by_language(&'string_literals_map self, predicate: &Predicate<'predicate>) -> &'string_literals_map StringLiteralsMap<'a, NaiveIetfBcp47LanguageTag<'a>>
	{
		static Empty: SyncLazy<StringLiteralsMap<'static, NaiveIetfBcp47LanguageTag<'static>>> = SyncLazy::new(StringLiteralsMap::default);
		
		match self.get(unsafe { transmute(predicate) })
		{
			None => unsafe { transmute(Empty.deref()) },
			
			Some(objects) => objects.string_literals_by_language(),
		}
	}
	
	#[inline(always)]
	fn get_string_literals_by_absolute_internationalized_resource_identifier(&'string_literals_map self, predicate: &Predicate<'predicate>) -> &'string_literals_map StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>
	{
		static Empty: SyncLazy<StringLiteralsMap<'static, AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>>> = SyncLazy::new(StringLiteralsMap::default);
		
		match self.get(unsafe { transmute(predicate) })
		{
			// This seems like a bug in Rust to do with with const generics.
			None => unsafe { transmute(Empty.deref()) },
			
			Some(objects) => objects.string_literals_by_absolute_internationalized_resource_identifier(),
		}
	}
}
