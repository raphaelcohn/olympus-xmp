// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Represents an `IRIREF`.
///
///
/// # `file` scheme parsing.
///
/// Note that parsing does not support some of the non-standard `file` scheme variants defined in RFC 3987, Appendix E, specifically:-
///
/// * Appendix E.2, DOS and Windows Drive Letters.
/// 	* Support for this could be added as a special case but there is no place in the domain design to currently put a drive letter.
/// * Appendix E.3, UNC Strings.
/// 	* Support for this could be added as a special case but then requires changes to the parsing of Internet Protocol Version 6 addresses to support percent-encoding of `[` and `]`.
/// * Appendix E.4, Backslash as Separator.
/// 	* Support for this is very painful.
///
/// Appendix E.1, User Information, is supported.
///
///
/// # `ftp` scheme parsing.
///
/// Whilst only draft RFCs exist, these do not permit `;` except in the final path segment.
/// This logic permits `;` in any path segment.
///
///
/// # `mailto` scheme parsing.
///
/// The `mailto` RFC 6068, Section 2, Syntax of a 'mailto' URI, permits an empty rootless path (`mailtoURI = "mailto:" [ to ] [ hfields ];  to = addr-spec *("," addr-spec )`).
/// This is not permitted as per RFC 3987, Section 2; this logic follows the latter for now.
/// Furthermore, normalization of domain names in email addresses is not performed.
///
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
				query: match self.query
				{
					Some(query) => Some(query.try_to_own()?),
					
					None => None,
				},
				hash_fragment: match self.hash_fragment
				{
					Some(hash_fragment) => Some(hash_fragment.try_to_own()?),
					
					None => None,
				},
			}
		)
	}
}

impl<'a, const PathDepth: usize> TryFrom<&'a str> for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	type Error = AbsoluteInternationalizedResourceIdentifierStringParseError;
	
	/// The resultant instance will only be owned if required.
	#[inline(always)]
	fn try_from(string: &'a str) -> Result<Self, Self::Error>
	{
		use AbsoluteInternationalizedResourceIdentifierStringParseError::*;
		
		let (scheme, scheme_specific_parsing_rule, remaining_utf8_bytes) = Scheme::parse(string.as_bytes()).map_err(SchemeParse)?;
		let remaining_string = unsafe { from_utf8_unchecked(remaining_utf8_bytes) };
		let (hierarchy, parse_next) = Hierarchy::parse(scheme_specific_parsing_rule, remaining_string).map_err(HierarchyParse)?;
		
		use ParseNextAfterHierarchy::*;
		let (query, hash_fragment) = match parse_next
		{
			Query { remaining } => match self::Query::parse(remaining, scheme_specific_parsing_rule)
			{
				Err(error) => return Err(QueryParse(error)),
				
				Ok((query, None)) => (Some(query), None),
				
				Ok((query, Some(remaining))) => (Some(query), Some(HashFragment::parse(remaining, scheme_specific_parsing_rule)?)),
			}
			
			NoQueryFragment { remaining } => (None, Some(HashFragment::parse(remaining, scheme_specific_parsing_rule)?)),
			
			NoQueryNoFragment => (None, None),
		};
		Ok(Self::new(scheme, hierarchy, query, hash_fragment))
	}
}

