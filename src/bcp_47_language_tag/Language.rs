// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Basically, a 2 - 8 byte alpha code.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Language
{
	#[allow(missing_docs)]
	Iso
	{
		#[allow(missing_docs)]
		shortest_iso_639_code: IanaRegisteredIso639Code,
		
		#[allow(missing_docs)]
		extension: Option<LanguageExtension>,
	},
	
	/// Reserved for future use.
	Reserved(ReservedLanguageSubtag),
	
	#[allow(missing_docs)]
	Registered(RegisteredLanguageSubtag),
}
