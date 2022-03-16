// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct LanguageRecord
{
	primary_language_subtag: Option<String>,
	
	suppress_script: Option<String>,
	
	macrolanguage: Option<String>,
	
	scope: Scope,
}

impl ParseRecord for LanguageRecord
{
	type Key = LanguageSubtag;
	
	#[inline(always)]
	fn parse_key_range(inclusive_from: &str, inclusive_to: &str) -> Result<&'static [Self::Key], TagOrSubtagRangeError>
	{
		use LanguageSubtag::Three;
		
		static PrivateUseRange: [LanguageSubtag; 520] =
		[
			Three([b'q', b'a', b'a']),
			Three([b'q', b'a', b'b']),
			Three([b'q', b'a', b'c']),
			Three([b'q', b'a', b'd']),
			Three([b'q', b'a', b'e']),
			Three([b'q', b'a', b'f']),
			Three([b'q', b'a', b'g']),
			Three([b'q', b'a', b'h']),
			Three([b'q', b'a', b'i']),
			Three([b'q', b'a', b'j']),
			Three([b'q', b'a', b'k']),
			Three([b'q', b'a', b'l']),
			Three([b'q', b'a', b'm']),
			Three([b'q', b'a', b'n']),
			Three([b'q', b'a', b'o']),
			Three([b'q', b'a', b'p']),
			Three([b'q', b'a', b'q']),
			Three([b'q', b'a', b'r']),
			Three([b'q', b'a', b's']),
			Three([b'q', b'a', b't']),
			Three([b'q', b'a', b'u']),
			Three([b'q', b'a', b'v']),
			Three([b'q', b'a', b'w']),
			Three([b'q', b'a', b'x']),
			Three([b'q', b'a', b'y']),
			Three([b'q', b'a', b'z']),
			
			Three([b'q', b'b', b'a']),
			Three([b'q', b'b', b'b']),
			Three([b'q', b'b', b'c']),
			Three([b'q', b'b', b'd']),
			Three([b'q', b'b', b'e']),
			Three([b'q', b'b', b'f']),
			Three([b'q', b'b', b'g']),
			Three([b'q', b'b', b'h']),
			Three([b'q', b'b', b'i']),
			Three([b'q', b'b', b'j']),
			Three([b'q', b'b', b'k']),
			Three([b'q', b'b', b'l']),
			Three([b'q', b'b', b'm']),
			Three([b'q', b'b', b'n']),
			Three([b'q', b'b', b'o']),
			Three([b'q', b'b', b'p']),
			Three([b'q', b'b', b'q']),
			Three([b'q', b'b', b'r']),
			Three([b'q', b'b', b's']),
			Three([b'q', b'b', b't']),
			Three([b'q', b'b', b'u']),
			Three([b'q', b'b', b'v']),
			Three([b'q', b'b', b'w']),
			Three([b'q', b'b', b'x']),
			Three([b'q', b'b', b'y']),
			Three([b'q', b'b', b'z']),
			
			Three([b'q', b'c', b'a']),
			Three([b'q', b'c', b'b']),
			Three([b'q', b'c', b'c']),
			Three([b'q', b'c', b'd']),
			Three([b'q', b'c', b'e']),
			Three([b'q', b'c', b'f']),
			Three([b'q', b'c', b'g']),
			Three([b'q', b'c', b'h']),
			Three([b'q', b'c', b'i']),
			Three([b'q', b'c', b'j']),
			Three([b'q', b'c', b'k']),
			Three([b'q', b'c', b'l']),
			Three([b'q', b'c', b'm']),
			Three([b'q', b'c', b'n']),
			Three([b'q', b'c', b'o']),
			Three([b'q', b'c', b'p']),
			Three([b'q', b'c', b'q']),
			Three([b'q', b'c', b'r']),
			Three([b'q', b'c', b's']),
			Three([b'q', b'c', b't']),
			Three([b'q', b'c', b'u']),
			Three([b'q', b'c', b'v']),
			Three([b'q', b'c', b'w']),
			Three([b'q', b'c', b'x']),
			Three([b'q', b'c', b'y']),
			Three([b'q', b'c', b'z']),
			
			Three([b'q', b'd', b'a']),
			Three([b'q', b'd', b'b']),
			Three([b'q', b'd', b'c']),
			Three([b'q', b'd', b'd']),
			Three([b'q', b'd', b'e']),
			Three([b'q', b'd', b'f']),
			Three([b'q', b'd', b'g']),
			Three([b'q', b'd', b'h']),
			Three([b'q', b'd', b'i']),
			Three([b'q', b'd', b'j']),
			Three([b'q', b'd', b'k']),
			Three([b'q', b'd', b'l']),
			Three([b'q', b'd', b'm']),
			Three([b'q', b'd', b'n']),
			Three([b'q', b'd', b'o']),
			Three([b'q', b'd', b'p']),
			Three([b'q', b'd', b'q']),
			Three([b'q', b'd', b'r']),
			Three([b'q', b'd', b's']),
			Three([b'q', b'd', b't']),
			Three([b'q', b'd', b'u']),
			Three([b'q', b'd', b'v']),
			Three([b'q', b'd', b'w']),
			Three([b'q', b'd', b'x']),
			Three([b'q', b'd', b'y']),
			Three([b'q', b'd', b'z']),
			
			Three([b'q', b'e', b'a']),
			Three([b'q', b'e', b'b']),
			Three([b'q', b'e', b'c']),
			Three([b'q', b'e', b'd']),
			Three([b'q', b'e', b'e']),
			Three([b'q', b'e', b'f']),
			Three([b'q', b'e', b'g']),
			Three([b'q', b'e', b'h']),
			Three([b'q', b'e', b'i']),
			Three([b'q', b'e', b'j']),
			Three([b'q', b'e', b'k']),
			Three([b'q', b'e', b'l']),
			Three([b'q', b'e', b'm']),
			Three([b'q', b'e', b'n']),
			Three([b'q', b'e', b'o']),
			Three([b'q', b'e', b'p']),
			Three([b'q', b'e', b'q']),
			Three([b'q', b'e', b'r']),
			Three([b'q', b'e', b's']),
			Three([b'q', b'e', b't']),
			Three([b'q', b'e', b'u']),
			Three([b'q', b'e', b'v']),
			Three([b'q', b'e', b'w']),
			Three([b'q', b'e', b'x']),
			Three([b'q', b'e', b'y']),
			Three([b'q', b'e', b'z']),
			
			Three([b'q', b'f', b'a']),
			Three([b'q', b'f', b'b']),
			Three([b'q', b'f', b'c']),
			Three([b'q', b'f', b'd']),
			Three([b'q', b'f', b'e']),
			Three([b'q', b'f', b'f']),
			Three([b'q', b'f', b'g']),
			Three([b'q', b'f', b'h']),
			Three([b'q', b'f', b'i']),
			Three([b'q', b'f', b'j']),
			Three([b'q', b'f', b'k']),
			Three([b'q', b'f', b'l']),
			Three([b'q', b'f', b'm']),
			Three([b'q', b'f', b'n']),
			Three([b'q', b'f', b'o']),
			Three([b'q', b'f', b'p']),
			Three([b'q', b'f', b'q']),
			Three([b'q', b'f', b'r']),
			Three([b'q', b'f', b's']),
			Three([b'q', b'f', b't']),
			Three([b'q', b'f', b'u']),
			Three([b'q', b'f', b'v']),
			Three([b'q', b'f', b'w']),
			Three([b'q', b'f', b'x']),
			Three([b'q', b'f', b'y']),
			Three([b'q', b'f', b'z']),
			
			Three([b'q', b'g', b'a']),
			Three([b'q', b'g', b'b']),
			Three([b'q', b'g', b'c']),
			Three([b'q', b'g', b'd']),
			Three([b'q', b'g', b'e']),
			Three([b'q', b'g', b'f']),
			Three([b'q', b'g', b'g']),
			Three([b'q', b'g', b'h']),
			Three([b'q', b'g', b'i']),
			Three([b'q', b'g', b'j']),
			Three([b'q', b'g', b'k']),
			Three([b'q', b'g', b'l']),
			Three([b'q', b'g', b'm']),
			Three([b'q', b'g', b'n']),
			Three([b'q', b'g', b'o']),
			Three([b'q', b'g', b'p']),
			Three([b'q', b'g', b'q']),
			Three([b'q', b'g', b'r']),
			Three([b'q', b'g', b's']),
			Three([b'q', b'g', b't']),
			Three([b'q', b'g', b'u']),
			Three([b'q', b'g', b'v']),
			Three([b'q', b'g', b'w']),
			Three([b'q', b'g', b'x']),
			Three([b'q', b'g', b'y']),
			Three([b'q', b'g', b'z']),
			
			Three([b'q', b'h', b'a']),
			Three([b'q', b'h', b'b']),
			Three([b'q', b'h', b'c']),
			Three([b'q', b'h', b'd']),
			Three([b'q', b'h', b'e']),
			Three([b'q', b'h', b'f']),
			Three([b'q', b'h', b'g']),
			Three([b'q', b'h', b'h']),
			Three([b'q', b'h', b'i']),
			Three([b'q', b'h', b'j']),
			Three([b'q', b'h', b'k']),
			Three([b'q', b'h', b'l']),
			Three([b'q', b'h', b'm']),
			Three([b'q', b'h', b'n']),
			Three([b'q', b'h', b'o']),
			Three([b'q', b'h', b'p']),
			Three([b'q', b'h', b'q']),
			Three([b'q', b'h', b'r']),
			Three([b'q', b'h', b's']),
			Three([b'q', b'h', b't']),
			Three([b'q', b'h', b'u']),
			Three([b'q', b'h', b'v']),
			Three([b'q', b'h', b'w']),
			Three([b'q', b'h', b'x']),
			Three([b'q', b'h', b'y']),
			Three([b'q', b'h', b'z']),
			
			Three([b'q', b'i', b'a']),
			Three([b'q', b'i', b'b']),
			Three([b'q', b'i', b'c']),
			Three([b'q', b'i', b'd']),
			Three([b'q', b'i', b'e']),
			Three([b'q', b'i', b'f']),
			Three([b'q', b'i', b'g']),
			Three([b'q', b'i', b'h']),
			Three([b'q', b'i', b'i']),
			Three([b'q', b'i', b'j']),
			Three([b'q', b'i', b'k']),
			Three([b'q', b'i', b'l']),
			Three([b'q', b'i', b'm']),
			Three([b'q', b'i', b'n']),
			Three([b'q', b'i', b'o']),
			Three([b'q', b'i', b'p']),
			Three([b'q', b'i', b'q']),
			Three([b'q', b'i', b'r']),
			Three([b'q', b'i', b's']),
			Three([b'q', b'i', b't']),
			Three([b'q', b'i', b'u']),
			Three([b'q', b'i', b'v']),
			Three([b'q', b'i', b'w']),
			Three([b'q', b'i', b'x']),
			Three([b'q', b'i', b'y']),
			Three([b'q', b'i', b'z']),
			
			Three([b'q', b'j', b'a']),
			Three([b'q', b'j', b'b']),
			Three([b'q', b'j', b'c']),
			Three([b'q', b'j', b'd']),
			Three([b'q', b'j', b'e']),
			Three([b'q', b'j', b'f']),
			Three([b'q', b'j', b'g']),
			Three([b'q', b'j', b'h']),
			Three([b'q', b'j', b'i']),
			Three([b'q', b'j', b'j']),
			Three([b'q', b'j', b'k']),
			Three([b'q', b'j', b'l']),
			Three([b'q', b'j', b'm']),
			Three([b'q', b'j', b'n']),
			Three([b'q', b'j', b'o']),
			Three([b'q', b'j', b'p']),
			Three([b'q', b'j', b'q']),
			Three([b'q', b'j', b'r']),
			Three([b'q', b'j', b's']),
			Three([b'q', b'j', b't']),
			Three([b'q', b'j', b'u']),
			Three([b'q', b'j', b'v']),
			Three([b'q', b'j', b'w']),
			Three([b'q', b'j', b'x']),
			Three([b'q', b'j', b'y']),
			Three([b'q', b'j', b'z']),
			
			Three([b'q', b'k', b'a']),
			Three([b'q', b'k', b'b']),
			Three([b'q', b'k', b'c']),
			Three([b'q', b'k', b'd']),
			Three([b'q', b'k', b'e']),
			Three([b'q', b'k', b'f']),
			Three([b'q', b'k', b'g']),
			Three([b'q', b'k', b'h']),
			Three([b'q', b'k', b'i']),
			Three([b'q', b'k', b'j']),
			Three([b'q', b'k', b'k']),
			Three([b'q', b'k', b'l']),
			Three([b'q', b'k', b'm']),
			Three([b'q', b'k', b'n']),
			Three([b'q', b'k', b'o']),
			Three([b'q', b'k', b'p']),
			Three([b'q', b'k', b'q']),
			Three([b'q', b'k', b'r']),
			Three([b'q', b'k', b's']),
			Three([b'q', b'k', b't']),
			Three([b'q', b'k', b'u']),
			Three([b'q', b'k', b'v']),
			Three([b'q', b'k', b'w']),
			Three([b'q', b'k', b'x']),
			Three([b'q', b'k', b'y']),
			Three([b'q', b'k', b'z']),
			
			Three([b'q', b'l', b'a']),
			Three([b'q', b'l', b'b']),
			Three([b'q', b'l', b'c']),
			Three([b'q', b'l', b'd']),
			Three([b'q', b'l', b'e']),
			Three([b'q', b'l', b'f']),
			Three([b'q', b'l', b'g']),
			Three([b'q', b'l', b'h']),
			Three([b'q', b'l', b'i']),
			Three([b'q', b'l', b'j']),
			Three([b'q', b'l', b'k']),
			Three([b'q', b'l', b'l']),
			Three([b'q', b'l', b'm']),
			Three([b'q', b'l', b'n']),
			Three([b'q', b'l', b'o']),
			Three([b'q', b'l', b'p']),
			Three([b'q', b'l', b'q']),
			Three([b'q', b'l', b'r']),
			Three([b'q', b'l', b's']),
			Three([b'q', b'l', b't']),
			Three([b'q', b'l', b'u']),
			Three([b'q', b'l', b'v']),
			Three([b'q', b'l', b'w']),
			Three([b'q', b'l', b'x']),
			Three([b'q', b'l', b'y']),
			Three([b'q', b'l', b'z']),
			
			Three([b'q', b'm', b'a']),
			Three([b'q', b'm', b'b']),
			Three([b'q', b'm', b'c']),
			Three([b'q', b'm', b'd']),
			Three([b'q', b'm', b'e']),
			Three([b'q', b'm', b'f']),
			Three([b'q', b'm', b'g']),
			Three([b'q', b'm', b'h']),
			Three([b'q', b'm', b'i']),
			Three([b'q', b'm', b'j']),
			Three([b'q', b'm', b'k']),
			Three([b'q', b'm', b'l']),
			Three([b'q', b'm', b'm']),
			Three([b'q', b'm', b'n']),
			Three([b'q', b'm', b'o']),
			Three([b'q', b'm', b'p']),
			Three([b'q', b'm', b'q']),
			Three([b'q', b'm', b'r']),
			Three([b'q', b'm', b's']),
			Three([b'q', b'm', b't']),
			Three([b'q', b'm', b'u']),
			Three([b'q', b'm', b'v']),
			Three([b'q', b'm', b'w']),
			Three([b'q', b'm', b'x']),
			Three([b'q', b'm', b'y']),
			Three([b'q', b'm', b'z']),
			
			Three([b'q', b'n', b'a']),
			Three([b'q', b'n', b'b']),
			Three([b'q', b'n', b'c']),
			Three([b'q', b'n', b'd']),
			Three([b'q', b'n', b'e']),
			Three([b'q', b'n', b'f']),
			Three([b'q', b'n', b'g']),
			Three([b'q', b'n', b'h']),
			Three([b'q', b'n', b'i']),
			Three([b'q', b'n', b'j']),
			Three([b'q', b'n', b'k']),
			Three([b'q', b'n', b'l']),
			Three([b'q', b'n', b'm']),
			Three([b'q', b'n', b'n']),
			Three([b'q', b'n', b'o']),
			Three([b'q', b'n', b'p']),
			Three([b'q', b'n', b'q']),
			Three([b'q', b'n', b'r']),
			Three([b'q', b'n', b's']),
			Three([b'q', b'n', b't']),
			Three([b'q', b'n', b'u']),
			Three([b'q', b'n', b'v']),
			Three([b'q', b'n', b'w']),
			Three([b'q', b'n', b'x']),
			Three([b'q', b'n', b'y']),
			Three([b'q', b'n', b'z']),
			
			Three([b'q', b'o', b'a']),
			Three([b'q', b'o', b'b']),
			Three([b'q', b'o', b'c']),
			Three([b'q', b'o', b'd']),
			Three([b'q', b'o', b'e']),
			Three([b'q', b'o', b'f']),
			Three([b'q', b'o', b'g']),
			Three([b'q', b'o', b'h']),
			Three([b'q', b'o', b'i']),
			Three([b'q', b'o', b'j']),
			Three([b'q', b'o', b'k']),
			Three([b'q', b'o', b'l']),
			Three([b'q', b'o', b'm']),
			Three([b'q', b'o', b'n']),
			Three([b'q', b'o', b'o']),
			Three([b'q', b'o', b'p']),
			Three([b'q', b'o', b'q']),
			Three([b'q', b'o', b'r']),
			Three([b'q', b'o', b's']),
			Three([b'q', b'o', b't']),
			Three([b'q', b'o', b'u']),
			Three([b'q', b'o', b'v']),
			Three([b'q', b'o', b'w']),
			Three([b'q', b'o', b'x']),
			Three([b'q', b'o', b'y']),
			Three([b'q', b'o', b'z']),
			
			Three([b'q', b'p', b'a']),
			Three([b'q', b'p', b'b']),
			Three([b'q', b'p', b'c']),
			Three([b'q', b'p', b'd']),
			Three([b'q', b'p', b'e']),
			Three([b'q', b'p', b'f']),
			Three([b'q', b'p', b'g']),
			Three([b'q', b'p', b'h']),
			Three([b'q', b'p', b'i']),
			Three([b'q', b'p', b'j']),
			Three([b'q', b'p', b'k']),
			Three([b'q', b'p', b'l']),
			Three([b'q', b'p', b'm']),
			Three([b'q', b'p', b'n']),
			Three([b'q', b'p', b'o']),
			Three([b'q', b'p', b'p']),
			Three([b'q', b'p', b'q']),
			Three([b'q', b'p', b'r']),
			Three([b'q', b'p', b's']),
			Three([b'q', b'p', b't']),
			Three([b'q', b'p', b'u']),
			Three([b'q', b'p', b'v']),
			Three([b'q', b'p', b'w']),
			Three([b'q', b'p', b'x']),
			Three([b'q', b'p', b'y']),
			Three([b'q', b'p', b'z']),
			
			Three([b'q', b'q', b'a']),
			Three([b'q', b'q', b'b']),
			Three([b'q', b'q', b'c']),
			Three([b'q', b'q', b'd']),
			Three([b'q', b'q', b'e']),
			Three([b'q', b'q', b'f']),
			Three([b'q', b'q', b'g']),
			Three([b'q', b'q', b'h']),
			Three([b'q', b'q', b'i']),
			Three([b'q', b'q', b'j']),
			Three([b'q', b'q', b'k']),
			Three([b'q', b'q', b'l']),
			Three([b'q', b'q', b'm']),
			Three([b'q', b'q', b'n']),
			Three([b'q', b'q', b'o']),
			Three([b'q', b'q', b'p']),
			Three([b'q', b'q', b'q']),
			Three([b'q', b'q', b'r']),
			Three([b'q', b'q', b's']),
			Three([b'q', b'q', b't']),
			Three([b'q', b'q', b'u']),
			Three([b'q', b'q', b'v']),
			Three([b'q', b'q', b'w']),
			Three([b'q', b'q', b'x']),
			Three([b'q', b'q', b'y']),
			Three([b'q', b'q', b'z']),
			
			Three([b'q', b'r', b'a']),
			Three([b'q', b'r', b'b']),
			Three([b'q', b'r', b'c']),
			Three([b'q', b'r', b'd']),
			Three([b'q', b'r', b'e']),
			Three([b'q', b'r', b'f']),
			Three([b'q', b'r', b'g']),
			Three([b'q', b'r', b'h']),
			Three([b'q', b'r', b'i']),
			Three([b'q', b'r', b'j']),
			Three([b'q', b'r', b'k']),
			Three([b'q', b'r', b'l']),
			Three([b'q', b'r', b'm']),
			Three([b'q', b'r', b'n']),
			Three([b'q', b'r', b'o']),
			Three([b'q', b'r', b'p']),
			Three([b'q', b'r', b'q']),
			Three([b'q', b'r', b'r']),
			Three([b'q', b'r', b's']),
			Three([b'q', b'r', b't']),
			Three([b'q', b'r', b'u']),
			Three([b'q', b'r', b'v']),
			Three([b'q', b'r', b'w']),
			Three([b'q', b'r', b'x']),
			Three([b'q', b'r', b'y']),
			Three([b'q', b'r', b'z']),
			
			Three([b'q', b's', b'a']),
			Three([b'q', b's', b'b']),
			Three([b'q', b's', b'c']),
			Three([b'q', b's', b'd']),
			Three([b'q', b's', b'e']),
			Three([b'q', b's', b'f']),
			Three([b'q', b's', b'g']),
			Three([b'q', b's', b'h']),
			Three([b'q', b's', b'i']),
			Three([b'q', b's', b'j']),
			Three([b'q', b's', b'k']),
			Three([b'q', b's', b'l']),
			Three([b'q', b's', b'm']),
			Three([b'q', b's', b'n']),
			Three([b'q', b's', b'o']),
			Three([b'q', b's', b'p']),
			Three([b'q', b's', b'q']),
			Three([b'q', b's', b'r']),
			Three([b'q', b's', b's']),
			Three([b'q', b's', b't']),
			Three([b'q', b's', b'u']),
			Three([b'q', b's', b'v']),
			Three([b'q', b's', b'w']),
			Three([b'q', b's', b'x']),
			Three([b'q', b's', b'y']),
			Three([b'q', b's', b'z']),
			
			Three([b'q', b't', b'a']),
			Three([b'q', b't', b'b']),
			Three([b'q', b't', b'c']),
			Three([b'q', b't', b'd']),
			Three([b'q', b't', b'e']),
			Three([b'q', b't', b'f']),
			Three([b'q', b't', b'g']),
			Three([b'q', b't', b'h']),
			Three([b'q', b't', b'i']),
			Three([b'q', b't', b'j']),
			Three([b'q', b't', b'k']),
			Three([b'q', b't', b'l']),
			Three([b'q', b't', b'm']),
			Three([b'q', b't', b'n']),
			Three([b'q', b't', b'o']),
			Three([b'q', b't', b'p']),
			Three([b'q', b't', b'q']),
			Three([b'q', b't', b'r']),
			Three([b'q', b't', b's']),
			Three([b'q', b't', b't']),
			Three([b'q', b't', b'u']),
			Three([b'q', b't', b'v']),
			Three([b'q', b't', b'w']),
			Three([b'q', b't', b'x']),
			Three([b'q', b't', b'y']),
			Three([b'q', b't', b'z']),
		];
		
		match (inclusive_from, inclusive_to)
		{
			("qaa", "qtz") => Ok(&PrivateUseRange),
			
			_ => Err(TagOrSubtagRangeError::TypeDoesNotSupportSpecificRange { inclusive_from: inclusive_from.to_string(), inclusive_to: inclusive_from.to_string() }),
		}
	}
	
	#[inline(always)]
	fn parse_key(subtag: String) -> Result<Self::Key, KeyParseError>
	{
		use LanguageSubtag::*;
		
		match subtag.len()
		{
			2 => Ok(Two(Self::subtag_to_byte_array::<2>(&subtag)?)),
			
			3 => Ok(Three(Self::subtag_to_byte_array::<3>(&subtag)?)),
			
			length @ _ => Err(KeyParseError::SubtagInvalidLength { length, minimum: 2, maximum: 3, subtag: subtag.to_string() }),
		}
	}
	
	#[inline(always)]
	fn parse(preferred_value: Option<String>, prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<Self, RecordParseError>
	{
		if !prefix.is_empty()
		{
			Err(FieldNotPermittedError::PrefixNotPermittedInThisRecordType)?
		}
		
		Ok
		(
			Self
			{
				primary_language_subtag: preferred_value,
				
				suppress_script,
				
				macrolanguage,
				
				scope: scope.unwrap_or_default(),
			}
		)
	}
}
