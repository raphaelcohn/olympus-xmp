// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Obsolete as of IIM version 4.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IimSupplementalCategories(HashSet<IimCategoryCode>);

impl const Deref for IimSupplementalCategories
{
	type Target = HashSet<IimCategoryCode>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'a> XmpAttributeValue<'a> for IimSupplementalCategories
{
	type Error = IimSupplementalCategoriesParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		use IimSupplementalCategoriesParseError::*;
		
		const Space: char = ' ';
		let mut set = HashSet::with_capacity(Self::Maximum as usize);
		let mut iterator = raw.split(Space);
		for index in 0 .. Self::Maximum
		{
			match iterator.next()
			{
				None => return Ok(Self(set)),
				
				Some(potential) =>
				{
					let category_code = IimCategoryCode::parse(potential).map_err(|cause| InvalidSupplementalCategory { index, cause })?;
					if !set.insert(category_code)
					{
						return Err(DuplicateSupplementalCategory { category_code, index })
					}
				}
			}
		}
		
		if iterator.next().is_some()
		{
			Err(MoreThanEightSupplementalCategories)
		}
		else
		{
			Ok(Self(set))
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::IimSupplementalCategories(error)
	}
}

impl IimSupplementalCategories
{
	const Maximum: u8 = 8;
}
