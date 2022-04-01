// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct Parser(HashMap<M49Code, CsvM49CodeType>);

impl Default for Parser
{
	fn default() -> Self
	{
		Self(HashMap::with_capacity(1000))
	}
}

impl Parser
{
	pub(super) fn parse() -> HashMap<M49Code, CsvM49CodeType>
	{
		let mut this = Self::default();
		this.parse_inner();
		this.0
	}
	
	fn parse_inner(&mut self)
	{
		use UnitedNationsOfficialLanguage::*;
		
		self.add_extant_countries_absent_from_csv_for_political_reasons();
		
		self.parse_initial_csv(Series_M_Nº49_English, English);
		self.parse_subsequent_csv(Series_M_Nº49_Arabic, Arabic);
		self.parse_subsequent_csv(Series_M_Nº49_Chinese, Chinese);
		self.parse_subsequent_csv(Series_M_Nº49_French, French);
		self.parse_subsequent_csv(Series_M_Nº49_Russian, Russian);
		self.parse_subsequent_csv(Series_M_Nº49_Spanish, Spanish);
	}
	
	#[inline(always)]
	fn add_extant_countries_absent_from_csv_for_political_reasons(&mut self)
	{
		for (m49_code, names_in_united_nations_official_languages, iso_3166_1_alpha2_code, iso_3166_1_alpha3_code) in ExtantCountriesAbsentFromCsvForPoliticalReasons
		{
			self.add(&m49_code.0, CsvM49CodeType::Country(CsvCountry
			{
				names_in_united_nations_official_languages,
				iso_3166_1_alpha2_code,
				iso_3166_1_alpha3_code: Some(iso_3166_1_alpha3_code),
				developing: Default::default()
			}))
		}
	}
	
	fn parse_initial_csv(&mut self, csv: StaticCsv, language: UnitedNationsOfficialLanguage)
	{
		for record in Self::parse_unsd_m49_csv(csv, language)
		{
			self.parse_initial_record(record, language)
		}
	}
	
	fn parse_subsequent_csv(&mut self, csv: StaticCsv, language: UnitedNationsOfficialLanguage)
	{
		for record in Self::parse_unsd_m49_csv(csv, language)
		{
			self.parse_subsequent_record(record, language);
		}
	}
	
	#[inline(always)]
	fn parse_initial_record(&mut self, record: Record, language: UnitedNationsOfficialLanguage)
	{
		use CsvM49CodeType::*;
		
		self.parse_initial_record_name::<"global">(language, record.global, Global);
		self.parse_initial_record_name::<"country">(language, record.country, |names_in_united_nations_official_languages| Country(record.extant_country(names_in_united_nations_official_languages)));
		
		if let Some(region) = record.region
		{
			self.parse_initial_record_name::<"region">(language, region.region, Region);
			
			if let Some(sub_region) = region.sub_region
			{
				self.parse_initial_record_name::<"sub_region">(language, sub_region.sub_region, SubRegion);
				
				if let Some(intermediate_region) = sub_region.intermediate_region
				{
					self.parse_initial_record_name::<"intermediate_region">(language, intermediate_region, IntermediateRegion);
				}
			}
		}
	}
	
	#[inline(always)]
	fn parse_initial_record_name<const M49_CODE_TYPE: &'static str>(&mut self, language: UnitedNationsOfficialLanguage, name_and_m49_code: NameAndM49Code, constructor: impl FnOnce(NamesInUnitedNationsOfficialLanguages) -> CsvM49CodeType)
	{
		let m49_code = name_and_m49_code.m49_code;
		let non_empty_name = name_and_m49_code.non_empty_name();
		
		let insert = constructor
		({
			let mut names_in_united_nations_official_languages = NamesInUnitedNationsOfficialLanguages::default();
			language.initial(&mut names_in_united_nations_official_languages, non_empty_name);
			names_in_united_nations_official_languages
		});
		
		if let Some(previous) = self.0.insert(m49_code, insert.clone())
		{
			assert_eq!(previous, insert, "Changed {} {:?} record for {} => {}", M49_CODE_TYPE, language, m49_code, non_empty_name);
		}
	}
	
