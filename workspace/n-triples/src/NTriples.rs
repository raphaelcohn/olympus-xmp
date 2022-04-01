// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// N-Triples.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct NTriples<'a>(BTreeMap<Subject<'a>, BTreeMap<Predicate<'a>, Objects<'a>>>);

impl<'a> NTriples<'a>
{
	/*
FAOSTAT
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeFAOSTAT> "953" .

?M49
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeUN> "001" .

?Constitutents?
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#hasMember> <http://stats-class.fao.uniroma2.it/geo/m49/142> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#hasMember> <http://stats-class.fao.uniroma2.it/geo/m49/019> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#hasMember> <http://stats-class.fao.uniroma2.it/geo/m49/150> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#hasMember> <http://stats-class.fao.uniroma2.it/geo/m49/002> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#hasMember> <http://stats-class.fao.uniroma2.it/geo/m49/009> .

Name (again, but without language).
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#nameShortEN> "World" .

Deprecated
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2002/07/owl#deprecated> "false"^^<http://www.w3.org/2001/XMLSchema#boolean> .

Notation
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#notation> "001" .

Constitutents (fugly).
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/202> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/142> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/747> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/062> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/150> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/419> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/753> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/002> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/513> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/009> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#narrower> <http://stats-class.fao.uniroma2.it/geo/m49/019> .
	 */
	// <http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#prefLabel> "World"@en .
	// fn prefLabel(&self, subject: &Subject<'a>) -> Option<&BTreeMap<&'a [u8], Cow<'a, str>>>
	// {
	// 	let m = self.predicates(subject)?;
	// 	let x = m.get(&Predicate::from("http://www.w3.org/2004/02/skos/core#prefLabel"))?;
	// 	Some(&x.string_literals_by_language)
	// }
	
	/// Predicates.
	#[inline(always)]
	pub fn predicates(&self, subject: &Subject<'a>) -> Option<&BTreeMap<Predicate<'a>, Objects<'a>>>
	{
		self.0.get(subject)
	}
	
	/// Parses using the official specification at <https://www.w3.org/TR/n-triples/>.
	#[inline(always)]
	pub fn parse(n_triples_string: &'a str) -> Result<Self, NTriplesParseError>
	{
		let mut this = Self::default();
		
		let mut remaining_bytes = n_triples_string.as_bytes();
		while !remaining_bytes.is_empty()
		{
			let (n_triple, option_remaining_bytes) = NTriple::parse(remaining_bytes).map_err(NTriplesParseError::NTripleParse)?;
			
			this.push(n_triple)?;
			
			match option_remaining_bytes
			{
				None => break,
				
				Some(option_remaining_bytes) => remaining_bytes = option_remaining_bytes,
			}
		}
		
		Ok(this)
	}
	
	#[inline(always)]
	fn push(&mut self, n_triple: NTriple<'a>) -> Result<(), NTriplesParseError>
	{
		let NTriple { subject, predicate, object } = n_triple;
		
		// TODO: Out of memory!
		let predicates = self.0.entry(subject).or_default();
		let objects = predicates.entry(predicate).or_default();
		objects.push(object).map_err(NTriplesParseError::OutOfMemory)
	}
}
