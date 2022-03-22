// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Records (database).
#[derive(Debug)]
pub struct Records
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
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn file_date(&self) -> Date
	{
		self.file_date
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn extlang(&self) -> &HashMap<[u8; 3], (Record, ExtlangRecord)>
	{
		&self.extlang
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn grandfathered(&self) -> &HashMap<String, (Record, GrandfatheredRecord)>
	{
		&self.grandfathered
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn language(&self) -> &HashMap<LanguageSubtag, (Record, LanguageRecord)>
	{
		&self.language
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn redundant(&self) -> &HashMap<String, (Record, RedundantRecord)>
	{
		&self.redundant
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn region(&self) -> &HashMap<RegionSubtag, (Record, RegionRecord)>
	{
		&self.region
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn script(&self) -> &HashMap<[u8; 4], (Record, ScriptRecord)>
	{
		&self.script
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn variant(&self) -> &HashMap<String, (Record, VariantRecord)>
	{
		&self.variant
	}
	
	#[inline(always)]
	pub(super) fn parse(mut events: PullEventParser) -> Result<Self, LanguageSubtagRegistryFileContentsParseError>
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
	fn parse_records(&mut self, mut events: PullEventParser) -> Result<(), LanguageSubtagRegistryFileContentsParseError>
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
