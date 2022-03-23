// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Normal
{
	language: Language,
	
	script: Option<IanaRegisteredIso15924ScriptCode>,
	
	region: Option<IanaRegisteredRegionCode>,

	variants: HashSet<Variant>,

	extensions: HashMap<Singleton, Extension>,

	private_use: Option<PrivateUse>,
}

impl From<Language> for Normal
{
	#[inline(always)]
	fn from(language: Language) -> Self
	{
		Self
		{
			language,
			
			script: None,
			
			region: None,
		
			variants: HashSet::default(),
		
			extensions: HashMap::default(),
		
			private_use: None,
		}
	}
}

impl Normal
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn language(&self) -> &Language
	{
		&self.language
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn script(&self) -> Option<IanaRegisteredIso15924ScriptCode>
	{
		self.script
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn region(&self) -> Option<IanaRegisteredRegionCode>
	{
		self.region
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn variants(&self) -> &HashSet<Variant>
	{
		&self.variants
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn extensions(&self) -> &HashMap<Singleton, Extension>
	{
		&self.extensions
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn private_use(&self) -> Option<&PrivateUse>
	{
		self.private_use.as_ref()
	}
}
