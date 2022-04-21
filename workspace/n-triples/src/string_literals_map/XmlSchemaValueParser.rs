// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parser.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct XmlSchemaValueParser<'string_literals_map, SP: StrParser<'string_literals_map>, const PathDepth: usize>
{
	key: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>,
	
	marker: PhantomData<(&'string_literals_map (), SP)>,
}

impl<'string_literals_map, SP: StrParser<'string_literals_map>, const PathDepth: usize> XmlSchemaValueParser<'string_literals_map, SP, PathDepth>
{
	#[inline(always)]
	const fn new(key: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>) -> Self
	{
		Self
		{
			key,
			
			marker: PhantomData,
		}
	}
	
	#[inline(always)]
	fn parse_domain_type<'a: 'string_literals_map, DomainType: TryFrom<SP::Item, Error=TryFromError>, TryFromError: error::Error>(self, string_literals_map: &'string_literals_map StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>) ->
	StringLiteralsMapValuesIterator<'a, 'string_literals_map, TryFromStrParser<'string_literals_map, SP, DomainType, TryFromError>>
	{
		self.get_inner::<TryFromStrParser::<SP, DomainType, TryFromError>>(string_literals_map)
	}
	
	#[inline(always)]
	fn parse_from_str_like<'a: 'string_literals_map>(self, string_literals_map: &'string_literals_map StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>) -> StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP>
	{
		self.get_inner::<SP>(string_literals_map)
	}
	
	#[inline(always)]
	fn get_inner<'a: 'string_literals_map, SP2: StrParser<'string_literals_map>>(self, string_literals_map: &'string_literals_map StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>) -> StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP2>
	{
		match string_literals_map.get(unsafe { transmute(&self.key) })
		{
			None => StringLiteralsMapValuesIterator::Empty,
			
			Some(non_empty) => StringLiteralsMapValuesIterator::new(non_empty.as_slice()),
		}
	}
}

impl<'string_literals_map, const PathDepth: usize> XmlSchemaValueParser<'string_literals_map, &'string_literals_map str, PathDepth>
{
	#[allow(missing_docs)]
	pub const String: Self = Self::new(AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaString);
}

impl<'string_literals_map, const PathDepth: usize> XmlSchemaValueParser<'string_literals_map, bool, PathDepth>
{
	#[allow(missing_docs)]
	pub const Boolean: Self = Self::new(AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaBoolean);
}

impl<'string_literals_map, const PathDepth: usize> XmlSchemaValueParser<'string_literals_map, i64, PathDepth>
{
	#[allow(missing_docs)]
	pub const Integer: Self = Self::new(AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaInteger);
}

impl<'string_literals_map, const PathDepth: usize> XmlSchemaValueParser<'string_literals_map, DateTime<FixedOffset>, PathDepth>
{
	#[allow(missing_docs)]
	pub const DateTime: Self = Self::new(AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaDateTime);
}
