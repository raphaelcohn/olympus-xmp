// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


macro_rules! parse_digit_alphanumeric_variant_4
{
	($subtag: ident) =>
	{
		{
			const length: usize = 4;
			const length_alphanumeric: usize = length - 1;
			let digit = Digit::construct(Digit::validate_and_convert_byte::<_, _, length>($subtag, InvalidDigit, 0)?);
			Alphanumeric::validate_and_convert_array::<_, _, _, _, length_alphanumeric>($subtag.get_unchecked_range_safe(1 .. ), |alphanumeric_array| DigitAlphanumeric3(digit, alphanumeric_array), InvalidAlphanumeric)?
		}
	}
}
