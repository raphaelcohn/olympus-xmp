// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnlyOneXmlSchemaValueError<StrParseError: error::Error>
{
	#[allow(missing_docs)]
	Missing
	{
		xml_schema_value_kind: XmlSchemaValueKind,
	},
	
	#[allow(missing_docs)]
	Optional(OptionalXmlSchemaValueError<StrParseError>),
}

impl<StrParseError: error::Error> Display for OnlyOneXmlSchemaValueError<StrParseError>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl<StrParseError: 'static + error::Error> error::Error for OnlyOneXmlSchemaValueError<StrParseError>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use OnlyOneXmlSchemaValueError::*;
		
		match self
		{
			Optional(cause) => Some(cause),
			
			_ => None,
		}
	}
}
