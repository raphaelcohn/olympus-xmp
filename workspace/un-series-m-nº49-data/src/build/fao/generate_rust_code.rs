// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) fn generate_rust_code(_out_folder_path: &Path) -> Result<(), NTriplesParseError>
{
	cargo_rerun_if_changed!();
	cargo_rerun_if_changed!("n_triples");
	
	for (m49_code, n_triples_file_bytes) in NTriplesFiles
	{
		if let Err(error) = process_n_triples(m49_code, n_triples_file_bytes)
		{
			let message = format!("M49 {}", m49_code_string(*m49_code));
			Err(error).expect(&message)
		}
	}
	
	unreachable!()
}

fn process_n_triples(m49_code: StaticM49Code, n_triples_file_bytes: &'static [u8]) -> Result<(), ProcessNTriplesError<'static>>
{
	const GeoM49: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth> = AbsoluteInternationalizedResourceIdentifier::http("stats-class.fao.uniroma2.it", ["geo", "m49"]);
	const FaoOntologiesGeopolitical: Predicate<'static> = Predicate::http("stats-class.fao.uniroma2.it", ["ontologies", "geopolitical"]);
	
	let subject = Subject::AbsoluteInternationalizedResourceIdentifier(GeoM49.with_path_segment::<_, true>(m49_code_string(*m49_code)).unwrap());
	
	let n_triples = NTriples::parse(n_triples_file_bytes)?;
	let predicates = n_triples.predicates(&subject).ok_or(ProcessNTriplesError::MissingPredicatesForSubject)?;
	drop(subject);
	
	// eg "953".
	const CodeFaoStat: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("codeFAOSTAT");
	let code_fao_stat = predicates.get_only_one_xml_schema_string(CodeFaoStat)?;
	eprintln!("{}", code_fao_stat);
	
	// eg "001".
	const CodeUn: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("codeUN");
	let code_un = predicates.get_only_one_xml_schema_string(CodeUn)?;
	eprintln!("{}", code_un);
	
	// Should have 1 or more.
	// All look like 'http://stats-class.fao.uniroma2.it/geo/m49/142'.
	const hasMember: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("hasMember");
	for member in predicates.get_absolute_internationalized_resource_identifiers(&hasMember)
	{
		eprintln!("Member {}", member)
	}
	
	/* TODO; require checking for optional presence:-
nameShortAR
nameShortEN
nameShortES
nameShortFR
nameShortRU
nameShortZH
	 */
	// eg "World".
	const nameShortEn: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortEN");
	let name_short_en = predicates.get_only_one_xml_schema_string(nameShortEn)?;
	eprintln!("Name short EN {}", name_short_en);
	
	// eg 'false'.
	let deprecated = predicates.get_only_one_xml_schema_boolean(Predicate::OwlDeprecated)?;
	eprintln!("Deprecated {}", deprecated);
	
	// eg "001".
	let notation = predicates.get_only_one_xml_schema_string(Predicate::SimpleKnowledgeOrganizationSchemeCoreNotation)?;
	eprintln!("Notation {}", notation);
	
	// Should have 1 or more.
	// All look like 'http://stats-class.fao.uniroma2.it/geo/m49/142'.
	// TODO: There is a bug here: They are all the same value!!!!
	for narrower in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::SimpleKnowledgeOrganizationSchemeCoreNarrower)
	{
		eprintln!("Narrower {}", narrower);
	}
	
	for in_scheme in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::SimpleKnowledgeOrganizationSchemeCoreInScheme)
	{
		eprintln!("InScheme {}", in_scheme);
	}
	
	for top_concept_of in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::SimpleKnowledgeOrganizationSchemeCoreTopConceptOf)
	{
		eprintln!("TopConceptOf {}", top_concept_of);
	}
	
	for rdf_syntax_type in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::RdfSyntaxType)
	{
		eprintln!("RdfSyntaxType {}", rdf_syntax_type);
	}
	
	let modified = predicates.get_only_one_xml_schema_date_time(Predicate::DublinCoreModified)?;
	eprintln!("Modified {}", modified);
	
	for (k, v) in predicates.get_string_literals_by_language(&Predicate::SimpleKnowledgeOrganizationSchemeCorePrefLabel).iter()
	{
		eprintln!("PrefLabel {} => {:?}", k, v);
	}
	
	for (k, v) in predicates.get_string_literals_by_language(&Predicate::SimpleKnowledgeOrganizationSchemeCoreAltLabel).iter()
	{
		eprintln!("AltLabel {} => {:?}", k, v);
	}
	
	/*
		assert_eq!(code_un, m49_code_string);
		assert_eq!(code_un, notation);
		assert_eq!(name_short_en, preferred_label_en);
		assert_eq!(name_short_en, alternative_label_en);
	 */
	Ok(())
}

/*
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#broader-geoM49> <http://stats-class.fao.uniroma2.it/geo/m49/054> .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#broader-geoSDG> <http://stats-class.fao.uniroma2.it/geo/m49/054> .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeAGROVOC> "7230" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeDBPediaID> "Solomon_Islands" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeFAOSTAT> "025" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeGAUL> "225" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeISO2> "SB" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeISO3> "SLB" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeUN> "090" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeUNDP> "SOI" .

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
