// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct LanguageRecord
{
	primary_language_subtag: Option<String>,
	
	suppress_script: Option<[u8; 4]>,
	
	macrolanguage: Option<String>,
	
	scope: Scope,
}

impl ParseRecord for LanguageRecord
{
	type Key = LanguageSubtag;
	
	#[inline(always)]
	fn parse_key_range(inclusive_from: &str, inclusive_to: &str) -> Result<&'static [Self::Key], TagOrSubtagRangeError>
	{
		use LanguageSubtag::ThreeLowerCaseAlpha;
		
		static PrivateUseRange: [LanguageSubtag; 520] =
		[
			ThreeLowerCaseAlpha([q, a, a]),
			ThreeLowerCaseAlpha([q, a, b]),
			ThreeLowerCaseAlpha([q, a, c]),
			ThreeLowerCaseAlpha([q, a, d]),
			ThreeLowerCaseAlpha([q, a, e]),
			ThreeLowerCaseAlpha([q, a, f]),
			ThreeLowerCaseAlpha([q, a, g]),
			ThreeLowerCaseAlpha([q, a, h]),
			ThreeLowerCaseAlpha([q, a, i]),
			ThreeLowerCaseAlpha([q, a, j]),
			ThreeLowerCaseAlpha([q, a, k]),
			ThreeLowerCaseAlpha([q, a, l]),
			ThreeLowerCaseAlpha([q, a, m]),
			ThreeLowerCaseAlpha([q, a, n]),
			ThreeLowerCaseAlpha([q, a, o]),
			ThreeLowerCaseAlpha([q, a, p]),
			ThreeLowerCaseAlpha([q, a, q]),
			ThreeLowerCaseAlpha([q, a, r]),
			ThreeLowerCaseAlpha([q, a, s]),
			ThreeLowerCaseAlpha([q, a, t]),
			ThreeLowerCaseAlpha([q, a, u]),
			ThreeLowerCaseAlpha([q, a, v]),
			ThreeLowerCaseAlpha([q, a, w]),
			ThreeLowerCaseAlpha([q, a, x]),
			ThreeLowerCaseAlpha([q, a, y]),
			ThreeLowerCaseAlpha([q, a, z]),
			
			ThreeLowerCaseAlpha([q, b, a]),
			ThreeLowerCaseAlpha([q, b, b]),
			ThreeLowerCaseAlpha([q, b, c]),
			ThreeLowerCaseAlpha([q, b, d]),
			ThreeLowerCaseAlpha([q, b, e]),
			ThreeLowerCaseAlpha([q, b, f]),
			ThreeLowerCaseAlpha([q, b, g]),
			ThreeLowerCaseAlpha([q, b, h]),
			ThreeLowerCaseAlpha([q, b, i]),
			ThreeLowerCaseAlpha([q, b, j]),
			ThreeLowerCaseAlpha([q, b, k]),
			ThreeLowerCaseAlpha([q, b, l]),
			ThreeLowerCaseAlpha([q, b, m]),
			ThreeLowerCaseAlpha([q, b, n]),
			ThreeLowerCaseAlpha([q, b, o]),
			ThreeLowerCaseAlpha([q, b, p]),
			ThreeLowerCaseAlpha([q, b, q]),
			ThreeLowerCaseAlpha([q, b, r]),
			ThreeLowerCaseAlpha([q, b, s]),
			ThreeLowerCaseAlpha([q, b, t]),
			ThreeLowerCaseAlpha([q, b, u]),
			ThreeLowerCaseAlpha([q, b, v]),
			ThreeLowerCaseAlpha([q, b, w]),
			ThreeLowerCaseAlpha([q, b, x]),
			ThreeLowerCaseAlpha([q, b, y]),
			ThreeLowerCaseAlpha([q, b, z]),
			
			ThreeLowerCaseAlpha([q, c, a]),
			ThreeLowerCaseAlpha([q, c, b]),
			ThreeLowerCaseAlpha([q, c, c]),
			ThreeLowerCaseAlpha([q, c, d]),
			ThreeLowerCaseAlpha([q, c, e]),
			ThreeLowerCaseAlpha([q, c, f]),
			ThreeLowerCaseAlpha([q, c, g]),
			ThreeLowerCaseAlpha([q, c, h]),
			ThreeLowerCaseAlpha([q, c, i]),
			ThreeLowerCaseAlpha([q, c, j]),
			ThreeLowerCaseAlpha([q, c, k]),
			ThreeLowerCaseAlpha([q, c, l]),
			ThreeLowerCaseAlpha([q, c, m]),
			ThreeLowerCaseAlpha([q, c, n]),
			ThreeLowerCaseAlpha([q, c, o]),
			ThreeLowerCaseAlpha([q, c, p]),
			ThreeLowerCaseAlpha([q, c, q]),
			ThreeLowerCaseAlpha([q, c, r]),
			ThreeLowerCaseAlpha([q, c, s]),
			ThreeLowerCaseAlpha([q, c, t]),
			ThreeLowerCaseAlpha([q, c, u]),
			ThreeLowerCaseAlpha([q, c, v]),
			ThreeLowerCaseAlpha([q, c, w]),
			ThreeLowerCaseAlpha([q, c, x]),
			ThreeLowerCaseAlpha([q, c, y]),
			ThreeLowerCaseAlpha([q, c, z]),
			
			ThreeLowerCaseAlpha([q, d, a]),
			ThreeLowerCaseAlpha([q, d, b]),
			ThreeLowerCaseAlpha([q, d, c]),
			ThreeLowerCaseAlpha([q, d, d]),
			ThreeLowerCaseAlpha([q, d, e]),
			ThreeLowerCaseAlpha([q, d, f]),
			ThreeLowerCaseAlpha([q, d, g]),
			ThreeLowerCaseAlpha([q, d, h]),
			ThreeLowerCaseAlpha([q, d, i]),
			ThreeLowerCaseAlpha([q, d, j]),
			ThreeLowerCaseAlpha([q, d, k]),
			ThreeLowerCaseAlpha([q, d, l]),
			ThreeLowerCaseAlpha([q, d, m]),
			ThreeLowerCaseAlpha([q, d, n]),
			ThreeLowerCaseAlpha([q, d, o]),
			ThreeLowerCaseAlpha([q, d, p]),
			ThreeLowerCaseAlpha([q, d, q]),
			ThreeLowerCaseAlpha([q, d, r]),
			ThreeLowerCaseAlpha([q, d, s]),
			ThreeLowerCaseAlpha([q, d, t]),
			ThreeLowerCaseAlpha([q, d, u]),
			ThreeLowerCaseAlpha([q, d, v]),
			ThreeLowerCaseAlpha([q, d, w]),
			ThreeLowerCaseAlpha([q, d, x]),
			ThreeLowerCaseAlpha([q, d, y]),
			ThreeLowerCaseAlpha([q, d, z]),
			
			ThreeLowerCaseAlpha([q, e, a]),
			ThreeLowerCaseAlpha([q, e, b]),
			ThreeLowerCaseAlpha([q, e, c]),
			ThreeLowerCaseAlpha([q, e, d]),
			ThreeLowerCaseAlpha([q, e, e]),
			ThreeLowerCaseAlpha([q, e, f]),
			ThreeLowerCaseAlpha([q, e, g]),
			ThreeLowerCaseAlpha([q, e, h]),
			ThreeLowerCaseAlpha([q, e, i]),
			ThreeLowerCaseAlpha([q, e, j]),
			ThreeLowerCaseAlpha([q, e, k]),
			ThreeLowerCaseAlpha([q, e, l]),
			ThreeLowerCaseAlpha([q, e, m]),
			ThreeLowerCaseAlpha([q, e, n]),
			ThreeLowerCaseAlpha([q, e, o]),
			ThreeLowerCaseAlpha([q, e, p]),
			ThreeLowerCaseAlpha([q, e, q]),
			ThreeLowerCaseAlpha([q, e, r]),
			ThreeLowerCaseAlpha([q, e, s]),
			ThreeLowerCaseAlpha([q, e, t]),
			ThreeLowerCaseAlpha([q, e, u]),
			ThreeLowerCaseAlpha([q, e, v]),
			ThreeLowerCaseAlpha([q, e, w]),
			ThreeLowerCaseAlpha([q, e, x]),
			ThreeLowerCaseAlpha([q, e, y]),
			ThreeLowerCaseAlpha([q, e, z]),
			
			ThreeLowerCaseAlpha([q, f, a]),
			ThreeLowerCaseAlpha([q, f, b]),
			ThreeLowerCaseAlpha([q, f, c]),
			ThreeLowerCaseAlpha([q, f, d]),
			ThreeLowerCaseAlpha([q, f, e]),
			ThreeLowerCaseAlpha([q, f, f]),
			ThreeLowerCaseAlpha([q, f, g]),
			ThreeLowerCaseAlpha([q, f, h]),
			ThreeLowerCaseAlpha([q, f, i]),
			ThreeLowerCaseAlpha([q, f, j]),
			ThreeLowerCaseAlpha([q, f, k]),
			ThreeLowerCaseAlpha([q, f, l]),
			ThreeLowerCaseAlpha([q, f, m]),
			ThreeLowerCaseAlpha([q, f, n]),
			ThreeLowerCaseAlpha([q, f, o]),
			ThreeLowerCaseAlpha([q, f, p]),
			ThreeLowerCaseAlpha([q, f, q]),
			ThreeLowerCaseAlpha([q, f, r]),
			ThreeLowerCaseAlpha([q, f, s]),
			ThreeLowerCaseAlpha([q, f, t]),
			ThreeLowerCaseAlpha([q, f, u]),
			ThreeLowerCaseAlpha([q, f, v]),
			ThreeLowerCaseAlpha([q, f, w]),
			ThreeLowerCaseAlpha([q, f, x]),
			ThreeLowerCaseAlpha([q, f, y]),
			ThreeLowerCaseAlpha([q, f, z]),
			
			ThreeLowerCaseAlpha([q, g, a]),
			ThreeLowerCaseAlpha([q, g, b]),
			ThreeLowerCaseAlpha([q, g, c]),
			ThreeLowerCaseAlpha([q, g, d]),
			ThreeLowerCaseAlpha([q, g, e]),
			ThreeLowerCaseAlpha([q, g, f]),
			ThreeLowerCaseAlpha([q, g, g]),
			ThreeLowerCaseAlpha([q, g, h]),
			ThreeLowerCaseAlpha([q, g, i]),
			ThreeLowerCaseAlpha([q, g, j]),
			ThreeLowerCaseAlpha([q, g, k]),
			ThreeLowerCaseAlpha([q, g, l]),
			ThreeLowerCaseAlpha([q, g, m]),
			ThreeLowerCaseAlpha([q, g, n]),
			ThreeLowerCaseAlpha([q, g, o]),
			ThreeLowerCaseAlpha([q, g, p]),
			ThreeLowerCaseAlpha([q, g, q]),
			ThreeLowerCaseAlpha([q, g, r]),
			ThreeLowerCaseAlpha([q, g, s]),
			ThreeLowerCaseAlpha([q, g, t]),
			ThreeLowerCaseAlpha([q, g, u]),
			ThreeLowerCaseAlpha([q, g, v]),
			ThreeLowerCaseAlpha([q, g, w]),
			ThreeLowerCaseAlpha([q, g, x]),
			ThreeLowerCaseAlpha([q, g, y]),
			ThreeLowerCaseAlpha([q, g, z]),
			
			ThreeLowerCaseAlpha([q, h, a]),
			ThreeLowerCaseAlpha([q, h, b]),
			ThreeLowerCaseAlpha([q, h, c]),
			ThreeLowerCaseAlpha([q, h, d]),
			ThreeLowerCaseAlpha([q, h, e]),
			ThreeLowerCaseAlpha([q, h, f]),
			ThreeLowerCaseAlpha([q, h, g]),
			ThreeLowerCaseAlpha([q, h, h]),
			ThreeLowerCaseAlpha([q, h, i]),
			ThreeLowerCaseAlpha([q, h, j]),
			ThreeLowerCaseAlpha([q, h, k]),
			ThreeLowerCaseAlpha([q, h, l]),
			ThreeLowerCaseAlpha([q, h, m]),
			ThreeLowerCaseAlpha([q, h, n]),
			ThreeLowerCaseAlpha([q, h, o]),
			ThreeLowerCaseAlpha([q, h, p]),
			ThreeLowerCaseAlpha([q, h, q]),
			ThreeLowerCaseAlpha([q, h, r]),
			ThreeLowerCaseAlpha([q, h, s]),
			ThreeLowerCaseAlpha([q, h, t]),
			ThreeLowerCaseAlpha([q, h, u]),
			ThreeLowerCaseAlpha([q, h, v]),
			ThreeLowerCaseAlpha([q, h, w]),
			ThreeLowerCaseAlpha([q, h, x]),
			ThreeLowerCaseAlpha([q, h, y]),
			ThreeLowerCaseAlpha([q, h, z]),
			
			ThreeLowerCaseAlpha([q, i, a]),
			ThreeLowerCaseAlpha([q, i, b]),
			ThreeLowerCaseAlpha([q, i, c]),
			ThreeLowerCaseAlpha([q, i, d]),
			ThreeLowerCaseAlpha([q, i, e]),
			ThreeLowerCaseAlpha([q, i, f]),
			ThreeLowerCaseAlpha([q, i, g]),
			ThreeLowerCaseAlpha([q, i, h]),
			ThreeLowerCaseAlpha([q, i, i]),
			ThreeLowerCaseAlpha([q, i, j]),
			ThreeLowerCaseAlpha([q, i, k]),
			ThreeLowerCaseAlpha([q, i, l]),
			ThreeLowerCaseAlpha([q, i, m]),
			ThreeLowerCaseAlpha([q, i, n]),
			ThreeLowerCaseAlpha([q, i, o]),
			ThreeLowerCaseAlpha([q, i, p]),
			ThreeLowerCaseAlpha([q, i, q]),
			ThreeLowerCaseAlpha([q, i, r]),
			ThreeLowerCaseAlpha([q, i, s]),
			ThreeLowerCaseAlpha([q, i, t]),
			ThreeLowerCaseAlpha([q, i, u]),
			ThreeLowerCaseAlpha([q, i, v]),
			ThreeLowerCaseAlpha([q, i, w]),
			ThreeLowerCaseAlpha([q, i, x]),
			ThreeLowerCaseAlpha([q, i, y]),
			ThreeLowerCaseAlpha([q, i, z]),
			
			ThreeLowerCaseAlpha([q, j, a]),
			ThreeLowerCaseAlpha([q, j, b]),
			ThreeLowerCaseAlpha([q, j, c]),
			ThreeLowerCaseAlpha([q, j, d]),
			ThreeLowerCaseAlpha([q, j, e]),
			ThreeLowerCaseAlpha([q, j, f]),
			ThreeLowerCaseAlpha([q, j, g]),
			ThreeLowerCaseAlpha([q, j, h]),
			ThreeLowerCaseAlpha([q, j, i]),
			ThreeLowerCaseAlpha([q, j, j]),
			ThreeLowerCaseAlpha([q, j, k]),
			ThreeLowerCaseAlpha([q, j, l]),
			ThreeLowerCaseAlpha([q, j, m]),
			ThreeLowerCaseAlpha([q, j, n]),
			ThreeLowerCaseAlpha([q, j, o]),
			ThreeLowerCaseAlpha([q, j, p]),
			ThreeLowerCaseAlpha([q, j, q]),
			ThreeLowerCaseAlpha([q, j, r]),
			ThreeLowerCaseAlpha([q, j, s]),
			ThreeLowerCaseAlpha([q, j, t]),
			ThreeLowerCaseAlpha([q, j, u]),
			ThreeLowerCaseAlpha([q, j, v]),
			ThreeLowerCaseAlpha([q, j, w]),
			ThreeLowerCaseAlpha([q, j, x]),
			ThreeLowerCaseAlpha([q, j, y]),
			ThreeLowerCaseAlpha([q, j, z]),
			
			ThreeLowerCaseAlpha([q, k, a]),
			ThreeLowerCaseAlpha([q, k, b]),
			ThreeLowerCaseAlpha([q, k, c]),
			ThreeLowerCaseAlpha([q, k, d]),
			ThreeLowerCaseAlpha([q, k, e]),
			ThreeLowerCaseAlpha([q, k, f]),
			ThreeLowerCaseAlpha([q, k, g]),
			ThreeLowerCaseAlpha([q, k, h]),
			ThreeLowerCaseAlpha([q, k, i]),
			ThreeLowerCaseAlpha([q, k, j]),
			ThreeLowerCaseAlpha([q, k, k]),
			ThreeLowerCaseAlpha([q, k, l]),
			ThreeLowerCaseAlpha([q, k, m]),
			ThreeLowerCaseAlpha([q, k, n]),
			ThreeLowerCaseAlpha([q, k, o]),
			ThreeLowerCaseAlpha([q, k, p]),
			ThreeLowerCaseAlpha([q, k, q]),
			ThreeLowerCaseAlpha([q, k, r]),
			ThreeLowerCaseAlpha([q, k, s]),
			ThreeLowerCaseAlpha([q, k, t]),
			ThreeLowerCaseAlpha([q, k, u]),
			ThreeLowerCaseAlpha([q, k, v]),
			ThreeLowerCaseAlpha([q, k, w]),
			ThreeLowerCaseAlpha([q, k, x]),
			ThreeLowerCaseAlpha([q, k, y]),
			ThreeLowerCaseAlpha([q, k, z]),
			
			ThreeLowerCaseAlpha([q, l, a]),
			ThreeLowerCaseAlpha([q, l, b]),
			ThreeLowerCaseAlpha([q, l, c]),
			ThreeLowerCaseAlpha([q, l, d]),
			ThreeLowerCaseAlpha([q, l, e]),
			ThreeLowerCaseAlpha([q, l, f]),
			ThreeLowerCaseAlpha([q, l, g]),
			ThreeLowerCaseAlpha([q, l, h]),
			ThreeLowerCaseAlpha([q, l, i]),
			ThreeLowerCaseAlpha([q, l, j]),
			ThreeLowerCaseAlpha([q, l, k]),
			ThreeLowerCaseAlpha([q, l, l]),
			ThreeLowerCaseAlpha([q, l, m]),
			ThreeLowerCaseAlpha([q, l, n]),
			ThreeLowerCaseAlpha([q, l, o]),
			ThreeLowerCaseAlpha([q, l, p]),
			ThreeLowerCaseAlpha([q, l, q]),
			ThreeLowerCaseAlpha([q, l, r]),
			ThreeLowerCaseAlpha([q, l, s]),
			ThreeLowerCaseAlpha([q, l, t]),
			ThreeLowerCaseAlpha([q, l, u]),
			ThreeLowerCaseAlpha([q, l, v]),
			ThreeLowerCaseAlpha([q, l, w]),
			ThreeLowerCaseAlpha([q, l, x]),
			ThreeLowerCaseAlpha([q, l, y]),
			ThreeLowerCaseAlpha([q, l, z]),
			
			ThreeLowerCaseAlpha([q, m, a]),
			ThreeLowerCaseAlpha([q, m, b]),
			ThreeLowerCaseAlpha([q, m, c]),
			ThreeLowerCaseAlpha([q, m, d]),
			ThreeLowerCaseAlpha([q, m, e]),
			ThreeLowerCaseAlpha([q, m, f]),
			ThreeLowerCaseAlpha([q, m, g]),
			ThreeLowerCaseAlpha([q, m, h]),
			ThreeLowerCaseAlpha([q, m, i]),
			ThreeLowerCaseAlpha([q, m, j]),
			ThreeLowerCaseAlpha([q, m, k]),
			ThreeLowerCaseAlpha([q, m, l]),
			ThreeLowerCaseAlpha([q, m, m]),
			ThreeLowerCaseAlpha([q, m, n]),
			ThreeLowerCaseAlpha([q, m, o]),
			ThreeLowerCaseAlpha([q, m, p]),
			ThreeLowerCaseAlpha([q, m, q]),
			ThreeLowerCaseAlpha([q, m, r]),
			ThreeLowerCaseAlpha([q, m, s]),
			ThreeLowerCaseAlpha([q, m, t]),
			ThreeLowerCaseAlpha([q, m, u]),
			ThreeLowerCaseAlpha([q, m, v]),
			ThreeLowerCaseAlpha([q, m, w]),
			ThreeLowerCaseAlpha([q, m, x]),
			ThreeLowerCaseAlpha([q, m, y]),
			ThreeLowerCaseAlpha([q, m, z]),
			
			ThreeLowerCaseAlpha([q, n, a]),
			ThreeLowerCaseAlpha([q, n, b]),
			ThreeLowerCaseAlpha([q, n, c]),
			ThreeLowerCaseAlpha([q, n, d]),
			ThreeLowerCaseAlpha([q, n, e]),
			ThreeLowerCaseAlpha([q, n, f]),
			ThreeLowerCaseAlpha([q, n, g]),
			ThreeLowerCaseAlpha([q, n, h]),
			ThreeLowerCaseAlpha([q, n, i]),
			ThreeLowerCaseAlpha([q, n, j]),
			ThreeLowerCaseAlpha([q, n, k]),
			ThreeLowerCaseAlpha([q, n, l]),
			ThreeLowerCaseAlpha([q, n, m]),
			ThreeLowerCaseAlpha([q, n, n]),
			ThreeLowerCaseAlpha([q, n, o]),
			ThreeLowerCaseAlpha([q, n, p]),
			ThreeLowerCaseAlpha([q, n, q]),
			ThreeLowerCaseAlpha([q, n, r]),
			ThreeLowerCaseAlpha([q, n, s]),
			ThreeLowerCaseAlpha([q, n, t]),
			ThreeLowerCaseAlpha([q, n, u]),
			ThreeLowerCaseAlpha([q, n, v]),
			ThreeLowerCaseAlpha([q, n, w]),
			ThreeLowerCaseAlpha([q, n, x]),
			ThreeLowerCaseAlpha([q, n, y]),
			ThreeLowerCaseAlpha([q, n, z]),
			
			ThreeLowerCaseAlpha([q, o, a]),
			ThreeLowerCaseAlpha([q, o, b]),
			ThreeLowerCaseAlpha([q, o, c]),
			ThreeLowerCaseAlpha([q, o, d]),
			ThreeLowerCaseAlpha([q, o, e]),
			ThreeLowerCaseAlpha([q, o, f]),
			ThreeLowerCaseAlpha([q, o, g]),
			ThreeLowerCaseAlpha([q, o, h]),
			ThreeLowerCaseAlpha([q, o, i]),
			ThreeLowerCaseAlpha([q, o, j]),
			ThreeLowerCaseAlpha([q, o, k]),
			ThreeLowerCaseAlpha([q, o, l]),
			ThreeLowerCaseAlpha([q, o, m]),
			ThreeLowerCaseAlpha([q, o, n]),
			ThreeLowerCaseAlpha([q, o, o]),
			ThreeLowerCaseAlpha([q, o, p]),
			ThreeLowerCaseAlpha([q, o, q]),
			ThreeLowerCaseAlpha([q, o, r]),
			ThreeLowerCaseAlpha([q, o, s]),
			ThreeLowerCaseAlpha([q, o, t]),
			ThreeLowerCaseAlpha([q, o, u]),
			ThreeLowerCaseAlpha([q, o, v]),
			ThreeLowerCaseAlpha([q, o, w]),
			ThreeLowerCaseAlpha([q, o, x]),
			ThreeLowerCaseAlpha([q, o, y]),
			ThreeLowerCaseAlpha([q, o, z]),
			
			ThreeLowerCaseAlpha([q, p, a]),
			ThreeLowerCaseAlpha([q, p, b]),
			ThreeLowerCaseAlpha([q, p, c]),
			ThreeLowerCaseAlpha([q, p, d]),
			ThreeLowerCaseAlpha([q, p, e]),
			ThreeLowerCaseAlpha([q, p, f]),
			ThreeLowerCaseAlpha([q, p, g]),
			ThreeLowerCaseAlpha([q, p, h]),
			ThreeLowerCaseAlpha([q, p, i]),
			ThreeLowerCaseAlpha([q, p, j]),
			ThreeLowerCaseAlpha([q, p, k]),
			ThreeLowerCaseAlpha([q, p, l]),
			ThreeLowerCaseAlpha([q, p, m]),
			ThreeLowerCaseAlpha([q, p, n]),
			ThreeLowerCaseAlpha([q, p, o]),
			ThreeLowerCaseAlpha([q, p, p]),
			ThreeLowerCaseAlpha([q, p, q]),
			ThreeLowerCaseAlpha([q, p, r]),
			ThreeLowerCaseAlpha([q, p, s]),
			ThreeLowerCaseAlpha([q, p, t]),
			ThreeLowerCaseAlpha([q, p, u]),
			ThreeLowerCaseAlpha([q, p, v]),
			ThreeLowerCaseAlpha([q, p, w]),
			ThreeLowerCaseAlpha([q, p, x]),
			ThreeLowerCaseAlpha([q, p, y]),
			ThreeLowerCaseAlpha([q, p, z]),
			
			ThreeLowerCaseAlpha([q, q, a]),
			ThreeLowerCaseAlpha([q, q, b]),
			ThreeLowerCaseAlpha([q, q, c]),
			ThreeLowerCaseAlpha([q, q, d]),
			ThreeLowerCaseAlpha([q, q, e]),
			ThreeLowerCaseAlpha([q, q, f]),
			ThreeLowerCaseAlpha([q, q, g]),
			ThreeLowerCaseAlpha([q, q, h]),
			ThreeLowerCaseAlpha([q, q, i]),
			ThreeLowerCaseAlpha([q, q, j]),
			ThreeLowerCaseAlpha([q, q, k]),
			ThreeLowerCaseAlpha([q, q, l]),
			ThreeLowerCaseAlpha([q, q, m]),
			ThreeLowerCaseAlpha([q, q, n]),
			ThreeLowerCaseAlpha([q, q, o]),
			ThreeLowerCaseAlpha([q, q, p]),
			ThreeLowerCaseAlpha([q, q, q]),
			ThreeLowerCaseAlpha([q, q, r]),
			ThreeLowerCaseAlpha([q, q, s]),
			ThreeLowerCaseAlpha([q, q, t]),
			ThreeLowerCaseAlpha([q, q, u]),
			ThreeLowerCaseAlpha([q, q, v]),
			ThreeLowerCaseAlpha([q, q, w]),
			ThreeLowerCaseAlpha([q, q, x]),
			ThreeLowerCaseAlpha([q, q, y]),
			ThreeLowerCaseAlpha([q, q, z]),
			
			ThreeLowerCaseAlpha([q, r, a]),
			ThreeLowerCaseAlpha([q, r, b]),
			ThreeLowerCaseAlpha([q, r, c]),
			ThreeLowerCaseAlpha([q, r, d]),
			ThreeLowerCaseAlpha([q, r, e]),
			ThreeLowerCaseAlpha([q, r, f]),
			ThreeLowerCaseAlpha([q, r, g]),
			ThreeLowerCaseAlpha([q, r, h]),
			ThreeLowerCaseAlpha([q, r, i]),
			ThreeLowerCaseAlpha([q, r, j]),
			ThreeLowerCaseAlpha([q, r, k]),
			ThreeLowerCaseAlpha([q, r, l]),
			ThreeLowerCaseAlpha([q, r, m]),
			ThreeLowerCaseAlpha([q, r, n]),
			ThreeLowerCaseAlpha([q, r, o]),
			ThreeLowerCaseAlpha([q, r, p]),
			ThreeLowerCaseAlpha([q, r, q]),
			ThreeLowerCaseAlpha([q, r, r]),
			ThreeLowerCaseAlpha([q, r, s]),
			ThreeLowerCaseAlpha([q, r, t]),
			ThreeLowerCaseAlpha([q, r, u]),
			ThreeLowerCaseAlpha([q, r, v]),
			ThreeLowerCaseAlpha([q, r, w]),
			ThreeLowerCaseAlpha([q, r, x]),
			ThreeLowerCaseAlpha([q, r, y]),
			ThreeLowerCaseAlpha([q, r, z]),
			
			ThreeLowerCaseAlpha([q, s, a]),
			ThreeLowerCaseAlpha([q, s, b]),
			ThreeLowerCaseAlpha([q, s, c]),
			ThreeLowerCaseAlpha([q, s, d]),
			ThreeLowerCaseAlpha([q, s, e]),
			ThreeLowerCaseAlpha([q, s, f]),
			ThreeLowerCaseAlpha([q, s, g]),
			ThreeLowerCaseAlpha([q, s, h]),
			ThreeLowerCaseAlpha([q, s, i]),
			ThreeLowerCaseAlpha([q, s, j]),
			ThreeLowerCaseAlpha([q, s, k]),
			ThreeLowerCaseAlpha([q, s, l]),
			ThreeLowerCaseAlpha([q, s, m]),
			ThreeLowerCaseAlpha([q, s, n]),
			ThreeLowerCaseAlpha([q, s, o]),
			ThreeLowerCaseAlpha([q, s, p]),
			ThreeLowerCaseAlpha([q, s, q]),
			ThreeLowerCaseAlpha([q, s, r]),
			ThreeLowerCaseAlpha([q, s, s]),
			ThreeLowerCaseAlpha([q, s, t]),
			ThreeLowerCaseAlpha([q, s, u]),
			ThreeLowerCaseAlpha([q, s, v]),
			ThreeLowerCaseAlpha([q, s, w]),
			ThreeLowerCaseAlpha([q, s, x]),
			ThreeLowerCaseAlpha([q, s, y]),
			ThreeLowerCaseAlpha([q, s, z]),
			
			ThreeLowerCaseAlpha([q, t, a]),
			ThreeLowerCaseAlpha([q, t, b]),
			ThreeLowerCaseAlpha([q, t, c]),
			ThreeLowerCaseAlpha([q, t, d]),
			ThreeLowerCaseAlpha([q, t, e]),
			ThreeLowerCaseAlpha([q, t, f]),
			ThreeLowerCaseAlpha([q, t, g]),
			ThreeLowerCaseAlpha([q, t, h]),
			ThreeLowerCaseAlpha([q, t, i]),
			ThreeLowerCaseAlpha([q, t, j]),
			ThreeLowerCaseAlpha([q, t, k]),
			ThreeLowerCaseAlpha([q, t, l]),
			ThreeLowerCaseAlpha([q, t, m]),
			ThreeLowerCaseAlpha([q, t, n]),
			ThreeLowerCaseAlpha([q, t, o]),
			ThreeLowerCaseAlpha([q, t, p]),
			ThreeLowerCaseAlpha([q, t, q]),
			ThreeLowerCaseAlpha([q, t, r]),
			ThreeLowerCaseAlpha([q, t, s]),
			ThreeLowerCaseAlpha([q, t, t]),
			ThreeLowerCaseAlpha([q, t, u]),
			ThreeLowerCaseAlpha([q, t, v]),
			ThreeLowerCaseAlpha([q, t, w]),
			ThreeLowerCaseAlpha([q, t, x]),
			ThreeLowerCaseAlpha([q, t, y]),
			ThreeLowerCaseAlpha([q, t, z]),
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
			2 => Ok(TwoLowerCaseAlpha(Self::subtag_to_byte_array::<_, 2>(&subtag, Self::validate_is_lower_case_alpha)?)),
			
			3 => Ok(ThreeLowerCaseAlpha(Self::subtag_to_byte_array::<_, 3>(&subtag, Self::validate_is_lower_case_alpha)?)),
			
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
				
				suppress_script: match suppress_script
				{
					None => None,
					
					Some(suppress_script) => Some(ScriptRecord::parse_key(suppress_script)?),
				},
				
				macrolanguage,
				
				scope: scope.unwrap_or_default(),
			}
		)
	}
}
