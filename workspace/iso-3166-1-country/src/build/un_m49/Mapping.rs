// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct Mapping(BTreeMap<M49Code, M49CodeType>);

impl Default for Mapping
{
	fn default() -> Self
	{
		Self(BTreeMap::default())
	}
}

impl Mapping
{
	pub(super) fn mapping() -> BTreeMap<M49Code, M49CodeType>
	{
		let mut this = Self::default();
		this.parse();
		this.0
	}
	
	fn parse(&mut self)
	{
		self.parse_initial_csv(English, |names| &mut names.english);
		self.parse_subsequent_csv(Arabic, |names| &mut names.arabic);
		self.parse_subsequent_csv(Chinese, |names| &mut names.chinese);
		self.parse_subsequent_csv(French, |names| &mut names.french);
		self.parse_subsequent_csv(Russian, |names| &mut names.russian);
		self.parse_subsequent_csv(Spanish, |names| &mut names.spanish);
	}
	
	fn parse_initial_csv(&mut self, csv: &'static str, get_field_in_names: impl Copy + FnOnce(&mut Names) -> &mut Option<&'static str>)
	{
		for record in Self::parse_unsd_m49_csv(csv, true)
		{
			self.map_initial_record(record, get_field_in_names)
		}
	}
	
	fn parse_subsequent_csv(&mut self, csv: &'static str, get_field_in_names: impl Copy + FnOnce(&mut Names) -> &mut Option<&'static str>)
	{
		for record in Self::parse_unsd_m49_csv(csv, false)
		{
			self.map_subsequent_record(record, get_field_in_names);
		}
	}
	
	#[inline(always)]
	fn map_initial_record(&mut self, record: Record, get_field_in_names: impl Copy + FnOnce(&mut Names) -> &mut Option<&'static str>)
	{
		use M49CodeType::*;
		
		self.map_initial_record_name::<"global">(get_field_in_names, record.global, Global);
		self.map_initial_record_name::<"country">(get_field_in_names, record.country, |names| Country(record.country(names)));
		
		if let Some(region) = record.region
		{
			self.map_initial_record_name::<"region">(get_field_in_names, region.region, Region);
			
			if let Some(sub_region) = region.sub_region
			{
				self.map_initial_record_name::<"sub_region">(get_field_in_names, sub_region.sub_region, SubRegion);
				
				if let Some(intermediate_region) = sub_region.intermediate_region
				{
					self.map_initial_record_name::<"intermediate_region">(get_field_in_names, intermediate_region, IntermediateRegion);
				}
			}
		}
	}
	
	#[inline(always)]
	fn map_initial_record_name<const M49_CODE_TYPE: &'static str>(&mut self, get_field_in_names: impl FnOnce(&mut Names) -> &mut Option<&'static str>, name_and_m49_code: NameAndM49Code, constructor: impl FnOnce(Names) -> M49CodeType)
	{
		let m49_code = name_and_m49_code.m49_code;
		let name = name_and_m49_code.name;
		
		let insert = constructor
		({
			let mut names = Names::default();
			*get_field_in_names(&mut names) = Some(name);
			names
		});
		
		if let Some(previous) = self.0.insert(m49_code, insert.clone())
		{
			assert_eq!(previous, insert, "Changed {} Arabic record for {} => {}", M49_CODE_TYPE, m49_code, name);
		}
	}
	
	#[inline(always)]
	fn map_subsequent_record(&mut self, record: Record, get_field_in_names: impl Copy + FnOnce(&mut Names) -> &mut Option<&'static str>)
	{
		use M49CodeType::*;
		
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
						$m49_code_type(names) => Self::add_name(names, name_and_m49_code, get_field_in_names),
						
						_ => panic!("Expected {} M.49 entry", M49_CODE_TYPE)
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
				assert_eq!(country.iso_3166_1_alpha2_code, record.iso_3166_1_alpha2_code);
				assert_eq!(country.iso_3166_1_alpha3_code, record.iso_3166_1_alpha3_code);
				assert_eq!(country.developing, record.developing);
			}
			
			_ => unreachable!("Already validated this can not be reached")
		}
	}
	
	#[inline(always)]
	fn get_mut(&mut self, name_and_m49_code: NameAndM49Code, missing: &str) -> &mut M49CodeType
	{
		let expect = format!("Missing entry for {}", missing);
		self.0.get_mut(&name_and_m49_code.m49_code).expect(&expect)
	}
	
	#[inline(always)]
	fn add_name(mut names: impl AsMut<Names>, name_and_m49_code: NameAndM49Code, get_field_in_names: impl FnOnce(&mut Names) -> &mut Option<&'static str>)
	{
		let name = name_and_m49_code.name;
		
		let names = names.as_mut();
		let was = get_field_in_names(names).replace(name);
		if let Some(was) = was
		{
			assert_eq!(was, name, "Name was {} but is now {}", was, name)
		}
	}
	
	fn parse_unsd_m49_csv(csv: &'static str, treat_bonaire_as_special_as_broken_data_in_english_csv_file: bool) -> impl Iterator<Item=Record>
	{
		inefficient_csv_records(csv).map(move |record|
		{
			let parser = RecordParser::from(record);
			parser.parse_record(treat_bonaire_as_special_as_broken_data_in_english_csv_file)
		})
	}
}
