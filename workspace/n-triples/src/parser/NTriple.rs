// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct NTriple<'a>
{
	pub(super) subject: Subject<'a>,
	
	pub(super) predicate: Predicate<'a>,
	
	pub(super) object: Object<'a>,
}

impl<'a> NTriple<'a>
{
	#[inline(always)]
	pub(super) fn parse(mut remaining_bytes: &'a [u8]) -> Result<(Self, Option<&'a [u8]>), NTripleParseError>
	{
		use NTripleParseError::*;
		
		let subject = Self::parse_subject(&mut remaining_bytes).map_err(SubjectParse)?;
		
		let predicate = Self::parse_predicate(&mut remaining_bytes).map_err(PredicateParse)?;
		
		let object = Self::parse_object(&mut remaining_bytes).map_err(ObjectParse)?;
		
		Self::parse_period(&mut remaining_bytes).map_err(PeriodParse)?;
		
		let option_remaining_bytes = Self::parse_comment_and_end_of_line(remaining_bytes).map_err(CommentParse)?;
		
		Ok
		(
			(
				Self
				{
					subject,
					predicate,
					object,
				},
				option_remaining_bytes
			)
		)
	}
	
	#[inline(always)]
	fn parse_subject(remaining_bytes: &mut &'a [u8]) -> Result<Subject<'a>, SubjectParseError>
	{
		use Subject::*;
		use SubjectParseError::*;
		match get_0(remaining_bytes)
		{
			Some(OpenAngleBracket) => self::AbsoluteInternationalizedResourceIdentifier::parse(remaining_bytes, AbsoluteInternationalizedResourceIdentifier).map_err(InternationalizedResourceIdentifierParse),
			
			Some(Underscore) => BlankNodeLabel::parse(remaining_bytes, BlankNode).map_err(BlankNodeLabelParse),
			
			Some(invalid) => return Err(CanNotStartWith(invalid)),
			
			None => return Err(ALineMustStartWithASubject),
		}
	}
	
	#[inline(always)]
	fn parse_predicate(remaining_bytes: &mut &'a [u8]) -> Result<Predicate<'a>, PredicateParseError>
	{
		use PredicateParseError::*;
		loop
		{
			match get_0(remaining_bytes)
			{
				Some(OpenAngleBracket) => return Predicate::parse(remaining_bytes, |predicate| predicate).map_err(PredicateParse),
				
				Some(Space | Tab) => continue,
				
				Some(invalid) => return Err(CanNotStartWith(invalid)),
				
				None => return Err(ALineMustContinueWithAPredicate),
			}
		}
	}
	
	#[inline(always)]
	fn parse_object(remaining_bytes: &mut &'a [u8]) -> Result<Object<'a>, ObjectParseError>
	{
		use Object::*;
		use ObjectParseError::*;
		loop
		{
			match get_0(remaining_bytes)
			{
				Some(OpenAngleBracket) => return self::AbsoluteInternationalizedResourceIdentifier::parse(remaining_bytes, AbsoluteInternationalizedResourceIdentifier).map_err(InternationalizedResourceIdentifierParse),
				
				Some(Underscore) => return BlankNodeLabel::parse(remaining_bytes, BlankNode).map_err(BlankNodeLabelParse),
				
				Some(DoubleQuote) => return StringLiteral::parse(remaining_bytes, Literal).map_err(StringLiteralParse),
				
				Some(Space | Tab) => continue,
				
				Some(invalid) => return Err(CanNotStartWith(invalid)),
				
				None => return Err(ALineMustContinueWithAnObject),
			}
		}
	}
	
	#[inline(always)]
	fn parse_period(remaining_bytes: &mut &'a [u8]) -> Result<(), PeriodParseError>
	{
		loop
		{
			use PeriodParseError::*;
			match get_0(remaining_bytes)
			{
				Some(Period) => return Ok(()),
				
				Some(Space | Tab) => continue,
				
				Some(invalid) => return Err(CanNotStartWith(invalid)),
				
				None => return Err(ALineMustContinueWithAPeriod),
			}
		}
	}
	
	#[inline(always)]
	fn parse_comment_and_end_of_line(mut remaining_bytes: &'a [u8]) -> Result<Option<&'a [u8]>, CommentParseError>
	{
		loop
		{
			match get_0(&mut remaining_bytes)
			{
				Some(Hash) => return Ok(memchr(LineFeed, remaining_bytes).map(|index| remaining_bytes.after_index(index))),
				
				Some(LineFeed) => return Ok(Some(remaining_bytes)),
				
				Some(Space | Tab) => continue,
				
				Some(invalid) => return Err(CommentParseError(invalid)),
				
				None => return Ok(None),
			}
		}
	}
	
}
