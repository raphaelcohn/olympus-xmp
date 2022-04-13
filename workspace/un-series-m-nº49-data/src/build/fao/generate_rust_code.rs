// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::borrow::Borrow;
use n_triples::PathDepth;
use n_triples::internationalized_resource_identifier::AbsoluteInternationalizedResourceIdentifier;

#[inline(always)]
pub(super) fn generate_rust_code(out_folder_path: &Path) -> Result<(), NTriplesParseError>
{
	cargo_rerun_if_changed!();
	cargo_rerun_if_changed!("n_triples");
	
	for (m49_code, n_triples_file_bytes) in NTriplesFiles
	{
		process_n_triples(m49_code, n_triples_file_bytes)
	}
	
	unreachable!()
}

fn process_n_triples(m49_code: StaticM49Code, n_triples_file_bytes: &'static [u8])
{
	const GeoM49: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth> = AbsoluteInternationalizedResourceIdentifier::http("stats-class.fao.uniroma2.it", ["geo", "m49"]);
	let m49_code_string = m49_code_string(*m49_code);
	let subject = Subject::AbsoluteInternationalizedResourceIdentifier(GeoM49.with_path_segment::<_, true>(m49_code_string.clone()).unwrap());
	{
		macro_rules! expect
		{
			($message: expr, $result: expr) =>
			{
				match $result
				{
					Ok(ok) => ok,
					
					Err(error) =>
					{
						let message = format!("M49 code {}: {}", m49_code_string, $message);
						Err(error).expect(&message)
					}
				}
			}
		}
	
		macro_rules! expect_option
		{
			($message: expr, $option: expr) =>
			{
				match $option
				{
					Some(ok) => ok,
					
					None =>
					{
						let message = format!("M49 code {}: {}", m49_code_string, $message);
						None.expect(&message)
					}
				}
			}
		}
		
		let n_triples = expect!("NTriples::parse", NTriples::parse(n_triples_file_bytes));
		let predicates = expect_option!("Missing predicates for subject", n_triples.predicates(&subject));
		
		{
			macro_rules! simple_string
			{
				($identifier: ident) =>
				{
					expect!(stringify!($identifier), predicates.get_simple_string(&$identifier))
				}
			}
			
			macro_rules! simple_boolean
			{
				($identifier: ident) =>
				{
					expect!(stringify!($identifier), predicates.get_simple_boolean(&$identifier))
				}
			}
			
			macro_rules! absolute_internationalized_resource_identifiers
			{
				($identifier: ident) =>
				{
					{
						let message = stringify!($identifier);
						expect!(message, predicates.get_absolute_internationalized_resource_identifiers(&$identifier))
					}
				}
			}
			
			const FaoOntologiesGeopolitical: Predicate<'static> = Predicate::http("stats-class.fao.uniroma2.it", ["ontologies", "geopolitical"]);
			
			/// `"http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeFAOSTAT"`.
			/// eg "953".
			const GeopoliticalCodeFAOSTAT: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("codeFAOSTAT");
			let code_fao_stat = simple_string!(GeopoliticalCodeFAOSTAT);
			
			/// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeUN`.
			/// eg "001".
			const GeopoliticalCodeUN: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("codeUN");
			let code_un = simple_string!(GeopoliticalCodeUN);
			
			/// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical#hasMember`.
			/// Should have 1 or more.
			/// All look like 'http://stats-class.fao.uniroma2.it/geo/m49/142'.
			const GeopoliticalHasMember: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("hasMember");
			for member in absolute_internationalized_resource_identifiers!(GeopoliticalHasMember)
			{
				eprintln!("Member {:?}", member)
			}
			
			/// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical#nameShortEN`.
			/// eg "World".
			const GeopoliticalNameShortEnglish: Predicate<'static> =  FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortEN");
			let name_short_en = simple_string!(GeopoliticalNameShortEnglish);
			
			// eg 'false'.
			const OwlDeprecated: Predicate<'static> = Predicate::OwlDeprecated;
			let deprecated = simple_boolean!(OwlDeprecated);
			
			// eg "001".
			const SimpleKnowledgeOrganizationSchemeCoreNotation: Predicate<'static> = Predicate::SimpleKnowledgeOrganizationSchemeCoreNotation;
			let notation = simple_string!(SimpleKnowledgeOrganizationSchemeCoreNotation);
			
			// Should have 1 or more.
			// All look like 'http://stats-class.fao.uniroma2.it/geo/m49/142'.
			const SimpleKnowledgeOrganizationSchemeCoreNarrower: Predicate<'static> = Predicate::SimpleKnowledgeOrganizationSchemeCoreNarrower;
			let narrower = absolute_internationalized_resource_identifiers!(SimpleKnowledgeOrganizationSchemeCoreNarrower);
			
			// <SimpleKnowledgeOrganizationSchemeCorePrefLabel> "World"@en
			// There may be only one label per language for this SKOS property.
			const SimpleKnowledgeOrganizationSchemeCorePrefLabel: Predicate<'static> = Predicate::SimpleKnowledgeOrganizationSchemeCorePrefLabel;
			let preferred_label_en =
			{
				let objects = expect_option!("SimpleKnowledgeOrganizationSchemeCorePrefLabel", predicates.get(&SimpleKnowledgeOrganizationSchemeCorePrefLabel));
				for (key, value) in objects.string_literals_by_language()
				{
					eprintln!("Key, Value {:?}, {:?}", key.borrow(), value)
				}
			};
			
			assert_eq!(code_un, m49_code_string);
			assert_eq!(code_un, notation);
			//assert_eq!(name_short_en, preferred_label_en);
		}
	}
}

/*
TODO: For Africa:-
TODO: http://www.w3.org/1999/02/22-rdf-syntax-ns#type
<http://stats-class.fao.uniroma2.it/geo/m49/002> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2004/02/skos/core#Concept> .
<http://stats-class.fao.uniroma2.it/geo/m49/002> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#geographical_region> .

TODO: inGroup
<http://stats-class.fao.uniroma2.it/geo/m49/002> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#inGroup> <http://stats-class.fao.uniroma2.it/geo/m49/001> .

TODO: broader
<http://stats-class.fao.uniroma2.it/geo/m49/002> <http://www.w3.org/2004/02/skos/core#broader> <http://stats-class.fao.uniroma2.it/geo/m49/001> .

TODO: inScheme
<http://stats-class.fao.uniroma2.it/geo/m49/002> <http://www.w3.org/2004/02/skos/core#inScheme> <http://stats-class.fao.uniroma2.it/geo/M49/FAO/2019-07> .
<http://stats-class.fao.uniroma2.it/geo/m49/002> <http://www.w3.org/2004/02/skos/core#inScheme> <http://stats-class.fao.uniroma2.it/geo/M49/FAO/2019-12> .
<http://stats-class.fao.uniroma2.it/geo/m49/002> <http://www.w3.org/2004/02/skos/core#inScheme> <http://stats-class.fao.uniroma2.it/geo/M49/FAO/current> .
<http://stats-class.fao.uniroma2.it/geo/m49/002> <http://www.w3.org/2004/02/skos/core#inScheme> <http://stats-class.fao.uniroma2.it/geo/M49> .

TODO: For World, altLabel
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#altLabel> "World"@en .

TODO: For World, topConceptOf
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#topConceptOf> <http://stats-class.fao.uniroma2.it/geo/M49> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#topConceptOf> <http://stats-class.fao.uniroma2.it/geo/M49/FAO/2019-12> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#topConceptOf> <http://stats-class.fao.uniroma2.it/geo/M49/SDG-groups> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#topConceptOf> <http://stats-class.fao.uniroma2.it/geo/M49/FAO/current> .
<http://stats-class.fao.uniroma2.it/geo/m49/001> <http://www.w3.org/2004/02/skos/core#topConceptOf> <http://stats-class.fao.uniroma2.it/geo/M49/FAO/2019-07> .

TODO: For Afghanistan, say
<http://stats-class.fao.uniroma2.it/geo/m49/004> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#validSince> "1900"^^<http://www.w3.org/2001/XMLSchema#integer> .
<http://stats-class.fao.uniroma2.it/geo/m49/004> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#validUntil> "9999"^^<http://www.w3.org/2001/XMLSchema#integer> .
 */
