// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Objects.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Objects<'a>
{
	iri: Vec<IRI<'a>>,

	blank_node: Vec<BlankNodeLabel<'a>>,
	
	string_literals_by_language: BTreeMap<&'a [u8], Vec<Cow<'a, str>>>,
	
	string_literals_by_iri: BTreeMap<IRI<'a>, Vec<Cow<'a, str>>>,
}

impl<'a> Objects<'a>
{
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
			IRI(iri) =>
			{
				let vec = &mut self.iri;
				vec_push_one(vec, iri)
			}
			
			BlankNode(blank_node_label) =>
			{
				let vec = &mut self.blank_node;
				vec_push_one(vec, blank_node_label)
			}
			
			Literal(StringLiteral { literal_value, literal_tag: Language(raw_ietf_bcp_47_language_tag) }) =>
			{
				let vec = self.string_literals_by_language.entry(raw_ietf_bcp_47_language_tag).or_default();
				vec_push_one(vec, literal_value)
			}
			
			Literal(StringLiteral { literal_value, literal_tag: Datatype(iri) }) =>
			{
				let vec = self.string_literals_by_iri.entry(iri).or_default();
				vec_push_one(vec, literal_value)
			}
		}
	}
}
