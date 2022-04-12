// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// [RFC 3987, Section 2.2](https://datatracker.ietf.org/doc/html/rfc3987#section-2.2).
///
/// `iprivate = %xE000-F8FF / %xF0000-FFFFD / %x100000-10FFFD`.
/// Instead of one macro, we separate by UTF-8 encoding count.
macro_rules! iprivate_3
{
	() =>
	{
		xE000 ..= xF8FF
	}
}
