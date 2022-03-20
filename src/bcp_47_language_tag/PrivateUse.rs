// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Start with `x`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PrivateUse(OneWithOptionalSuffixes<PrivateUsePortion>);

impl Default for PrivateUse
{
	#[inline(always)]
	fn default() -> Self
	{
		const default: PrivateUsePortion = PrivateUsePortion::Alphanumeric7(Alphanumeric::new_array_unchecked(b"default"));
		Self(OneWithOptionalSuffixes::without_suffixes(default))
	}
}

impl PrivateUse
{
	#[inline(always)]
	fn parse(mut subtags: MemchrIterator<Hyphen>) -> Result<Self, PrivateUseSubtagsParseError>
	{
		use PrivateUseSubtagsParseError::*;
		
		Ok
		(
			Self
			(
				OneWithOptionalSuffixes
				{
					one: PrivateUsePortion::parse(next_or_error!(subtags, HasNoSubtags))?,
					
					suffixes:
					{
						let mut suffixes = vec![];
						for subtag in subtags
						{
							suffixes.push(PrivateUsePortion::parse(subtag)?)
						}
						suffixes
					}
				}
			)
		)
	}
}
