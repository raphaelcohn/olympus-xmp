// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Eq, PartialEq)]
enum ProcessNTriplesError<'predicate>
{
	NTriplesParse(NTriplesParseError),
	
	MissingPredicatesForSubject,
	
	OnlyOneString(GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<Infallible>>),
	
	OnlyOneBoolean(GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<ParseBoolError>>),
	
	OnlyOneInteger(GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<ParseIntError>>),
	
	OnlyOneDateTime(GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<ParseDateTimeError>>),
	
	OptionalString(GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<Infallible>>),
	
	OptionalBoolean(GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<ParseBoolError>>),
	
	OptionalInteger(GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<ParseIntError>>),
	
	OptionalDateTime(GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<ParseDateTimeError>>),
	
	OptionalAbsoluteInternationalizedResourceIdentifier(OptionalAbsoluteInternationalizedResourceIdentifierError<'predicate>),
	
	RemovePrefix(RemovePrefixError<'predicate, PathDepth>),
}

impl<'predicate> From<NTriplesParseError> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: NTriplesParseError) -> Self
	{
		ProcessNTriplesError::NTriplesParse(cause)
	}
}

impl<'predicate> From<GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<Infallible>>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<Infallible>>) -> Self
	{
		ProcessNTriplesError::OnlyOneString(cause)
	}
}

impl<'predicate> From<GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<ParseBoolError>>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<ParseBoolError>>) -> Self
	{
		ProcessNTriplesError::OnlyOneBoolean(cause)
	}
}

impl<'predicate> From<GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<ParseIntError>>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<ParseIntError>>) -> Self
	{
		ProcessNTriplesError::OnlyOneInteger(cause)
	}
}

impl<'predicate> From<GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<ParseDateTimeError>>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: GetXmlSchemaValueError<'predicate, OnlyOneXmlSchemaValueError<ParseDateTimeError>>) -> Self
	{
		ProcessNTriplesError::OnlyOneDateTime(cause)
	}
}

impl<'predicate> From<GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<Infallible>>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<Infallible>>) -> Self
	{
		ProcessNTriplesError::OptionalString(cause)
	}
}

impl<'predicate> From<GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<ParseBoolError>>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<ParseBoolError>>) -> Self
	{
		ProcessNTriplesError::OptionalBoolean(cause)
	}
}

impl<'predicate> From<GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<ParseIntError>>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<ParseIntError>>) -> Self
	{
		ProcessNTriplesError::OptionalInteger(cause)
	}
}

impl<'predicate> From<GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<ParseDateTimeError>>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: GetXmlSchemaValueError<'predicate, OptionalXmlSchemaValueError<ParseDateTimeError>>) -> Self
	{
		ProcessNTriplesError::OptionalDateTime(cause)
	}
}

impl<'predicate> From<OptionalAbsoluteInternationalizedResourceIdentifierError<'predicate>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: OptionalAbsoluteInternationalizedResourceIdentifierError<'predicate>) -> Self
	{
		ProcessNTriplesError::OptionalAbsoluteInternationalizedResourceIdentifier(cause)
	}
}

impl<'predicate> From<RemovePrefixError<'predicate, PathDepth>> for ProcessNTriplesError<'predicate>
{
	#[inline(always)]
	fn from(cause: RemovePrefixError<'predicate, PathDepth>) -> Self
	{
		ProcessNTriplesError::RemovePrefix(cause)
	}
}
