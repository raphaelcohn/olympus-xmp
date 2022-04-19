// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Objects.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Objects<'a>
{
	absolute_internationalized_resource_identifiers: Vec<AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>,

	blank_nodes: Vec<BlankNodeLabel<'a>>,
	
	string_literals_by_language: StringLiteralsMap<'a, NaiveIetfBcp47LanguageTag<'a>>,
	
	string_literals_by_absolute_internationalized_resource_identifier: StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>,
}

impl<'a> TryToOwnInPlace for Objects<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.absolute_internationalized_resource_identifiers.try_to_own_in_place()?;
		self.blank_nodes.try_to_own_in_place()?;
		self.string_literals_by_language.try_to_own_in_place()?;
		self.string_literals_by_absolute_internationalized_resource_identifier.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for Objects<'a>
{
	type TryToOwned = Objects<'static>;

	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> Objects<'a>
{
	/// Internationalized Resource Identifiers (IRIs).
	#[inline(always)]
	pub fn absolute_internationalized_resource_identifiers(&self) -> &[AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>]
	{
		&self.absolute_internationalized_resource_identifiers
	}
	
	/// Blank node labels.
	#[inline(always)]
	pub fn blank_nodes(&self) -> &[BlankNodeLabel<'a>]
	{
		&self.blank_nodes
	}
	
	/// String literals by language tag.
	///
	/// If an entry is present, its value will never be an empty Vec.
	#[inline(always)]
	pub fn string_literals_by_language(&self) -> &StringLiteralsMap<'a, NaiveIetfBcp47LanguageTag<'a>>
	{
		&self.string_literals_by_language
	}
	
	/// String literals by Internationalized Resource Identifier (IRI).
	///
	/// If an entry is present, its value will never be an empty Vec.
	#[inline(always)]
	pub fn string_literals_by_absolute_internationalized_resource_identifier(&self) -> &StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>
	{
		&self.string_literals_by_absolute_internationalized_resource_identifier
	}
	
	#[inline(always)]
	fn push(&mut self, object: Object<'a>) -> Result<(), TryReserveError>
	{
		#[inline(always)]
		fn vec_push_one<T>(vec: &mut Vec<T>, one: T) -> Result<(), TryReserveError>
		{
			vec.try_reserve(1)?;
			vec.push_unchecked(one);
			Ok(())
		}
		
		use LiteralTag::*;
		use Object::*;
		match object
		{
			AbsoluteInternationalizedResourceIdentifier(absolute_internationalized_resource_identifier) =>
			{
				let vec = &mut self.absolute_internationalized_resource_identifiers;
				vec_push_one(vec, absolute_internationalized_resource_identifier)
			}
			
			BlankNode(blank_node_label) =>
			{
				let vec = &mut self.blank_nodes;
				vec_push_one(vec, blank_node_label)
			}
			
			Literal(StringLiteral { literal_value, literal_tag: Language(naive_ietf_bcp_47_language_tag) }) =>
			{
				let vec = self.string_literals_by_language.entry(naive_ietf_bcp_47_language_tag);
				vec_push_one(vec, literal_value)
			}
			
			Literal(StringLiteral { literal_value, literal_tag: Datatype(absolute_internationalized_resource_identifier) }) =>
			{
				let vec = self.string_literals_by_absolute_internationalized_resource_identifier.entry(absolute_internationalized_resource_identifier);
				vec_push_one(vec, literal_value)
			}
		}
	}
}
