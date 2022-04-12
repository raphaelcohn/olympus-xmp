// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Represents an `IRIREF`.
///
/// This is raw; it is not validated according to RFC 3987.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AbsoluteInternationalizedResourceIdentifier<'a, const PathDepth: usize>
{
	#[allow(missing_docs)]
	pub scheme: Scheme<'a>,
	
	#[allow(missing_docs)]
	pub hierarchy: Hierarchy<'a, PathDepth>,
	
	#[allow(missing_docs)]
	pub query: Option<Query<'a>>,
	
	#[allow(missing_docs)]
	pub hash_fragment: Option<HashFragment<'a>>,
}

// impl<'a, const PathDepth: usize> TryToOwnInPlace for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
// {
// 	#[inline(always)]
// 	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
// 	{
// 		self.scheme.try_to_own_in_place()?;
// 		self.hierarchy.try_to_own_in_place()?;
// 		self.query.try_to_own_in_place()?;
// 		self.hash_fragment.try_to_own_in_place()
// 	}
// }
//
// impl<'a, const PathDepth: usize> TryToOwn for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
// {
// 	type TryToOwned = AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>;
//
// 	#[inline(always)]
// 	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
// 	{
// 		self.try_to_own_in_place()?;
// 		Ok(unsafe { transmute(self) })
// 	}
// }

impl<'a, const PathDepth: usize> TryFrom<Cow<'a, str>> for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	type Error = AbsoluteInternationalizedResourceIdentifierComponentsParseError;
	
	#[inline(always)]
	fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error>
	{
		Self::try_from(value.as_ref())
	}
}

impl<'a, const PathDepth: usize> TryFrom<&'a str> for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	type Error = AbsoluteInternationalizedResourceIdentifierComponentsParseError;
	
	#[inline(always)]
	fn try_from(string: &'a str) -> Result<Self, Self::Error>
	{
		use AbsoluteInternationalizedResourceIdentifierComponentsParseError::*;
		
		let (scheme, remaining_utf8_bytes) = Scheme::parse(string.as_bytes()).map_err(SchemeParse)?;
		let (hierarchy, parse_next) = Hierarchy::parse(remaining_utf8_bytes).map_err(HierarchyParse)?;
		
		use ParseNextAfterHierarchy::*;
		let (query, hash_fragment) = match parse_next
		{
			Query { remaining_utf8_bytes } => match self::Query::parse(remaining_utf8_bytes)
			{
				Err(error) => return Err(QueryParse(error)),
				
				Ok((query, None)) => (Some(query), None),
				
				Ok((query, Some(remaining_utf8_bytes))) => (Some(query), Some(HashFragment::parse(remaining_utf8_bytes)?)),
			}
			
			NoQueryFragment { remaining_utf8_bytes} => (None, Some(HashFragment::parse(remaining_utf8_bytes)?)),
			
			NoQueryNoFragment => (None, None),
		};
		Ok
		(
			Self
			{
				scheme,
				hierarchy,
				query,
				hash_fragment,
			}
		)
	}
}

impl<'a, const PathDepth: usize> AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	/// `http://www.w3.org/2002/07/owl#<hash_fragment>`.
	#[inline(always)]
	pub fn owl_2002_07<HF>(hash_fragment: HF) -> Self
	where HashFragment<'a>: FromUnchecked<HF>
	{
		Self::http_www_w3_org([PathSegment::_2002, PathSegment::_07, PathSegment::owl], hash_fragment)
	}
	
	/// `http://www.w3.org/2004/02/skos/core#<hash_fragment>`
	#[inline(always)]
	pub fn simple_knowledge_organization_scheme_2004_02_core<HF>(hash_fragment: HF) -> Self
	where HashFragment<'a>: FromUnchecked<HF>
	{
		Self::http_www_w3_org([PathSegment::_2004, PathSegment::_02, PathSegment::skos, PathSegment::core], hash_fragment)
	}
	
	/// `http://www.w3.org/2001/XMLSchema#<hash_fragment>`
	#[inline(always)]
	pub fn xml_schema_2001<HF>(hash_fragment: HF) -> Self
	where HashFragment<'a>: FromUnchecked<HF>
	{
		Self::http_www_w3_org([PathSegment::_2001, PathSegment::XMLSchema], hash_fragment)
	}
	
	#[inline(always)]
	pub(super) fn parse<R>(remaining_bytes: &mut &'a [u8], constructor: impl FnOnce(Self) -> R) -> Result<R, AbsoluteInternationalizedResourceIdentifierParseError>
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
	
	#[inline(always)]
	const fn http_www_w3_org<HF, const M: usize>(path_segments: [PathSegment<'a>; M], hash_fragment: HF) -> Self
	where HashFragment<'a>: FromUnchecked<HF>
	{
		Self
		{
			scheme: Scheme::http,
			hierarchy: Hierarchy::AuthorityAndAbsolutePath
			{
				authority: Authority::www_w3_org,
				path_segments: PathSegments::from(path_segments),
			},
			query: None,
			hash_fragment: Self::hash_fragment(hash_fragment),
		}
	}
	
	#[inline(always)]
	const fn hash_fragment<HF>(hash_fragment: HF) -> Option<HashFragment<'a>>
	where HashFragment<'a>: FromUnchecked<HF>
	{
		Some(unsafe { HashFragment::from_unchecked(hash_fragment) })
	}
}

impl<const PathDepth: usize> AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>
{
	/// `http://www.w3.org/2002/07/owl#deprecated`.
	pub const OwlDeprecated: Self = Self::owl_2002_07("deprecated");
	
	/// `http://www.w3.org/2004/02/skos/core#narrower`.
	pub const SimpleKnowledgeOrganizationSchemeCoreNarrower: Self = Self::simple_knowledge_organization_scheme_2004_02_core("narrower");
	
	/// `http://www.w3.org/2004/02/skos/core#notation`.
	pub const SimpleKnowledgeOrganizationSchemeCoreNotation: Self = Self::simple_knowledge_organization_scheme_2004_02_core("notation");
	
	/// `http://www.w3.org/2004/02/skos/core#prefLabel`.
	pub const SimpleKnowledgeOrganizationSchemeCorePrefLabel: Self = Self::simple_knowledge_organization_scheme_2004_02_core("prefLabel");
	
	/// `http://www.w3.org/2001/XMLSchema#boolean`.
	pub const XmlSchemaBoolean: Self = Self::xml_schema_2001("boolean");
	
	/// `http://www.w3.org/2001/XMLSchema#integer`.
	pub const XmlSchemaInteger: Self = Self::xml_schema_2001("integer");
	
	/// `http://www.w3.org/2001/XMLSchema#string`.
	pub const XmlSchemaString: Self = Self::xml_schema_2001("string");
	
	pub(super) const Simple: Self = Self::XmlSchemaString;
}
