// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RecordFields
{
	tag_or_subtag: Option<TagOrSubtag>,
	
	description: Vec<String>,
	
	added: Option<Date>,
	
	deprecated: Option<Date>,
	
	preferred_value: Option<String>,
	
	prefix: Vec<String>,
	
	suppress_script: Option<String>,
	
	macrolanguage: Option<String>,
	
	scope: Option<Scope>,
	
	comments: Vec<String>,
}

impl RecordFields
{
	#[inline(always)]
	fn parse(mut self, events: &mut PullEventParser) -> Result<(Option<Type>, Self, bool), FieldError>
	{
		use FieldError::*;
		use FieldParseError::*;
		use TagOrSubtag::*;
		
		#[inline(always)]
		fn parse_field_multiple_string(field: &mut Vec<String>, event: FieldEvent)
		{
			field.push(event.field_body().to_string())
		}
		
		#[inline(always)]
		fn parse_field_single_from_str<FS: FromStr<Err=E>, E: error::Error>(error: impl FnOnce(FieldParseError<E>) -> FieldError, field: &mut Option<FS>, event: FieldEvent) -> Result<(), FieldError>
		{
			parse_single(error, field, event, |field_body| FS::from_str(field_body).map_err(FromString))
		}
		
		#[inline(always)]
		fn parse_field_single_string(error: impl FnOnce(FieldParseError<Infallible>) -> FieldError, field: &mut Option<String>, event: FieldEvent) -> Result<(), FieldError>
		{
			parse_single_infallible(error, field, event, |string| string)
		}
		
		#[inline(always)]
		fn parse_single_infallible<F>(error: impl FnOnce(FieldParseError<Infallible>) -> FieldError, field: &mut Option<F>, event: FieldEvent, parser: impl FnOnce(String) -> F) -> Result<(), FieldError>
		{
			parse_single(error, field, event, |field_body| Ok(parser(field_body.to_string())))
		}
		
		#[inline(always)]
		fn parse_single<F, E: error::Error>(error: impl FnOnce(FieldParseError<E>) -> FieldError, field: &mut Option<F>, event: FieldEvent, parser: impl FnOnce(&str) -> Result<F, FieldParseError<E>>) -> Result<(), FieldError>
		{
			if field.is_some()
			{
				return Err(error(Repeated))
			}
			*field = Some(parser(event.field_body()).map_err(error)?);
			Ok(())
		}
		
		let mut type_: Option<Type> = None;
		
		let no_more_records = loop
		{
			use Event::*;
			let event = match events.next()
			{
				None => break true,
				
				Some(Ok(NewRecord)) => break false,
				
				Some(Ok(Field(event))) => event,
				
				Some(Err(error)) => return Err(InvalidField(error)),
			};
			
			match event.field_name()
			{
				"Type" => parse_field_single_from_str(FieldType, &mut type_, event)?,
				
				"Tag" => parse_single_infallible(FieldTag, &mut self.tag_or_subtag, event, Tag)?,
				
				"Subtag" => parse_single_infallible(FieldSubtag, &mut self.tag_or_subtag, event, Subtag)?,
				
				"Description" => parse_field_multiple_string(&mut self.description, event),
				
				"Added" => parse_field_single_from_str(FieldAdded, &mut self.added, event)?,
				
				"Deprecated" => parse_field_single_from_str(FieldDeprecated, &mut self.deprecated, event)?,
				
				"Preferred-Value" => parse_field_single_string(FieldPreferredValue, &mut self.preferred_value, event)?,
				
				"Prefix" => parse_field_multiple_string(&mut self.prefix, event),
				
				"Suppress-Script" => parse_field_single_string(FieldSuppressScript, &mut self.suppress_script, event)?,
				
				"Macrolanguage" => parse_field_single_string(FieldMacrolanguage, &mut self.macrolanguage, event)?,
				
				"Scope" => parse_field_single_from_str(FieldScope, &mut self.scope, event)?,
				
				"Comments" => parse_field_multiple_string(&mut self.comments, event),
				
				_ => (),
			}
		};
		
		Ok((type_, self, no_more_records))
	}
}
