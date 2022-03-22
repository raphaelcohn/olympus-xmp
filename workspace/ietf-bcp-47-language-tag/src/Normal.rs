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
		
			variants: HashSet::new(),
		
			extensions: HashMap::new(),
		
			private_use: None,
		}
	}
}