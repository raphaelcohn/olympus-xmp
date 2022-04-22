// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TryFromStrParser<'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, DomainType: TryFrom<SP::Item, Error=TryFromError>, TryFromError: 'static + error::Error, const PathDepth: usize>(PhantomData<(&'string_literals_map (), SP, DomainType, TryFromError)>)
where <SP as StrParser<'string_literals_map, PathDepth>>::Error: 'static;

impl<'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, DomainType: TryFrom<SP::Item, Error=TryFromError>, TryFromError: 'static + error::Error, const PathDepth: usize> StrParser<'string_literals_map, PathDepth> for TryFromStrParser<'string_literals_map, SP, DomainType, TryFromError, PathDepth>
where <SP as StrParser<'string_literals_map, PathDepth>>::Error: 'static
{
	type Item = DomainType;
	
	type Error = StringLiteralToDomainTypeParseError<SP::Error, TryFromError>;
	
	const Kind: XmlSchemaValueKind = SP::Kind;
	
	const Key: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth> = SP::Key;
	
	#[inline(always)]
	fn parse(value: &'string_literals_map str) -> Result<Self::Item, Self::Error>
	{
		use StringLiteralToDomainTypeParseError::*;
		
		match SP::parse(value)
		{
			Err(from_str_like_error) => Err(StrParse(from_str_like_error)),
			
			Ok(from_str_like_value) => match DomainType::try_from(from_str_like_value)
			{
				Err(try_from_error) => Err(TryFrom(try_from_error)),
				
				Ok(try_from_value) => Ok(try_from_value)
			},
		}
	}
}
