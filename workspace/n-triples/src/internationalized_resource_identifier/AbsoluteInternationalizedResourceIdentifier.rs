// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Represents an `IRIREF`.
///
/// This is raw; it is not validated according to RFC 3987.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AbsoluteInternationalizedResourceIdentifier<'a, const PathDepth: usize>
{
	/// Scheme.
	pub scheme: Scheme<'a>,
	
	/// Hierarchy.
	pub hierarchy: Hierarchy<'a, PathDepth>,
	
	/// Query.
	pub query: Option<Query<'a>>,
	
	/// Hash Fragment.
	pub hash_fragment: Option<HashFragment<'a>>,
}

impl<'a, const PathDepth: usize> Display for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}:{}", self.scheme, self.hierarchy)?;
		if let Some(ref query) = self.query
		{
			write!(f, "?{}", query)?;
		}
		if let Some(ref hash_fragment) = self.hash_fragment
		{
			write!(f, "#{}", hash_fragment)
		}
		else
		{
			Ok(())
		}
	}
}

impl<'a, const PathDepth: usize> TryToOwnInPlace for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.scheme.try_to_own_in_place()?;
		self.hierarchy.try_to_own_in_place()?;
		self.query.try_to_own_in_place()?;
		self.hash_fragment.try_to_own_in_place()
	}
}

impl<'a, const PathDepth: usize> TryToOwn for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	type TryToOwned = AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>;

	#[inline(always)]
	fn try_to_own(self) -> Result<Self::TryToOwned, TryReserveError>
	{
		Ok
		(
			AbsoluteInternationalizedResourceIdentifier::<'static, PathDepth>
			{
				scheme: self.scheme.try_to_own()?,
				hierarchy: self.hierarchy.try_to_own()?,
				query: self.query.try_to_own()?,
				hash_fragment: self.hash_fragment.try_to_own()?,
			}
		)
	}
}

impl<'a, const PathDepth: usize> TryFrom<&'a str> for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	type Error = AbsoluteInternationalizedResourceIdentifierComponentsParseError;
	
	/// The resultant instance will only be owned if required.
	#[inline(always)]
	fn try_from(string: &'a str) -> Result<Self, Self::Error>
	{
		use AbsoluteInternationalizedResourceIdentifierComponentsParseError::*;
		
		let (scheme, has_authority_and_absolute_path_with_dns_host_name, remaining_utf8_bytes) = Scheme::parse(string.as_bytes()).map_err(SchemeParse)?;
		let (hierarchy, parse_next) = Hierarchy::parse(has_authority_and_absolute_path_with_dns_host_name, remaining_utf8_bytes).map_err(HierarchyParse)?;
		
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
		Ok(Self::new(scheme, hierarchy, query, hash_fragment))
	}
}

