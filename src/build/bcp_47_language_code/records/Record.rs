// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Record
{
	description: Vec<String>,
	
	added: Date,
	
	deprecated: Option<Date>,
	
	comments: Vec<String>,
}

impl Record
{
	fn parse(events: &mut PullEventParser, records: &mut Records) -> Result<bool, LanguageSubtagRegistryFileParseError>
	{
		let (type_, record_fields, no_more_records) = RecordFields::default().parse(events)?;
		
		let type_ = type_.ok_or(LanguageSubtagRegistryFileParseError::MissingTypeField)?;
		
		Self::parse_record_from_fields(type_, records, record_fields).map_err(|cause| LanguageSubtagRegistryFileParseError::Record(cause, type_))?;
		
		Ok(no_more_records)
	}
	
	#[inline(always)]
	fn parse_record_from_fields(type_: Type, records: &mut Records, record_fields: RecordFields) -> Result<(), RecordParseError>
	{
		use MissingFieldError::MissingTagOrSubtag;
		use Type::*;
		
		let RecordFields { tag_or_subtag, description, added, deprecated, preferred_value, prefix, suppress_script, macrolanguage, scope, comments } = record_fields;
		
		let record = Self::parsed(description, added, deprecated, comments)?;
		let tag_or_subtag = Self::missing_single_field(tag_or_subtag, MissingTagOrSubtag)?;
		match type_
		{
			extlang => Self::record_using_subtag(&mut records.extlang, record, tag_or_subtag, preferred_value, prefix, suppress_script, macrolanguage, scope)?,
			
			grandfathered => Self::record_using_tag(&mut records.grandfathered, record, tag_or_subtag, preferred_value, prefix, suppress_script, macrolanguage, scope)?,
			
			language => Self::record_using_subtag(&mut records.language, record, tag_or_subtag, preferred_value, prefix, suppress_script, macrolanguage, scope)?,
			
			redundant => Self::record_using_tag(&mut records.redundant, record, tag_or_subtag, preferred_value, prefix, suppress_script, macrolanguage, scope)?,
			
			region => Self::record_using_subtag(&mut records.region, record, tag_or_subtag, preferred_value, prefix, suppress_script, macrolanguage, scope)?,
			
			script => Self::record_using_subtag(&mut records.script, record, tag_or_subtag, preferred_value, prefix, suppress_script, macrolanguage, scope)?,
			
			variant => Self::record_using_subtag(&mut records.variant, record, tag_or_subtag, preferred_value, prefix, suppress_script, macrolanguage, scope)?,
		};
		
		Ok(())
	}
	
	
	#[inline(always)]
	fn parsed(description: Vec<String>, added: Option<Date>, deprecated: Option<Date>, comments: Vec<String>) -> Result<Self, RecordParseError>
	{
		use MissingFieldError::*;
		Ok
		(
			Self
			{
				description: Self::missing_multiple_field(description, MissingDescription)?,
				added: Self::missing_single_field(added, MissingAdded)?,
				deprecated,
				comments,
			}
		)
	}
	
	#[inline(always)]
	fn missing_single_field<T>(field: Option<T>, error: MissingFieldError) -> Result<T, RecordParseError>
	{
		field.ok_or(RecordParseError::MissingField(error))
	}
	
	#[inline(always)]
	fn missing_multiple_field<T>(field: Vec<T>, error: MissingFieldError) -> Result<Vec<T>, RecordParseError>
	{
		if field.is_empty()
		{
			Err(RecordParseError::MissingField(error))
		}
		else
		{
			Ok(field)
		}
	}
	
	#[inline(always)]
	fn record_using_subtag<PR: ParseRecord>(hash_map: &mut HashMap<PR::Key, (Record, PR)>, record: Record, tag_or_subtag: TagOrSubtag, preferred_value: Option<String>, prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<(), RecordParseError>
	{
		use TagOrSubtag::*;
		match tag_or_subtag
		{
			Subtag(subtag) => Self::record(record, hash_map, subtag, preferred_value, prefix, suppress_script, macrolanguage, scope),
			
			Tag(_) => Err(RecordParseError::TagUsedInsteadOfSubtag),
		}
	}
	
	#[inline(always)]
	fn record_using_tag<PR: ParseRecord>(hash_map: &mut HashMap<PR::Key, (Record, PR)>, record: Record, tag_or_subtag: TagOrSubtag, preferred_value: Option<String>, prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<(), RecordParseError>
	{
		use TagOrSubtag::*;
		match tag_or_subtag
		{
			Subtag(_) => Err(RecordParseError::SubtagUsedInsteadOfTag),
			
			Tag(tag) => Self::record(record, hash_map, tag, preferred_value, prefix, suppress_script, macrolanguage, scope),
		}
	}
	
	const DoubleDot: &'static str = "..";
	
	#[inline(always)]
	fn record<PR: ParseRecord>(record: Record, hash_map: &mut HashMap<PR::Key, (Record, PR)>, tag_or_subtag: String, preferred_value: Option<String>, prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<(), RecordParseError>
	{
		let record_specific = PR::parse(preferred_value, prefix, suppress_script, macrolanguage, scope)?;
		
		if tag_or_subtag.contains(Self::DoubleDot)
		{
			Self::record_range(hash_map, tag_or_subtag, record, record_specific)?
		}
		else
		{
			let key = PR::parse_key( tag_or_subtag)?;
			Self::insert(hash_map, key, record, record_specific)?
		}
		Ok(())
	}
	
	#[inline(always)]
	fn record_range<PR: ParseRecord>(hash_map: &mut HashMap<PR::Key, (Record, PR)>, tag_or_subtag_range: String, record: Record, record_specific: PR) -> Result<(), KeyParseError>
	{
		#[inline(always)]
		fn split_next<'a>(split: &mut SplitN<'a, &str>) -> &'a str
		{
			let next = split.next();
			unsafe { next.unwrap_unchecked() }
		}
		
		let mut split = tag_or_subtag_range.splitn(2, Self::DoubleDot);
		let inclusive_from = split_next(&mut split);
		let inclusive_to = split_next(&mut split);
		
		if inclusive_from.len() != inclusive_to.len()
		{
			Err(TagOrSubtagRangeError::RangeKeysDifferentLengths { inclusive_from: inclusive_from.to_string(), inclusive_to: inclusive_to.to_string() })?
		}
		
		let keys = PR::parse_key_range(inclusive_from, inclusive_to)?;
		for key in keys
		{
			Self::insert(hash_map, key.clone(), record.clone(), record_specific.clone())?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn insert<PR: ParseRecord>(hash_map: &mut HashMap<PR::Key, (Record, PR)>, key: PR::Key, record: Record, record_specific: PR) -> Result<(), KeyParseError>
	{
		if hash_map.insert(key, (record, record_specific)).is_some()
		{
			Err(KeyParseError::DuplicateRecord)
		}
		else
		{
			Ok(())
		}
	}
}
