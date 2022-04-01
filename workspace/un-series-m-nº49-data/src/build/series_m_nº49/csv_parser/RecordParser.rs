// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(super) struct RecordParser(CrudeCsvLineIterator<'static>);

impl From<&'static str> for RecordParser
{
	#[inline(always)]
	fn from(record: &'static str) -> Self
	{
		assert!(!record.is_empty(), "Empty record");
		Self(CrudeCsvLineIterator::from(record))
	}
}

impl RecordParser
{
	#[inline(always)]
	pub(super) fn parse_record(mut self, english_csv_file_contains_country_names_with_commas: bool) -> Record
	{
		let global = self.parse_name_and_m49_code::<"global">();
		
		// Only Antarctica lacks a region.
		let region = match (self.parse_optional_name_and_m49_code::<"region">(), self.parse_optional_name_and_m49_code::<"sub_region">(), self.parse_optional_name_and_m49_code::<"intermediate_region">())
		{
			(None, None, None) => None,
			
			(Some(region), None, None) => Some(Region { region, sub_region: None }),
			
			(Some(region), Some(sub_region), None) => Some(Region { region, sub_region: Some(SubRegion { sub_region, intermediate_region: None }) }),
			
			(Some(region), Some(sub_region), Some(intermediate_region)) => Some(Region { region, sub_region: Some(SubRegion { sub_region, intermediate_region: Some(intermediate_region) }) }),
			
			(region, sub_region, intermediate_region) => panic!("Invalid combination of region, sub_region and intermediate_region: ({:?}, {:?}, {:?})", region, sub_region, intermediate_region),
		};
		
		let country = self.parse_m49_code_and_name::<"country">(english_csv_file_contains_country_names_with_commas);
		
		let record = Record
		{
			global,
			region,
			country,
			iso_3166_1_alpha2_code: self.parse_iso_3166_1_alpha2_code(country.is_sark()),
			iso_3166_1_alpha3_code: self.parse_iso_3166_1_alpha3_code(country.is_sark()),
			developing: Developing
			{
				least_developed_countries: self.parse_boolean::<"least_developed_countries">(),
				land_locked_developing_countries: self.parse_boolean::<"land_locked_developing_countries">(),
				small_island_developing_states: self.parse_boolean::<"small_island_developing_states">(),
			},
		};
		
		self.assert_no_more_fields();
		
		record
	}
	
	#[inline(always)]
	fn assert_no_more_fields(mut self)
	{
		assert!(self.0.next().is_none(), "Has extra fields")
	}
	
	// Sark has only an unofficial ISO-3166 dash 1 alpha2 code that is omitted from the UN's M.49 data as of March 2022 (`CQ`).
	#[inline(always)]
	fn parse_iso_3166_1_alpha2_code(&mut self, is_sark: bool) -> Iso3166Dash1Alpha2Code
	{
		const FIELD_NAME: &'static str = "iso_3166_1_alpha2_code";
		
		if is_sark
		{
			let next = self.next::<FIELD_NAME>();
			assert_eq!(next.len(), 0, "Sark has an ISO 3166-1 Alpha2 code");
			return Iso3166Dash1Alpha2Code([b'C', b'Q'])
		}
		
		let potential = self.next_n_bytes::<2, FIELD_NAME>();
		Iso3166Dash1Alpha2Code
		([
			Self::to_alpha::<2, 0, FIELD_NAME>(potential),
			Self::to_alpha::<2, 1, FIELD_NAME>(potential),
		])
	}
	
	// Sark does not have an assigned alpha3 code.
	#[inline(always)]
	fn parse_iso_3166_1_alpha3_code(&mut self, is_sark: bool) -> Option<Iso3166Dash1Alpha3Code>
	{
		const FIELD_NAME: &'static str = "iso_3166_1_alpha3_code";
		
		if is_sark
		{
			let next = self.next::<FIELD_NAME>();
			assert_eq!(next.len(), 0, "Sark has an ISO 3166-1 Alpha3 code");
			return None
		}
		
		let potential = self.next_n_bytes::<3, FIELD_NAME>();
		Some
		(
			Iso3166Dash1Alpha3Code
			([
				Self::to_alpha::<3, 0, FIELD_NAME>(potential),
				Self::to_alpha::<3, 1, FIELD_NAME>(potential),
				Self::to_alpha::<3, 2, FIELD_NAME>(potential),
			])
		)
	}
	
	#[inline(always)]
	fn parse_boolean<const FIELD_NAME: &'static str>(&mut self) -> bool
	{
		match self.next_str::<FIELD_NAME>()
		{
			"x" => true,
			
			"" => false,
			
			_ => panic!("Invalid boolean for {}", FIELD_NAME)
		}
	}
	
	#[inline(always)]
	fn parse_name_and_m49_code<const FIELD_NAME: &'static str>(&mut self) -> NameAndM49Code
	{
		let m49_code = Self::three_digit_m49_code::<FIELD_NAME>(self.next_n_bytes::<3, FIELD_NAME>());
		let name = self.next_str::<FIELD_NAME>();
		
		assert!(!name.is_empty(), "Empty name for {}", FIELD_NAME);
		
		NameAndM49Code::new(name, m49_code)
	}
	