	#[inline(always)]
	fn parse_subsequent_record(&mut self, record: Record, language: UnitedNationsOfficialLanguage)
	{
		use CsvM49CodeType::*;
		
		macro_rules! add_name
		{
			($m49_code_type: ident, $field: ident, record) =>
			{
				{
					let name_and_m49_code = record.$field;
					add_name!($m49_code_type, name_and_m49_code, $field @);
				}
			};
			
			($m49_code_type: ident, $field: ident, $sub_region: ident) =>
			{
				{
					let name_and_m49_code = $sub_region.$field;
					add_name!($m49_code_type, name_and_m49_code, $field @);
				}
			};
			
			($m49_code_type: ident, $name_and_m49_code: ident) =>
			{
				{
					add_name!($m49_code_type, $name_and_m49_code, $name_and_m49_code @);
				}
			};
			
			($m49_code_type: ident, $name_and_m49_code: expr, $M49_CODE_TYPE: tt @) =>
			{
				{
					const M49_CODE_TYPE: &'static str = stringify!($M49_CODE_TYPE);
					let name_and_m49_code = $name_and_m49_code;
					
					match self.get_mut(name_and_m49_code, M49_CODE_TYPE)
					{
						$m49_code_type(names) => Self::add_name(names, name_and_m49_code, language),
						
						_ => panic!("Expected {} {:?} M.49 entry", M49_CODE_TYPE, language)
					}
				}
			}
		}
		
		add_name!(Global, global, record);
		add_name!(Country, country, record);
		
		if let Some(region) = record.region
		{
			add_name!(Region, region, region);
			
			if let Some(sub_region) = region.sub_region
			{
				add_name!(SubRegion, sub_region, sub_region);
				
				if let Some(intermediate_region) = sub_region.intermediate_region
				{
					add_name!(IntermediateRegion, intermediate_region);
				}
			}
		}
		
		match self.0.get(&record.country.m49_code).unwrap()
		{
			Country(country) =>
			{
				assert_eq!(country.developing, record.developing);
				
				use Iso3166Dash1AlphaCode::*;
				match (country.iso_3166_1_alpha_code, record.iso_3166_1_alpha2_code, record.iso_3166_1_alpha3_code)
				{
					(Alpha2Only(left), right, None) => assert_eq!(left ,right),
					
					(Both { alpha2, alpha3 }, right_alpha2, Some(right_alpha3)) =>
					{
						assert_eq!(alpha2 ,right_alpha2);
						assert_eq!(alpha3 ,right_alpha3);
					},
					
					_ => panic!("ISO 3166-1 alpha codes mismatch"),
				}
			}
			
			_ => unreachable!("Already validated this can not be reached")
		}
	}
	
	#[inline(always)]
	fn add(&mut self, code: &[u8; 3], m49_code_type: CsvM49CodeType)
	{
		let was = self.0.insert(code.into(), m49_code_type);
		assert!(was.is_none())
	}
	
	#[inline(always)]
	fn get_mut(&mut self, name_and_m49_code: NameAndM49Code, missing: &str) -> &mut CsvM49CodeType
	{
		let expect = format!("Missing entry for {}", missing);
		self.0.get_mut(&name_and_m49_code.m49_code).expect(&expect)
	}
	
	#[inline(always)]
	fn add_name(mut names: impl AsMut<NamesInUnitedNationsOfficialLanguages>, name_and_m49_code: NameAndM49Code, language: UnitedNationsOfficialLanguage)
	{
		language.initial(names.as_mut(), name_and_m49_code.non_empty_name());
	}
	
	#[inline(always)]
	fn parse_unsd_m49_csv(csv: StaticCsv, language: UnitedNationsOfficialLanguage) -> impl Iterator<Item=Record>
	{
		let english_csv_file_contains_country_names_with_commas = language.is_english();
		inefficient_csv_records(csv).map(move |record|
		{
			let record_parser = RecordParser::from(record);
			record_parser.parse_record(english_csv_file_contains_country_names_with_commas)
		})
	}
}
