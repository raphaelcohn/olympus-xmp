// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Represents an `IRIREF`.
///
/// This is raw; it is not validated according to RFC 3987.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AbsoluteInternationalizedResourceIdentifier<'a>(Cow<'a, str>);

impl<'a> From<AbsoluteInternationalizedResourceIdentifier<'a>> for Cow<'a, str>
{
	#[inline(always)]
	fn from(value: AbsoluteInternationalizedResourceIdentifier<'a>) -> Self
	{
		value.0
	}
}

impl<'a> const From<Cow<'a, str>> for AbsoluteInternationalizedResourceIdentifier<'a>
{
	#[inline(always)]
	fn from(string: Cow<'a, str>) -> Self
	{
		Self(string)
	}
}

impl<'a> const From<String> for AbsoluteInternationalizedResourceIdentifier<'a>
{
	#[inline(always)]
	fn from(string: String) -> Self
	{
		Self(Cow::Owned(string))
	}
}

impl<'a> const From<&'a str> for AbsoluteInternationalizedResourceIdentifier<'a>
{
	#[inline(always)]
	fn from(string: &'a str) -> Self
	{
		Self(Cow::Borrowed(string))
	}
}

impl<'a> TryToOwnInPlace for AbsoluteInternationalizedResourceIdentifier<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		try_to_own_in_place_cow(&mut self.0)
	}
}

impl<'a> TryToOwn for AbsoluteInternationalizedResourceIdentifier<'a>
{
	type TryToOwned = AbsoluteInternationalizedResourceIdentifier<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl AbsoluteInternationalizedResourceIdentifier<'static>
{
	#[allow(missing_docs)]
	pub const OwlDeprecated: Self = Self::from("http://www.w3.org/2002/07/owl#deprecated");
	
	#[allow(missing_docs)]
	pub const SimpleKnowledgeOrganizationSchemeCoreNarrower: Self = Self::from("http://www.w3.org/2004/02/skos/core#narrower");
	
	#[allow(missing_docs)]
	pub const SimpleKnowledgeOrganizationSchemeCoreNotation: Self = Self::from("http://www.w3.org/2004/02/skos/core#notation");
	
	#[allow(missing_docs)]
	pub const SimpleKnowledgeOrganizationSchemeCorePrefLabel: Self = Self::from("http://www.w3.org/2004/02/skos/core#prefLabel");
	
	#[allow(missing_docs)]
	pub const XmlSchemaBoolean: Self = Self::from("http://www.w3.org/2001/XMLSchema#boolean");
	
	#[allow(missing_docs)]
	pub const XmlSchemaInteger: Self = Self::from("http://www.w3.org/2001/XMLSchema#integer");
	
	#[allow(missing_docs)]
	pub const XmlSchemaString: Self = Self::from("http://www.w3.org/2001/XMLSchema#string");
	
	const Simple: Self = Self::XmlSchemaString;
}

impl<'a> TryFrom<Cow<'a, str>> for AbsoluteInternationalizedResourceIdentifier<'a>
{
	type Error = AbsoluteInternationalizedResourceIdentifierComponentsParseError;
	
	#[inline(always)]
	fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error>
	{
		Self::try_from(value.as_ref())
	}
}

impl<'a> TryFrom<&'a str> for AbsoluteInternationalizedResourceIdentifier<'a>
{
	type Error = AbsoluteInternationalizedResourceIdentifierComponentsParseError;
	
	#[inline(always)]
	fn try_from(string: &'a str) -> Result<Self, Self::Error>
	{
		let (scheme, remaining_utf8_bytes) = Scheme::parse(string.as_bytes())?;
		Hierarchy::parse(remaining_utf8_bytes)?;
	}
}

impl<'a> AbsoluteInternationalizedResourceIdentifier<'a>
{
	#[inline(always)]
	fn parse<R>(remaining_bytes: &mut &'a [u8], constructor: impl FnOnce(Self) -> R) -> Result<R, AbsoluteInternationalizedResourceIdentifierParseError>
	{
		use AbsoluteInternationalizedResourceIdentifierParseError::*;
		
		let mut string = StringSoFar::new_stack(remaining_bytes);
		
		loop
		{
			let (character, utf8_character_length) = decode_next_utf8(remaining_bytes)?.ok_or(DidNotExpectEndParsingBody)?;
			match character
			{
				CloseAngleBracketChar => break,
				
				invalid @ (x00 ..= x20 | '<' | '"' | '{' | '}' | '|' | '`') => return Err(InvalidCharacter(invalid)),
				
				'\\' => match get_0(remaining_bytes).ok_or(EndOfFileParsingEscapeSequence)?
				{
					u => string.push_forcing_heap_UCHAR4(remaining_bytes).map_err(InvalidUCHAR4EscapeSequence)?,
					
					U => string.push_forcing_heap_UCHAR8(remaining_bytes).map_err(InvalidUCHAR8EscapeSequence)?,
					
					invalid => return Err(InvalidEscapeSequence(invalid)),
				}
				
				character @ _ => string.push(character, utf8_character_length)?,
			}
		}
		
		Ok(constructor(Self::try_from(string.to_cow())?))
	}
}
