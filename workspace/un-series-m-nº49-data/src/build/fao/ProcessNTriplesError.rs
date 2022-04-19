// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Eq, PartialEq)]
enum ProcessNTriplesError<'a>
{
	NTriplesParse(NTriplesParseError),
	
	MoreThanOne(MoreThanOneError),
	
	MissingPredicatesForSubject,
	
	OnlyOneXmlSchemaStringLiteral(OnlyOneXmlSchemaStringLiteralError<'a>),
}

impl<'a> From<NTriplesParseError> for ProcessNTriplesError<'a>
{
	#[inline(always)]
	fn from(cause: NTriplesParseError) -> Self
	{
		ProcessNTriplesError::NTriplesParse(cause)
	}
}

impl<'a> From<MoreThanOneError> for ProcessNTriplesError<'a>
{
	#[inline(always)]
	fn from(cause: MoreThanOneError) -> Self
	{
		ProcessNTriplesError::MoreThanOne(cause)
	}
}

impl<'a> From<OnlyOneXmlSchemaStringLiteralError<'a>> for ProcessNTriplesError<'a>
{
	#[inline(always)]
	fn from(cause: OnlyOneXmlSchemaStringLiteralError<'a>) -> Self
	{
		ProcessNTriplesError::OnlyOneXmlSchemaStringLiteral(cause)
	}
}
