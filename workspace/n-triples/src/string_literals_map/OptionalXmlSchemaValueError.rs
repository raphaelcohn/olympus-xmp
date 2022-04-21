// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionalXmlSchemaValueError<StrParseError: error::Error>
{
	#[allow(missing_docs)]
	TooMany
	{
		cause: MoreThanOneError,
		
		xml_schema_value_kind: XmlSchemaValueKind,
	},
	
	#[allow(missing_docs)]
	StrParse
	{
		cause: StrParseError,
		
		xml_schema_value_kind: XmlSchemaValueKind,
	},
}

impl<StrParseError: error::Error> Display for OptionalXmlSchemaValueError<StrParseError>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl<StrParseError: 'static + error::Error> error::Error for OptionalXmlSchemaValueError<StrParseError>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use OptionalXmlSchemaValueError::*;
		
		match self
		{
			TooMany { cause, .. } => Some(cause),
			
			StrParse { cause, .. } => Some(cause),
		}
	}
}