	// The English CSV file contains unquoted fields, some of which contain embedded commas!
	// eg "Bonaire, Sint Eustatius and Saba" parses as "Bonaire" " Sint Eustatius and Saba".
	#[inline(always)]
	fn parse_unquoted_name_with_embedded_comma<const FIELD_NAME: &'static str>(&mut self) -> &'static str
	{
		let snapshot_before_potential_start_of_name = self.0.snapshot();
		let potential_start_of_name = self.next_str::<FIELD_NAME>();
		
		let snapshot_before_potential_remainder_of_name = self.0.snapshot();
		let potential_remainder_of_name = self.next_str::<FIELD_NAME>();
		if potential_remainder_of_name.starts_with(" ")
		{
			Self::bytes_to_str(self.0.reset_and_skip_comma_next(snapshot_before_potential_start_of_name))
		}
		else
		{
			self.0.reset(snapshot_before_potential_remainder_of_name);
			potential_start_of_name
		}
	}
	
	#[inline(always)]
	fn parse_m49_code_and_name<const FIELD_NAME: &'static str>(&mut self, english_csv_file_contains_country_names_with_commas: bool) -> NameAndM49Code
	{
		let name = if english_csv_file_contains_country_names_with_commas
		{
			self.parse_unquoted_name_with_embedded_comma::<FIELD_NAME>()
		}
		else
		{
			self.next_str::<FIELD_NAME>()
		};
		assert!(!name.is_empty(), "Empty name for {}", FIELD_NAME);
		
		let m49_code = self.next_m49_code::<FIELD_NAME>();
		
		NameAndM49Code::new(name, m49_code)
	}
	
	#[inline(always)]
	fn parse_optional_name_and_m49_code<const FIELD_NAME: &'static str>(&mut self) -> Option<NameAndM49Code>
	{
		let code = self.next::<FIELD_NAME>();
		let m49_code = Self::optional_three_digit_m49_code::<FIELD_NAME>(code);
		let name = self.next_str::<FIELD_NAME>();
		
		match (name.is_empty(), m49_code)
		{
			(true, None) => None,
			
			(false, Some(m49_code)) => Some(NameAndM49Code::new(name, m49_code)),
			
			_ => panic!("Invalid name and M.49 combination for {} ('{:?}' and '{:?}')", FIELD_NAME, name, m49_code)
		}
	}
	
	#[inline(always)]
	fn optional_three_digit_m49_code<const FIELD_NAME: &'static str>(m49_code: &[u8]) -> Option<M49Code>
	{
		if m49_code.is_empty()
		{
			None
		}
		else
		{
			let pointer = m49_code.as_ptr().cast::<[u8; 3]>();
			Some(Self::three_digit_m49_code::<FIELD_NAME>(unsafe { &* pointer }))
		}
	}
	
	#[inline(always)]
	fn three_digit_m49_code<const FIELD_NAME: &'static str>(m49_code: &[u8; 3]) -> M49Code
	{
		let bytes = m49_code;
		assert_eq!(m49_code.len(), 3, "Length of M.49 code is not 3 numbers for {} (code was '{:?}')", FIELD_NAME, m49_code);
		
		M49Code
		([
			Self::to_number::<3, 0, FIELD_NAME>(bytes),
			Self::to_number::<3, 1, FIELD_NAME>(bytes),
			Self::to_number::<3, 2, FIELD_NAME>(bytes),
		])
	}
	
	#[inline(always)]
	fn to_number<const length: usize, const index: usize, const FIELD_NAME: &'static str>(bytes: &[u8; length]) -> u8
	{
		let byte = bytes.get_unchecked_value_safe(index);
		assert!(byte >= _0 && byte <= _9, "Invalid 0-9 at index {} for {}", index, FIELD_NAME);
		byte
	}
	
	#[inline(always)]
	fn to_alpha<const length: usize, const index: usize, const FIELD_NAME: &'static str>(bytes: &[u8; length]) -> u8
	{
		let byte = bytes.get_unchecked_value_safe(index);
		assert!(byte >= A && byte <= Z, "Invalid A-Z at index {} for {}", index, FIELD_NAME);
		byte
	}
	
	#[inline(always)]
	fn next_m49_code<const FIELD_NAME: &'static str>(&mut self) -> M49Code
	{
		Self::three_digit_m49_code::<FIELD_NAME>(self.next_n_bytes::<3, FIELD_NAME>())
	}
	
	#[inline(always)]
	fn next_n_bytes<const length: usize, const FIELD_NAME: &'static str>(&mut self) -> &'static [u8; length]
	{
		let potential = self.next::<FIELD_NAME>();
		assert_eq!(potential.len(), length, "Invalid length (not {}) for {}", length, FIELD_NAME);
		let pointer = potential.as_ptr().cast::<[u8; length]>();
		unsafe { &*pointer }
	}
	
	#[inline(always)]
	fn next_str<const FIELD_NAME: &'static str>(&mut self) -> &'static str
	{
		Self::bytes_to_str(self.next::<FIELD_NAME>())
	}
	
	#[inline(always)]
	fn next<const FIELD_NAME: &'static str>(&mut self) -> &'static [u8]
	{
		let next = self.0.next();
		next.unwrap()
	}
	
	#[inline(always)]
	fn bytes_to_str(bytes: &[u8]) -> &str
	{
		unsafe { from_utf8_unchecked(bytes) }
	}
}
