// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::collections::HashMap;
use std::num::{IntErrorKind, NonZeroU16};
use std::ops::Deref;
use swiss_army_knife::non_zero::new_non_zero_u16;
use n_triples::internationalized_resource_identifier::path::PathSegment;
use n_triples::internationalized_resource_identifier::Authority;
use n_triples::internationalized_resource_identifier::Hierarchy;
use n_triples::internationalized_resource_identifier::Host;
use n_triples::internationalized_resource_identifier::HostName;
use n_triples::internationalized_resource_identifier::Scheme;
use n_triples::FromUnchecked;
use n_triples::Objects;
use super::static_m49_code_to_static_str;

pub(super) fn generate_rust_code(_out_folder_path: &Path) -> Result<(), NTriplesParseError>
{
	cargo_rerun_if_changed!();
	cargo_rerun_if_changed!("n_triples");
	
	for (m49_code, n_triples_file_bytes) in NTriplesFiles
	{
		if let Err(error) = process_n_triples(m49_code, n_triples_file_bytes)
		{
			let message = format!("M49 {}", static_m49_code_to_static_str(m49_code));
			Err(error).expect(&message)
		}
	}
	
	unreachable!()
}

fn process_n_triples(m49_code: StaticM49Code, n_triples_file_bytes: &'static [u8]) -> Result<(), ProcessNTriplesError<'static>>
{
	type Url = Predicate<'static>;
	
	const StatsClassFaoUniroma2It: HostName = host_name("stats-class.fao.uniroma2.it");
	
	const geo: PathSegment = path_segment("geo");
	const m49: PathSegment = path_segment("m49");
	const M49: PathSegment = path_segment("M49");
	const SDG_groups: PathSegment = path_segment("SDG-groups");
	const FAO: PathSegment = path_segment("FAO");
	const _2019_07: PathSegment = path_segment("2019-07");
	const _2019_12: PathSegment = path_segment("2019-12");
	const current: PathSegment = path_segment("current");
	
	// `http://stats-class.fao.uniroma2.it/geo`.
	const Geo: Url = Predicate::http(StatsClassFaoUniroma2It, [geo]);
	
	// `http://stats-class.fao.uniroma2.it/geo/m49`.
	const Geom49: Url = Geo.append(m49);
	
	// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical`.
	const FaoOntologiesGeopolitical: Url = Predicate::http(StatsClassFaoUniroma2It, ["ontologies", "geopolitical"]);
	
	// `http://stats-class.fao.uniroma2.it/geo/M49`.
	const SchemeGeoM49: Url = Geo.append(M49);
	
	// `http://stats-class.fao.uniroma2.it/geo/M49/SDG-groups`.
	const SchemeGeoM49SocialDevelopmentGoalsGroups: Url = SchemeGeoM49.append(SDG_groups);
	
	// `http://stats-class.fao.uniroma2.it/geo/M49/FAO`.
	// This is not a scheme but the parent of several versioned schemes.
	const GeoM49Fao: Url = SchemeGeoM49.append(FAO);
	
	// `http://stats-class.fao.uniroma2.it/geo/M49/FAO/2019-07`.
	const SchemeGeoM49Fao201907: Url = GeoM49Fao.append(_2019_07);
	
	// `http://stats-class.fao.uniroma2.it/geo/M49/FAO/2019-12`.
	const SchemeGeoM49Fao201912: Url = GeoM49Fao.append(_2019_12);
	
	// `http://stats-class.fao.uniroma2.it/geo/M49/FAO/current`.
	const SchemeGeoM49FaoCurrent: Url = GeoM49Fao.append(current);
	
	// `http://stats-class.fao.uniroma2.it/geo/m49/<m49_code>`.
	let subject = Subject::AbsoluteInternationalizedResourceIdentifier(Geom49.append(m49_code));
	
	let n_triples = NTriples::parse(n_triples_file_bytes)?;
	let predicates = n_triples.predicates(&subject).ok_or(ProcessNTriplesError::MissingPredicatesForSubject)?;
	
	eprintln!();
	
	
	
	// eg "World".
	// Observed combinations:-
	/*
		(None)
		English only
		English-Spanish-French
		All
	 */
	const NameShortEnglish: Predicate<'static> = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortEN");
	let name_short_english = predicates.get_optional_xml_schema_string(NameShortEnglish)?;
	eprintln!("English short name {:?}", name_short_english);
	if let Some(name_short_english) = name_short_english
	{
		const NameShortArabic: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortAR");
		let name_short_arabic = predicates.get_optional_xml_schema_string(NameShortArabic)?;
		eprintln!("Arabic short name {:?}", name_short_arabic);
		
		const NameShortSpanish: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortES");
		let name_short_spanish = predicates.get_optional_xml_schema_string(NameShortSpanish)?;
		eprintln!("Spanish short name {:?}", name_short_spanish);
		
		const NameShortFrench: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortFR");
		let name_short_french = predicates.get_optional_xml_schema_string(NameShortFrench)?;
		eprintln!("French short name {:?}", name_short_french);
		
		const NameShortRussian: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortRU");
		let name_short_russian = predicates.get_optional_xml_schema_string(NameShortRussian)?;
		eprintln!("Russian short name {:?}", name_short_russian);
		
		const NameShortChinese: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("nameShortZH");
		let name_short_chinese = predicates.get_optional_xml_schema_string(NameShortChinese)?;
		eprintln!("Chinese short name {:?}", name_short_chinese);
	}
	
	// Can be missing, eg for 062.
	// Yet another appalling bad CSV database is downloadable from `<https://www.fao.org/faostat/en/#definitions>`; this includes some other obscure M49 terms.
	const CodeFaoStat: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("codeFAOSTAT");
	let code_fao_stat = predicates.get_optional_xml_schema_string(CodeFaoStat)?;
	eprintln!("Code FAO Stat {:?}", code_fao_stat);
	
	// eg "001".
	let notation = predicates.get_only_one_xml_schema_string(Predicate::SimpleKnowledgeOrganizationSchemeCoreNotation)?;
	eprintln!("Notation {}", notation);
	
	// Can be missing, eg for 062 - in which case we need to rely on the notation field.
	// eg "001".
	const CodeUn: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("codeUN");
	let code_un = predicates.get_optional_xml_schema_string(CodeUn)?;
	eprintln!("Code UN {:?}", code_un);
	
	const HasMember: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("hasMember");
	for member in predicates.get_absolute_internationalized_resource_identifiers(&HasMember)
	{
		eprintln!("Member {}", member.remove_leaving_one_path_segment_or_error(Geom49)?)
	}
	
	#[inline(always)]
	fn get_optional_absolute_internationalized_resource_identifier<'a, 'predicates>(predicates: &'predicates HashMap<Predicate<'a>, Objects<'a>>, predicate: Predicate<'static>) -> Result<Option<&'predicates PathSegment<'a>>, ProcessNTriplesError<'static>>
	{
		match predicates.get_optional_absolute_internationalized_resource_identifier(predicate)?
		{
			None => Ok(None),
			
			Some(x) => Ok(Some(x.remove_leaving_one_path_segment_or_error(Geom49)?))
		}
	}
	
	const BroaderGeoM49: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("broader-geoM49");
	eprintln!("BroaderGeoM49 {:?}", get_optional_absolute_internationalized_resource_identifier(predicates, BroaderGeoM49)?);
	
	const BroaderGeoSocialDevelopmentGoals: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("broader-geoSDG");
	eprintln!("BroaderGeoSocialDevelopmentGoals {:?}", get_optional_absolute_internationalized_resource_identifier(predicates, BroaderGeoSocialDevelopmentGoals)?);
	
	const InGroup: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("inGroup");
	eprintln!("InGroup {:?}", get_optional_absolute_internationalized_resource_identifier(predicates, InGroup)?);
	
	// TODO: This can be more than 1, eg for 015, as it includes unofficial intermediates (eg 747, Northern Africa and Western Asia).
	// eprintln!("SimpleKnowledgeOrganizationSchemeCoreBroader {:?}", get_optional_absolute_internationalized_resource_identifier(predicates, AbsoluteInternationalizedResourceIdentifier::SimpleKnowledgeOrganizationSchemeCoreBroader)?);
	
	// See <https://www.fao.org/agrovoc/.>
	// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeAGROVOC`.
	// eg `7230`.
	// Seems to have access to country names in more than the official languages, eg <https://agrovoc.fao.org/browse/agrovoc/en/page/c_33095>.
	const CodeAgrovoc: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("codeAGROVOC");
	// TODO: Should match <http://stats-class.fao.uniroma2.it/geo/m49/203> <http://www.w3.org/2004/02/skos/core#exactMatch> <http://aims.fao.org/aos/agrovoc/c_33095> .
	
	// See <https://www.dbpedia.org/about/>.
	// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeDBPediaID`.
	// eg `Solomon_Islands`.
	const CodeDbpediaIdentifier: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("codeDBPediaID");
	// TODO: Should match <http://stats-class.fao.uniroma2.it/geo/m49/203> <http://www.w3.org/2004/02/skos/core#exactMatch> <http://dbpedia.org/resource/Czech_Republic>
	
	// Global Administrative Unit Layers (GAUL) country level identifiers (also known as level 0).
	// Also known as `ADM0_CODE`.
	// "Level 0 (ADM0): International or country boundaries. National boundaries as provided by the UN Cartographic Unit (disputed area boundaries are also included). This is the highest level".
	// Can not be zero.
	// See the document "TechnicalAspecsts2015_Doc1.pdf" for more information.
	// Codes are never reused and their description are never updated; instead, new codes are issued.
	// See <https://data.apps.fao.org/map/catalog/srv/eng/catalog.search?id=12691#/metadata/9c35ba10-5649-41c8-bdfc-eb78e9e65654>.
	// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeGAUL`.
	// eg `225`.
	const CodeGaul: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("codeGAUL");
	
	// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeISO2`.
	// eg `SB`.
	const CodeIso2: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("codeISO2");
	
	// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeISO3`.
	// eg `SLB`.
	const CodeIso3: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("codeISO3");
	
	// `http://stats-class.fao.uniroma2.it/ontologies/geopolitical#codeUNDP`.
	// eg `SOI`.
	const CodeUnDp: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("codeUNDP");
	
	// eg 'false'.
	let deprecated = predicates.get_optional_xml_schema_boolean(Predicate::OwlDeprecated)?;
	eprintln!("Deprecated {:?}", deprecated);
	
	// Should have 1 or more.
	// All look like 'http://stats-class.fao.uniroma2.it/geo/m49/142'.
	for narrower in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::SimpleKnowledgeOrganizationSchemeCoreNarrower)
	{
		eprintln!("Narrower {}", narrower.remove_leaving_one_path_segment_or_error(Geom49)?);
	}
	
	// For AGROVOC and DBpedia URLs.
	// eg, For the Czech Republic:-
	// * `<http://aims.fao.org/aos/agrovoc/c_33095>`.
	// * `<http://dbpedia.org/resource/Czech_Republic>`.
	for exact_match in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::SimpleKnowledgeOrganizationSchemeCoreExactMatch)
	{
		eprintln!("Exact Match {}", exact_match);
	}
	
	// A sort-of Schema name.
	// TODO: Consider recording which schemes.
	for in_scheme in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::SimpleKnowledgeOrganizationSchemeCoreInScheme)
	{
		// Some items are only in one scheme, eg SDG-groups.
		
		match in_scheme
		{
			&AbsoluteInternationalizedResourceIdentifier { scheme: Scheme::http, hierarchy: Hierarchy::AuthorityAndAbsolutePath { authority: Authority { user_information: None, host: Host::Name(ref host_name), port: None }, ref absolute_path }, query: None, hash_fragment: None } if host_name == &StatsClassFaoUniroma2It =>
			{
				// Sadly, we can't have structural equality when using a Cow.
				
				// TODO: What we can do, at a little cost, is create a &[&'a str] array from path segments using AsRef or some such.
				
				let slice = absolute_path.deref();
				match slice.strip_prefix(&[geo, M49])
				{
					None => eprintln!("InScheme Unknown StatsClassFaoUniroma2It {}", in_scheme),
					
					Some(tail) => match tail.len()
					{
						0 => eprintln!("InScheme M49"),
						
						1 => if tail.get_unchecked_safe(0usize) == &SDG_groups
						{
							eprintln!("InScheme M49 SDG-groups")
						}
						else
						{
							eprintln!("InScheme Unknown StatsClassFaoUniroma2It {}", in_scheme)
						}
						
						2 => if tail.get_unchecked_safe(0usize) == &FAO
						{
							let version = tail.get_unchecked_safe(1usize);
							if version == &_2019_07
							{
								eprintln!("InScheme M49 FAO 2019-07")
							}
							else if version == &_2019_12
							{
								eprintln!("InScheme M49 FAO 2019-12")
							}
							else if version == &current
							{
								eprintln!("InScheme M49 FAO Current")
							}
							else
							{
								eprintln!("InScheme Unknown StatsClassFaoUniroma2It {}", in_scheme)
							}
						}
						else
						{
							eprintln!("InScheme Unknown StatsClassFaoUniroma2It {}", in_scheme)
						}
						
						_ => (),
					}
				}
				// match slice
				// {
				// 	&[geo, M49] => eprintln!("InScheme M49"),
				//
				// 	&[geo, M49, SDG_groups] => eprintln!("InScheme M49 SDG-groups"),
				//
				// 	&[geo, M49, FAO, _2019_07] => eprintln!("InScheme M49 FAO 2019-07"),
				//
				// 	&[geo, M49, FAO, _2019_12] => eprintln!("InScheme M49 FAO 2019-12"),
				//
				// 	&[geo, M49, FAO, current] => eprintln!("InScheme M49 FAO Current"),
				//
				// 	_ => eprintln!("InScheme Unknown StatsClassFaoUniroma2It {}", in_scheme),
				// }
				
				/*
				 RdfSyntaxType http://stats-class.fao.uniroma2.it/ontologies/geopolitical#geographical_region
  RdfSyntaxType http://www.w3.org/2004/02/skos/core#Concept
  http://stats-class.fao.uniroma2.it/ontologies/geopolitical#self_governing
				 */
			},
			
			_ => eprintln!("InScheme Unknown {}", in_scheme),
		}
	}
	
	// A sort-of Schema name.
	// TODO: Consider recording which schemes.
	for top_concept_of in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::SimpleKnowledgeOrganizationSchemeCoreTopConceptOf)
	{
		eprintln!("TopConceptOf {}", top_concept_of);
	}
	
	for rdf_syntax_type in predicates.get_absolute_internationalized_resource_identifiers(&Predicate::RdfSyntaxType)
	{
		eprintln!("RdfSyntaxType {}", rdf_syntax_type);
	}
	
	let modified = predicates.get_optional_xml_schema_boolean(Predicate::DublinCoreModified)?;
	eprintln!("Modified {:?}", modified);
	
	for (language, v) in predicates.get_string_literals_by_language(&Predicate::SimpleKnowledgeOrganizationSchemeCorePrefLabel).iter()
	{
		eprintln!("PrefLabel {} => {:?}", language, v);
	}
	
	for (language, v) in predicates.get_string_literals_by_language(&Predicate::SimpleKnowledgeOrganizationSchemeCoreAltLabel).iter()
	{
		eprintln!("AltLabel {} => {:?}", language, v);
	}
	
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
	struct Year(NonZeroU16);
	
	impl TryFrom<Integer> for Year
	{
		type Error = Infallible;
		
		#[inline(always)]
		fn try_from(value: Integer) -> Result<Self, Self::Error>
		{
			if value < 1900 || value > 9999
			{
				panic!("Year out of range")
			}
			else
			{
				Ok(Self(new_non_zero_u16(value as u16)))
			}
		}
	}
	
	// Usually 1900.
	// Seem to exist only for countries.
	const ValidSince: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("validSince");
	let valid_since = predicates.get_optional_xml_schema_integer_as_domain_type::<Year>(ValidSince)?;
	eprintln!("Valid Since {:?}", valid_since);
	
	// Usually 9999.
	// Seem to exist only for countries.
	const ValidUntil: Url = FaoOntologiesGeopolitical.with_hash_fragment_const("validUntil");
	let valid_until = predicates.get_optional_xml_schema_integer(ValidUntil)?;
	eprintln!("Valid Until {:?}", valid_until);
	
	match (valid_since, valid_until)
	{
		(None, Some(_)) => panic!("ValidUntil without ValidSince"),
		
		(Some(_), None) => panic!("ValidSince without ValidUntil"),
		
		(Some(valid_since), Some(valid_until)) =>
		{
			assert!(valid_since >= 1900);
			assert!(valid_until <= 9999);
			assert!(valid_since <= valid_until);
			
			// A value of 1992 has been identified.
			//assert_eq!(valid_since, 1900);
			assert_eq!(valid_until, 9999);
		}
		
		_ => (),
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

TODO: Agrovoc NTriples are available at say http://aims.fao.org/aos/agrovoc/c_33095.nt
	Very useful dataset.

There are non-country Agrovoc codes, eg http://aims.fao.org/aos/agrovoc/c_2448.html, which map to UN M49 codes

TODO: Identifies a country <http://stats-class.fao.uniroma2.it/geo/m49/086> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#self_governing>
TODO: Identifies a region <http://stats-class.fao.uniroma2.it/geo/m49/086> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://stats-class.fao.uniroma2.it/ontologies/geopolitical#geographical_region>
 
 */