impl<'a, const PathDepth: usize> AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	/// Removes 'prefix' returning one path segment if possible.
	///
	/// Scheme, query and hash_fragment must all match exactly.
	/// Hierarchy type must match exactly, and, if present, so must authority.
	/// If other has more path segments than self, no match will be returned.
	#[inline(always)]
	pub fn remove_leaving_one_path_segment_or_error<'prefix>(&self, prefix: AbsoluteInternationalizedResourceIdentifier<'prefix, PathDepth>) -> Result<&PathSegment<'a>, RemovePrefixError<'prefix, PathDepth>>
	{
		use RemovePrefixError::*;
		
		let option = self.remove(&prefix);
		match option
		{
			None => return Err(MissingPrefix(prefix)),
			
			Some(remainder) => match remainder.len()
			{
				0 => Err(Empty(prefix)),
				
				1 => Ok(remainder.get_unchecked_safe(0)),
				
				more_than_one @ _ => Err(MoreThanOne(MoreThanOneError { count: new_non_zero_usize(more_than_one) }, prefix))
			}
		}
	}
	
	/// Removes 'prefix' returning remaning path segments if possible.
	///
	/// Scheme, query and hash_fragment must all match exactly.
	/// Hierarchy type must match exactly, and, if present, so must authority.
	/// If other has more path segments than self, no match will be returned.
	#[inline(always)]
	pub fn remove<'prefix>(&self, prefix: &AbsoluteInternationalizedResourceIdentifier<'prefix, PathDepth>) -> Option<&[PathSegment<'a>]>
	{
		if self.scheme != prefix.scheme
		{
			return None
		}
		if self.query != prefix.query
		{
			return None
		}
		if self.hash_fragment != prefix.hash_fragment
		{
			return None
		}
		
		use Hierarchy::*;
		
		match (&self.hierarchy, &prefix.hierarchy)
		{
			(AuthorityAndAbsolutePath(authority_and_absolute_path), AuthorityAndAbsolutePath(prefix)) => authority_and_absolute_path.remove(prefix),
			
			(RootlessPath(non_empty_path), RootlessPath(prefix)) => non_empty_path.remove(prefix),
			
			(AbsolutePath(non_empty_path), AbsolutePath(prefix)) => non_empty_path.remove(prefix),
			
			(EmptyPath, EmptyPath) =>
			{
				static Empty: [PathSegment<'static>; 0] = [];
				Some(&Empty[..])
			},
			
			_ => None,
		}
	}
	
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
	/// If the hierarchy is `Hierarchy::EmptyPath`, it is converted according to the argument `convert_empty_path_to_absolute`:-
	///
	/// * If `true`, empty path becomes an absolute path.
	/// * If `false`, empty path becomes a rootless path.
	///
	/// Failure:-
	/// * Can fail with an `Err()` if there is not enough heap memory.
	/// * If the `path_segment` is empty and the hierarchy is `Hierarchy::EmptyPath`.
	#[inline(always)]
	pub fn with_path_segment<P, const convert_empty_path_to_absolute: bool>(mut self, path_segment: P) -> Result<Self, WithPathSegmentError>
	where PathSegment<'a>: FromUnchecked<P>
	{
		let path_segment = unsafe { PathSegment::from_unchecked(path_segment) };
		self.hierarchy.with_path_segment::<convert_empty_path_to_absolute>(path_segment)?;
		Ok(self)
	}
	
	/// Convenience wrapper for `with_path_segment_const()` which assumes `convert_empty_path_to_absolute` is `true`.
	///
	/// Makes for more elegant construction of const literals.
	#[inline(always)]
	pub const fn append<P>(self, path_segment: P) -> Self
	where PathSegment<'a>: ~const FromUnchecked<P>
	{
		self.with_path_segment_const::<P, true>(path_segment)
	}
	
	/// Appends a path segment if there is space on the stack.
	///
	/// If the hierarchy is `Hierarchy::EmptyPath`, it is converted according to the argument `convert_empty_path_to_absolute`:-
	///
	/// * If `true`, empty path becomes an absolute path.
	/// * If `false`, empty path becomes a rootless path.
	///
	/// Panics on Failure:-
	/// * If there is not enough memory on the stack.
	/// * If the `path_segment` is empty and the hierarchy is `Hierarchy::EmptyPath`.
	#[inline(always)]
	pub const fn with_path_segment_const<P, const convert_empty_path_to_absolute: bool>(mut self, path_segment: P) -> Self
	where PathSegment<'a>: ~const FromUnchecked<P>
	{
		let path_segment = unsafe { PathSegment::from_unchecked(path_segment) };
		let result = self.hierarchy.with_path_segment_const::<convert_empty_path_to_absolute>(path_segment);
		// NOTE: This rather odd construction, rather than using `if let Err(error) = result` or `match result`, is because of a bug in Rust const logic; the compiler thinks the `result` object has not been dropped, but the borrow checker thinks it has been partly moved.
		if result.is_err()
		{
			panic!("Out of stack memory or empty path segment passed to a Hierarchy::EmptyPath");
		}
		else
		{
			let _ = ManuallyDrop::new(result);
		}
		self
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
	
	/// Parses a string that is a valid UTF-8 sequence.
	///
	/// Takes a `Cow` as a caller may have created `string` by decoding, say, a N-triples IRI, which contained embedded escape sequences.
	#[inline(always)]
	pub fn parse_string(string: Cow<'a, str>) -> Result<Self, AbsoluteInternationalizedResourceIdentifierStringParseError>
	{
		use Cow::*;
		
		let this = match string
		{
			// Lifetime is 'a.
			Borrowed(borrowed) => Self::try_from(borrowed)?,
			
			// We have an instance of `String` which we can't return and and reference will only live as long as this method call, ie shorter than 'a.
			// This is a frustrating situation.
			// The only way out is to either somehow attach the `String` to Self, but that would still have us return `Self` with a lifetime of 'a, which forces the caller to carry on owning the original (unused) string.
			Owned(owned) =>
			{
				let mut this = AbsoluteInternationalizedResourceIdentifier::<PathDepth>::try_from(owned.as_str())?;
				this.try_to_own_in_place()?;
				// This is horrible; Rust does not allow transmute on types with const generics.
				return Ok(unsafe { transmute_copy(&this) })
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
	
	/// `http://www.w3.org/2004/02/skos/core#broader`.
	pub const SimpleKnowledgeOrganizationSchemeCoreBroader: Self = Self::http_www_w3_org_2004_02_simple_knowledge_organization_scheme_core("broader");
	
	/// `http://www.w3.org/2004/02/skos/core#exactMatch`.
	pub const SimpleKnowledgeOrganizationSchemeCoreExactMatch: Self = Self::http_www_w3_org_2004_02_simple_knowledge_organization_scheme_core("exactMatch");
	
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
