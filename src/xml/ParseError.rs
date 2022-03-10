// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Parse error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError
{
	#[allow(missing_docs)]
	Read(XmlReadError),
	
	#[allow(missing_docs)]
	EndDocumentWithoutRootNode(TextPosition),
	
	#[allow(missing_docs)]
	StartElementAfterRoot(TextPosition, XmlElementCommon),
	
	#[allow(missing_docs)]
	MismatchedEndElementName(TextPosition, XmlElementCommon, OwnedName),
	
	#[allow(missing_docs)]
	EndDocumentBeforeEndElement(TextPosition),
	
	#[allow(missing_docs)]
	TextBeforeRoot(TextPosition, String),
	
	#[allow(missing_docs)]
	TextAfterRoot(TextPosition, String),
	
	#[allow(missing_docs)]
	DuplicateAttribute(TextPosition),
}

impl From<XmlReadError> for ParseError
{
	#[inline(always)]
	fn from(error: XmlReadError) -> Self
	{
		ParseError::Read(error)
	}
}

impl Display for ParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use ParseError::*;
		match self
		{
			Read(error) => Some(error),
			
			_ => None,
		}
	}
}

impl ParseError
{
	/// Position in XML error occurred at.
	#[inline(always)]
	pub fn position(&self) -> TextPosition
	{
		use ParseError::*;
		match self
		{
			Read(error) => error.position(),
			
			EndDocumentWithoutRootNode(position) => *position,
			
			StartElementAfterRoot(position, _) => *position,
			
			MismatchedEndElementName(position, _, _) => *position,
			
			EndDocumentBeforeEndElement(position) => *position,
			
			TextBeforeRoot(position, _) => *position,
			
			TextAfterRoot(position, _) => *position,
			
			DuplicateAttribute(position) => *position,
		}
	}
}