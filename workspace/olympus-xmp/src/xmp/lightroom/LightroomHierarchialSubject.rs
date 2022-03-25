// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Lightroom hierarchial subject.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LightroomHierarchialSubject<'a>(Vec<Subject<'a>>);

impl<'a> XmpAttributeValue<'a> for LightroomHierarchialSubject<'a>
{
	type Error = LightroomHierarchialSubjectParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		if raw.is_empty()
		{
			return Err(LightroomHierarchialSubjectParseError::NoSubjects)
		}
		
		const Pipe: char = '|';
		let mut iterator = raw.split(Pipe);
		
		let first =
		{
			let raw_subject = iterator.next();
			let raw_subject = unsafe { raw_subject.unwrap_unchecked() };
			Self::parse_raw_subject(raw_subject, 0)?
		};
		
		let mut subjects = Vec::with_capacity(1);
		subjects.push_unchecked(first);
		
		let mut index = 1;
		for raw_subject in iterator
		{
			subjects.push(Self::parse_raw_subject(raw_subject, index)?);
			index += 1;
		}
		
		Ok(Self(subjects))
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::LightroomHierarchialSubject(error)
	}
}

impl<'a> LightroomHierarchialSubject<'a>
{
	#[inline(always)]
	fn parse_raw_subject(raw_subject: &'a str, index: usize) -> Result<Subject<'a>, LightroomHierarchialSubjectParseError>
	{
		Subject::parse(raw_subject).map_err(|cause| LightroomHierarchialSubjectParseError::EmptySubject { index, cause })
	}
}
