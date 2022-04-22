// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Authority and absolute path.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AuthorityAndAbsolutePath<'a, const PathDepth: usize>
{
	/// Authority.
	pub authority: Authority<'a>,
	
	/// Zero or more; each path segment can be empty.
	pub absolute_path: PathSegments<'a, PathDepth>,
}

impl<'a, const PathDepth: usize> Display for AuthorityAndAbsolutePath<'a, PathDepth>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}{}", self.authority, self.absolute_path)
	}
}

impl<'a, const PathDepth: usize> TryToOwnInPlace for AuthorityAndAbsolutePath<'a, PathDepth>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.authority.try_to_own_in_place()?;
		self.absolute_path.try_to_own_in_place()
	}
}

impl<'a, const PathDepth: usize> TryToOwn for AuthorityAndAbsolutePath<'a, PathDepth>
{
	type TryToOwned = AuthorityAndAbsolutePath<'static, PathDepth>;
	
	#[inline(always)]
	fn try_to_own(self) -> Result<Self::TryToOwned, TryReserveError>
	{
		Ok
		(
			AuthorityAndAbsolutePath
			{
				authority: self.authority.try_to_own()?,
				absolute_path: self.absolute_path.try_to_own()?
			}
		)
	}
}

impl<'a, const PathDepth: usize> AuthorityAndAbsolutePath<'a, PathDepth>
{
	/// Appends a path segment.
	///
	/// Can fail with an `Err()` if there is not enough heap memory.
	#[inline(always)]
	pub fn with_path_segment(&mut self, path_segment: PathSegment<'a>) -> Result<(), TryReserveError>
	{
		self.absolute_path.with_path_segment(path_segment)
	}
	
	/// Appends a path segment.
	///
	/// Can fail with an `Err()` if there is not enough memory to allocate on the stack, i.e. the length (`len()`) is already equal to `PathDepth`.
	#[inline(always)]
	pub const fn with_path_segment_const(&mut self, path_segment: PathSegment<'a>) -> Result<(), PathSegment<'a>>
	{
		self.absolute_path.with_path_segment_const(path_segment)
	}
	
	/// Removes 'prefix' returning remaning path segments if possible.
	///
	/// Scheme, query and hash_fragment must all match exactly.
	/// Hierarchy type must match exactly, and, if present, so must authority.
	/// If other has more path segments than self, no match will be returned.
	#[inline(always)]
	pub fn remove<'prefix>(&self, prefix: &AuthorityAndAbsolutePath<'prefix, PathDepth>) -> Option<&[PathSegment<'a>]>
	{
		if self.authority != prefix.authority
		{
			return None
		}
		self.absolute_path.remove(&prefix.absolute_path)
	}
	
	/// `.              = iauthority ipath-abempty`.
	/// `ipath-abempty  = *( "/" isegment )`.
	/// `isegment       = *ipchar`.
	/// `ipath-abempty` examples:-
	/// * ""
	/// * "/segment"
	/// * "/segment/segment"
	/// * "/"
	/// * "//"
	/// * "//segment"
	/// * "//segment/"
	/// * "//segment////segment"
	#[inline(always)]
	fn parse(scheme_specific_parsing_rule: &SchemeSpecificParsingRule, remaining_utf8_bytes: &'a [u8]) -> Result<(Self, ParseNextAfterHierarchy<'a>), AuthorityAndAbsolutePathParseError>
	{
		let mut absolute_path = PathSegments::default();
		
		let (authority, parse_next) = match memchr3(QuestionMark, Hash, Slash, remaining_utf8_bytes)
		{
			None =>
			{
				let authority = Authority::parse(scheme_specific_parsing_rule, remaining_utf8_bytes)?;
				(authority, ParseNextAfterHierarchy::NoQueryNoFragment)
			}
			
			Some(index) =>
			{
				let authority_bytes = remaining_utf8_bytes.before_index(index);
				let authority = Authority::parse(scheme_specific_parsing_rule, authority_bytes)?;
				
				let after_authority_bytes = remaining_utf8_bytes.after_index(index);
				
				let parse_next_after_hierarchy = match remaining_utf8_bytes.get_unchecked_value_safe(index)
				{
					// means there's just iauthority followed by an empty path then a query
					QuestionMark => ParseNextAfterHierarchy::query(remaining_utf8_bytes),
					
					// means there's just iauthority followed by an empty path then an empty query then a fragment
					Hash => ParseNextAfterHierarchy::fragment_no_query(after_authority_bytes),
					
					// after_authority_bytes is the start of the absolute path.
					Slash => absolute_path.parse(after_authority_bytes)?,
					
					_ => unreachable_code_const("memchr3")
				};
				
				(authority, parse_next_after_hierarchy)
			}
		};
		
		Ok((AuthorityAndAbsolutePath { authority, absolute_path }, parse_next))
	}
}
