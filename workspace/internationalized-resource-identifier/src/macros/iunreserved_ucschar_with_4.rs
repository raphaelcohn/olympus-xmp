// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// [RFC 3987, Section 2.2](https://datatracker.ietf.org/doc/html/rfc3987#section-2.2).
///
/// `ucschar = %xA0-D7FF / %xF900-FDCF / %xFDF0-FFEF / %x10000-1FFFD / %x20000-2FFFD / %x30000-3FFFD / %x40000-4FFFD / %x50000-5FFFD / %x60000-6FFFD / %x70000-7FFFD / %x80000-8FFFD / %x90000-9FFFD / %xA0000-AFFFD / %xB0000-BFFFD / %xC0000-CFFFD / %xD0000-DFFFD / %xE1000-EFFFD`.
/// Instead of one macro, we separate by UTF-8 encoding count.
macro_rules! iunreserved_with_ucschar_4
{
	() =>
	{
		chars::x10000 ..= chars::x1FFFD | chars::x20000 ..= chars::x2FFFD | chars::x30000 ..= chars::x3FFFD | chars::x40000 ..= chars::x4FFFD | chars::x50000 ..= chars::x5FFFD | chars::x60000 ..= chars::x6FFFD | chars::x70000 ..= chars::x7FFFD | chars::x80000 ..= chars::x8FFFD | chars::x90000 ..= chars::x9FFFD | chars::xA0000 ..= chars::xAFFFD | chars::xB0000 ..= chars::xBFFFD | chars::xC0000 ..= chars::xCFFFD | chars::xD0000 ..= chars::xDFFFD | chars::xE1000 ..= chars::xEFFFD
	}
}