impl<'a, const PathDepth: usize> AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	/// Http.
	#[inline(always)]
	pub const fn http<A, P, const M: usize>(authority: A, path_segments: [P; M]) -> Self
	where Authority<'a>: ~const FromUnchecked<A>, PathSegment<'a>: ~const FromUnchecked<P>,
	{
		Self::new_scheme_and_authority(Scheme::http, authority, path_segments)
	}
	
	/// Https.
	#[inline(always)]
	pub const fn https<A, P, const M: usize>(authority: A, path_segments: [P; M]) -> Self
	where Authority<'a>: ~const FromUnchecked<A>, PathSegment<'a>: ~const FromUnchecked<P>,
	{
		Self::new_scheme_and_authority(Scheme::https, authority, path_segments)
	}
	
	#[inline(always)]
	const fn new_scheme_and_authority<A, P, const M: usize>(scheme: Scheme<'a>, authority: A, path_segments: [P; M]) -> Self
	where Authority<'a>: ~const FromUnchecked<A>, PathSegment<'a>: ~const FromUnchecked<P>,
	{
		let authority = unsafe { Authority::from_unchecked(authority) };
		let path_segments = unsafe { PathSegments::from_unchecked(path_segments) };
		Self::new_minimal(scheme, authority.with(path_segments))
	}
	
	/// Appends a path segment.
	///
	/// Not const, but potentially could be.
	/// If the hierarchy is `Hierarchy::EmptyPath`, it is converted according to the argument `convert_empty_path_to_absolute`:-
	///
	/// * If `true`, empty path becomes an absolute path.
	/// * If `false`, empty path becomes a rootless path.
	///
	/// Failure:-
	/// * Can fail with an `Err()` if there is not enough memory.
	/// * If the `path_segment` is empty and the hierarchy is `Hierarchy::EmptyPath`.
	#[inline(always)]
	pub fn with_path_segment<P, const convert_empty_path_to_absolute: bool>(mut self, path_segment: P) -> Result<Self, WithPathSegmentError>
	where PathSegment<'a>: FromUnchecked<P>
	{
		let path_segment = unsafe { PathSegment::from_unchecked(path_segment) };
		self.hierarchy.with_path_segment::<convert_empty_path_to_absolute>(path_segment)?;
		Ok(self)
	}
	
	/// Replace query.
	#[inline(always)]
	pub fn with_query(mut self, query: Query<'a>) -> Self
	{
		self.query = Some(query);
		self
	}
	
	/// Replace query (const).
	///
	/// Only works if the query is previously `None`; panics if it was not.
	/// Does not check that the query is valid.
	#[inline(always)]
	pub const fn with_query_const<Q>(mut self, query: Q) -> Self
	where Query<'a>: ~const FromUnchecked<Q>
	{
		let was = self.query.replace(unsafe { Query::from_unchecked(query) });
		if was.is_some()
		{
			panic!("query was Some()")
		}
		forget(was);
		self
	}
	
	/// Replace hash fragment.
	#[inline(always)]
	pub fn with_hash_fragment(mut self, hash_fragment: HashFragment<'a>) -> Self
	{
		self.hash_fragment = Some(hash_fragment);
		self
	}
	
	/// Replace hash fragment (const).
	///
	/// Only works if the hash fragment is previously `None`; panics if it was not.
	/// Does not check that the hash fragment is valid.
	#[inline(always)]
	pub const fn with_hash_fragment_const<HF>(mut self, hash_fragment: HF) -> Self
	where HashFragment<'a>: ~const FromUnchecked<HF>
	{
		let was = self.hash_fragment.replace(unsafe { HashFragment::from_unchecked(hash_fragment) });
		if was.is_some()
		{
			panic!("hash_fragment was Some()")
		}
		forget(was);
		self
	}
	
	/// Create a new instance.
	#[inline(always)]
	pub const fn new_minimal(scheme: Scheme<'a>, hierarchy: Hierarchy<'a, PathDepth>) -> Self
	{
		Self::new(scheme, hierarchy, None, None)
	}
	
	/// Create a new instance.
	#[inline(always)]
	pub const fn new_with_query(scheme: Scheme<'a>, hierarchy: Hierarchy<'a, PathDepth>, query: Query<'a>) -> Self
	{
		Self::new(scheme, hierarchy, Some(query), None)
	}
	
	/// Create a new instance.
	#[inline(always)]
	pub const fn new_with_hash_fragment(scheme: Scheme<'a>, hierarchy: Hierarchy<'a, PathDepth>, hash_fragment: HashFragment<'a>) -> Self
	{
		Self::new(scheme, hierarchy, None, Some(hash_fragment))
	}
	
	/// Create a new instance.
	#[inline(always)]
	pub const fn new_with_query_and_hash_fragment(scheme: Scheme<'a>, hierarchy: Hierarchy<'a, PathDepth>, query: Query<'a>, hash_fragment: HashFragment<'a>) -> Self
	{
		Self::new(scheme, hierarchy, Some(query), Some(hash_fragment))
	}
	
	/// Create a new instance.
	#[inline(always)]
	pub const fn new(scheme: Scheme<'a>, hierarchy: Hierarchy<'a, PathDepth>, query: Option<Query<'a>>, hash_fragment: Option<HashFragment<'a>>) -> Self
	{
		Self
		{
			scheme,
			hierarchy,
			query,
			hash_fragment,
		}
	}
	
	/// `http://www.w3.org/2002/07/owl#<hash_fragment>`.
	#[inline(always)]
	pub const fn http_www_w3_org_2002_07_owl<HF>(hash_fragment: HF) -> Self
	where HashFragment<'a>: ~const FromUnchecked<HF>
	{
		Self::http_www_w3_org([PathSegment::_2002, PathSegment::_07, PathSegment::owl], hash_fragment)
	}
	
	/// `http://www.w3.org/2004/02/skos/core#<hash_fragment>`
	#[inline(always)]
	pub const fn http_www_w3_org_2004_02_simple_knowledge_organization_scheme_core<HF>(hash_fragment: HF) -> Self
	where HashFragment<'a>: ~const FromUnchecked<HF>
	{
		Self::http_www_w3_org([PathSegment::_2004, PathSegment::_02, PathSegment::skos, PathSegment::core], hash_fragment)
	}
	
	/// `http://www.w3.org/2001/XMLSchema#<hash_fragment>`
	#[inline(always)]
	pub const fn http_www_w3_org_2001_xml_schema<HF>(hash_fragment: HF) -> Self
	where HashFragment<'a>: ~const FromUnchecked<HF>
	{
		Self::http_www_w3_org([PathSegment::_2001, PathSegment::XMLSchema], hash_fragment)
	}
	
	/// `http://www.w3.org/1999/02/22-rdf-syntax-ns#<hash_fragment>`.
	#[inline(always)]
	pub const fn http_www_w3_org_1999_02_22_rdf_syntax_ns<HF>(hash_fragment: HF) -> Self
	where HashFragment<'a>: ~const FromUnchecked<HF>
	{
		Self::http_www_w3_org([PathSegment::_1999, PathSegment::_02, PathSegment::_22_rdf_syntax_ns], hash_fragment)
	}
	
	#[inline(always)]
	const fn http_www_w3_org<HF, const M: usize>(path_segments: [PathSegment<'a>; M], hash_fragment: HF) -> Self
	where HashFragment<'a>: ~const FromUnchecked<HF>
	{
		Self::http("www.w3.org", path_segments).with_hash_fragment_const(hash_fragment)
	}
	
	/// `http://purl.org/dc/terms/<term>`
	#[inline(always)]
	pub const fn http_purl_org_dc_terms(term: &'static str) -> Self
	{
		Self::http("purl.org", [PathSegment::dc, PathSegment::terms, unsafe { PathSegment::from_unchecked(term) }])
	}
	
	pub(super) fn parse<R>(remaining_bytes: &mut &'a [u8], constructor: impl FnOnce(Self) -> R) -> Result<R, AbsoluteInternationalizedResourceIdentifierParseError>
	{
		let string = Self::parse_escaped_string(remaining_bytes)?;
		let this = Self::parse_components(string)?;
		Ok(constructor(this))
	}
	
	#[inline(always)]
	fn parse_escaped_string(remaining_bytes: &mut &'a [u8]) -> Result<StringSoFar<'a>, AbsoluteInternationalizedResourceNTripleEscapedIdentifierParseError>
	{
		use AbsoluteInternationalizedResourceNTripleEscapedIdentifierParseError::*;
		
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
		
		Ok(string)
	}
	
	// So the problem we have here is that we may have created a new string if processing the escape sequences.
	// If so, then we need to return an owned implementation.
	#[inline(always)]
	fn parse_components(string: StringSoFar<'a>) -> Result<Self, AbsoluteInternationalizedResourceIdentifierComponentsParseError>
	{
		let cow = string.to_cow();
		let this = match cow
		{
			// The input string required no escapes.
			// Lifetime is 'a.
			Cow::Borrowed(borrowed) => Self::try_from(borrowed)?,
			
			// The input string required escapes.
			// We have an instance of `String` which we can't return and and reference will only live as long as this method call, ie shorter than 'a.
			// This is a frustrating situation.
			// The only way out is to either somehow attach the `String` to Self, but that would still have us return `Self` with a lifetime of 'a, which forces the caller to carry on owning the original (unused) string.
			Cow::Owned(owned) =>
			{
				let mut this = AbsoluteInternationalizedResourceIdentifier::<PathDepth>::try_from(owned.as_str())?;
				this.try_to_own_in_place()?;
				// This is horrible; Rust does not allow transmute on types with const generics.
				return Ok(unsafe { std::mem::transmute_copy(&this) })
			},
		};
		Ok(this)
	}
}

impl<const PathDepth: usize> AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>
{
	/// `http://www.w3.org/2002/07/owl#deprecated`.
	pub const OwlDeprecated: Self = Self::http_www_w3_org_2002_07_owl("deprecated");
	
	/// `http://www.w3.org/2004/02/skos/core#narrower`.
	pub const SimpleKnowledgeOrganizationSchemeCoreNarrower: Self = Self::http_www_w3_org_2004_02_simple_knowledge_organization_scheme_core("narrower");
	
	/// `http://www.w3.org/2004/02/skos/core#notation`.
	pub const SimpleKnowledgeOrganizationSchemeCoreNotation: Self = Self::http_www_w3_org_2004_02_simple_knowledge_organization_scheme_core("notation");
	
	/// `http://www.w3.org/2004/02/skos/core#prefLabel`.
	pub const SimpleKnowledgeOrganizationSchemeCorePrefLabel: Self = Self::http_www_w3_org_2004_02_simple_knowledge_organization_scheme_core("prefLabel");
	
	/// `http://www.w3.org/2004/02/skos/core#altLabel`.
	pub const SimpleKnowledgeOrganizationSchemeCoreAltLabel: Self = Self::http_www_w3_org_2004_02_simple_knowledge_organization_scheme_core("altLabel");
	
	/// `http://www.w3.org/2004/02/skos/core#inScheme`.
	pub const SimpleKnowledgeOrganizationSchemeCoreInScheme: Self = Self::http_www_w3_org_2004_02_simple_knowledge_organization_scheme_core("inScheme");
	
	/// `http://www.w3.org/2004/02/skos/core#topConceptOf`.
	pub const SimpleKnowledgeOrganizationSchemeCoreTopConceptOf: Self = Self::http_www_w3_org_2004_02_simple_knowledge_organization_scheme_core("topConceptOf");
	
	/// `http://www.w3.org/2001/XMLSchema#boolean`.
	pub const XmlSchemaBoolean: Self = Self::http_www_w3_org_2001_xml_schema("boolean");
	
	/// `http://www.w3.org/2001/XMLSchema#integer`.
	pub const XmlSchemaInteger: Self = Self::http_www_w3_org_2001_xml_schema("integer");
	
	/// `http://www.w3.org/2001/XMLSchema#string`.
	pub const XmlSchemaString: Self = Self::http_www_w3_org_2001_xml_schema("string");
	
	/// `http://www.w3.org/2001/XMLSchema#dateTime`.
	pub const XmlSchemaDateTime: Self = Self::http_www_w3_org_2001_xml_schema("dateTime");
	
	/// `http://purl.org/dc/terms/modified`.
	pub const DublinCoreModified: Self = Self::http_purl_org_dc_terms("modified");
	
	/// `http://www.w3.org/1999/02/22-rdf-syntax-ns#type`.
	pub const RdfSyntaxType: Self = Self::http_www_w3_org_1999_02_22_rdf_syntax_ns("type");
}
