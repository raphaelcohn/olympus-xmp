// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct Objects<'a>
{
	iri: Vec<IRI<'a>>,

	blank_node: Vec<BlankNodeLabel<'a>>,
	
	string_literals_by_language: BTreeMap<&'a [u8], Vec<Cow<'a, str>>>,
	
	string_literals_by_iri: BTreeMap<IRI<'a>, Vec<Cow<'a, str>>>,
}

impl<'a> Objects<'a>
{
	#[inline(always)]
	fn push(&mut self, object: Object<'a>)
	{
		use LiteralTag::*;
		use Object::*;
		match object
		{
			IRI(iri) => self.iri.push(iri),
			
			BlankNode(blank_node_label) => self.blank_node.push(blank_node_label),
			
			Literal(StringLiteral { literal_value, literal_tag: Language(raw_ietf_bcp_47_language_tag) }) => self.string_literals_by_language.entry(raw_ietf_bcp_47_language_tag).or_default().push(literal_value),
			
			Literal(StringLiteral { literal_value, literal_tag: Datatype(iri) }) => self.string_literals_by_iri.entry(iri).or_default().push(literal_value),
		}
	}
}
