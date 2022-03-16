// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct Records
{
	file_date: Date,
	
	extlang: HashMap<[u8; 3], (Record, ExtlangRecord)>,
	
	grandfathered: HashMap<String, (Record, GrandfatheredRecord)>,
	
	language: HashMap<LanguageSubtag, (Record, LanguageRecord)>,
	
	redundant: HashMap<String, (Record, RedundantRecord)>,
	
	region: HashMap<RegionSubtag, (Record, RegionRecord)>,
	
	script: HashMap<[u8; 4], (Record, ScriptRecord)>,
	
	variant: HashMap<String, (Record, VariantRecord)>,
}

impl Records
{
	pub(super) fn parse(mut events: PullEventParser) -> Result<Self, LanguageSubtagRegistryFileParseError>
	{
		let mut records = Self
		{
			file_date: Self::parse_file_header(&mut events)?,
			extlang: HashMap::default(),
			grandfathered: HashMap::default(),
			language: HashMap::default(),
			redundant: HashMap::default(),
			region: HashMap::default(),
			script: HashMap::default(),
			variant: HashMap::default(),
		};
		
		records.parse_records(events)?;
		
		Ok(records)
	}
	
	fn parse_file_header(events: &mut PullEventParser) -> Result<Date, RecordsFileHeaderParseError>
	{
		use Event::*;
		use RecordsFileHeaderParseError::*;
		
		let file_date = match events.next().ok_or(MissingFirstLine)??
		{
			Field(event) =>
			{
				if event.field_name() != "File-Date"
				{
					return Err(FileDateShouldBeFirstLine)
				}
				Date::from_str(event.field_body())?
			}
			
			NewRecord => return Err(FileDateShouldBeFirstLineNotNewRecord)
		};
		
		if events.next().ok_or(MissingSecondLine)?? != NewRecord
		{
			Err(SecondLineIsNotStartRecord)
		}
		else
		{
			Ok(file_date)
		}
	}
	
	#[inline(always)]
	fn parse_records(&mut self, mut events: PullEventParser) -> Result<(), LanguageSubtagRegistryFileParseError>
	{
		loop
		{
			if Record::parse(&mut events, self)?
			{
				break
			}
		}
		Ok(())
	}
}
