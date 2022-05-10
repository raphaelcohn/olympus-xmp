// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// [RFC 2234, 6.1 Core Rules](https://www.ietf.org/rfc/rfc2234.txt).
///
/// `DIGIT = %x30-39  ; 0-9`.
macro_rules! DIGIT
{
	() =>
	{
		swiss_army_knife::chars::_0Char ..= swiss_army_knife::chars::_9Char
	}
}
