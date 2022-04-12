// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Convenience.
pub trait Predicates<'a>
{
	#[allow(missing_docs)]
	fn get_simple_boolean(&self, predicate: &Predicate<'a>) -> Result<bool, GetPredicateError>;
	
	#[allow(missing_docs)]
	fn get_simple_integer(&self, predicate: &Predicate<'a>) -> Result<i64, GetPredicateError>;
	
	#[allow(missing_docs)]
	fn get_simple_string(&self, predicate: &Predicate<'a>) -> Result<&str, GetPredicateError>;
	
	#[allow(missing_docs)]
	fn get_absolute_internationalized_resource_identifiers(&self, predicate: &Predicate<'a>) -> Result<&[AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>], GetPredicateError>;
}

impl<'a> Predicates<'a> for HashMap<Predicate<'a>, Objects<'a>>
{
	#[inline(always)]
	fn get_simple_boolean(&self, predicate: &Predicate<'a>) -> Result<bool, GetPredicateError>
	{
		use GetPredicateError::*;
		
		let predicate = self.get(predicate).ok_or(PredicateAbsent)?;
		let vec = predicate.get_string_literals_by_absolute_internationalized_resource_identifier(&AbsoluteInternationalizedResourceIdentifier::XmlSchemaBoolean).ok_or(StringLiteralAbsent)?;
		let length = vec.len();
		debug_assert_ne!(length, 0);
		if length == 1
		{
			match vec.get_unchecked_safe(0).borrow()
			{
				"true" => Ok(true),
				
				"false" => Ok(false),
				
				_ => Err(StringLiteralIsNotABoolean),
			}
		}
		else
		{
			Err(MoreThanOneStringLiteral)
		}
	}
	
	#[inline(always)]
	fn get_simple_integer(&self, predicate: &Predicate<'a>) -> Result<i64, GetPredicateError>
	{
		use GetPredicateError::*;
		
		let predicate = self.get(predicate).ok_or(PredicateAbsent)?;
		let vec = predicate.get_string_literals_by_absolute_internationalized_resource_identifier(&AbsoluteInternationalizedResourceIdentifier::XmlSchemaInteger).ok_or(StringLiteralAbsent)?;
		let length = vec.len();
		debug_assert_ne!(length, 0);
		if length == 1
		{
			i64::from_str(vec.get_unchecked_safe(0).borrow()).map_err(StringLiteralIsNotAnInteger)
		}
		else
		{
			Err(MoreThanOneStringLiteral)
		}
	}
	
	#[inline(always)]
	fn get_simple_string(&self, predicate: &Predicate<'a>) -> Result<&str, GetPredicateError>
	{
		use GetPredicateError::*;
		
		let predicate = self.get(predicate).ok_or(PredicateAbsent)?;
		let vec = predicate.get_string_literals_by_absolute_internationalized_resource_identifier(&AbsoluteInternationalizedResourceIdentifier::XmlSchemaString).ok_or(StringLiteralAbsent)?;
		let length = vec.len();
		debug_assert_ne!(length, 0);
		if length == 1
		{
			Ok(vec.get_unchecked_safe(0).as_ref())
		}
		else
		{
			Err(MoreThanOneStringLiteral)
		}
	}
	
	#[inline(always)]
	fn get_absolute_internationalized_resource_identifiers(&self, predicate: &Predicate<'a>) -> Result<&[AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>], GetPredicateError>
	{
		use GetPredicateError::*;
		
		let predicate = self.get(predicate).ok_or(PredicateAbsent)?;
		Ok(predicate.absolute_internationalized_resource_identifiers())
	}
}
