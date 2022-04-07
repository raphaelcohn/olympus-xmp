// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// [RFC 2234, 6.1 Core Rules](https://www.ietf.org/rfc/rfc2234.txt).
///
/// `ALPHA = %x41-5A / %x61-7A   ; A-Z / a-z`.
macro_rules! ALPHA
{
	() =>
	{
		AChar ..= ZChar | aChar ..= zChar
	}
}
