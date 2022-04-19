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
	eprintln!("Code FAO Stat {}", code_fao_stat);
	
	// eg "001".
	const CodeUn: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("codeUN");
	let code_un = predicates.get_only_one_xml_schema_string(CodeUn)?;
	eprintln!("Code UN {}", code_un);
	
	// Should have 1 or more.
	// All look like 'http://stats-class.fao.uniroma2.it/geo/m49/142'.
	// TODO: Extract final path segment.
	const HasMember: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("hasMember");
	for member in predicates.get_absolute_internationalized_resource_identifiers(&HasMember)
	{
		eprintln!("Member {}", member)
	}
	
	const BroaderGeoM49: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("broader-geoM49");
	eprintln!("{:?}", predicates.get_zero_or_one_absolute_internationalized_resource_identifier(&BroaderGeoM49)?);
	
	const InGroup: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("inGroup");
	eprintln!("{:?}", predicates.get_zero_or_one_absolute_internationalized_resource_identifier(&InGroup)?);
	
	eprintln!("{:?}", predicates.get_zero_or_one_absolute_internationalized_resource_identifier(&AbsoluteInternationalizedResourceIdentifier::SimpleKnowledgeOrganizationSchemeCoreBroader)?);
	
	// eg "World".
	const NameShortEnglish: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortEN");
	let name_short_english = predicates.get_only_one_xml_schema_string(NameShortEnglish)?;
	eprintln!("English short name {}", name_short_english);
	
	const NameShortArabic: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortAR");
	let name_short_arabic = predicates.get_optional_xml_schema_string(NameShortArabic)?;
	eprintln!("Arabic short name {:?}", name_short_arabic);
	
	const NameShortSpanish: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortES");
	let name_short_spanish = predicates.get_optional_xml_schema_string(NameShortSpanish)?;
	eprintln!("Spanish short name {:?}", name_short_spanish);
	
	const NameShortFrench: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortFR");
	let name_short_french = predicates.get_optional_xml_schema_string(NameShortFrench)?;
	eprintln!("French short name {:?}", name_short_french);
	
	const NameShortRussian: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortRU");
	let name_short_russian = predicates.get_optional_xml_schema_string(NameShortRussian)?;
	eprintln!("Russian short name {:?}", name_short_russian);
	
	const NameShortChinese: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortZH");
	let name_short_chinese = predicates.get_optional_xml_schema_string(NameShortChinese)?;
	eprintln!("Chinese short name {:?}", name_short_chinese);
	
	// eg 'false'.
	let deprecated = predicates.get_only_one_xml_schema_boolean(Predicate::OwlDeprecated)?;
	eprintln!("Deprecated {}", deprecated);
	
	// eg "001".
	let notation = predicates.get_only_one_xml_schema_string(Predicate::SimpleKnowledgeOrganizationSchemeCoreNotation)?;
	eprintln!("Notation {}", notation);
	
	// Should have 1 or more.
	// All look like 'http://stats-class.fao.uniroma2.it/geo/m49/142'.
	for narrower in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::SimpleKnowledgeOrganizationSchemeCoreNarrower)
	{
		eprintln!("Narrower {}", narrower);
	}
	
	// A sort-of Schema name.
	for in_scheme in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::SimpleKnowledgeOrganizationSchemeCoreInScheme)
	{
		eprintln!("InScheme {}", in_scheme);
	}
	
	// A sort-of Schema name.
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
Seem to be for countries:-
TODO:
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#broader-geoSDG> <http://stats-class.fao.uniroma2.it/geo/m49/054> .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeAGROVOC> "7230" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeDBPediaID> "Solomon_Islands" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeGAUL> "225" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeISO2> "SB" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeISO3> "SLB" .
<http://stats-class.fao.uniroma2.it/geo/m49/090> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeUNDP> "SOI" .

TODO: inGroup
<http://stats-class.fao.uniroma2.it/geo/m49/002> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#inGroup> <http://stats-class.fao.uniroma2.it/geo/m49/001> .

TODO: For Afghanistan, say
<http://stats-class.fao.uniroma2.it/geo/m49/004> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#validSince> "1900"^^<http://www.w3.org/2001/XMLSchema#integer> .
<http://stats-class.fao.uniroma2.it/geo/m49/004> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#validUntil> "9999"^^<http://www.w3.org/2001/XMLSchema#integer> .
 */
