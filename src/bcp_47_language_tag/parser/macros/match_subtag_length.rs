// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


macro_rules! match_subtag_length
{
	($subtag: ident, $parse_1: expr, $parse_2: expr, $parse_3: expr, $parse_4: expr, $parse_5: expr, $parse_6: expr, $parse_7: expr, $parse_8: expr) =>
	{
		match $subtag.len()
		{
			0 => return_error_is_zero!(),
			
			1 => $parse_1,
			
			2 => $parse_2,
			
			3 => $parse_3,
			
			4 => $parse_4,
			
			5 => $parse_5,
			
			6 => $parse_6,
			
			7 => $parse_7,
			
			8 => $parse_8,
			
			length @ _ => return_error_is_greater_than_eight!(length),
		}
	};
	
	($subtag: ident, $validated_extension_code: ident, $parse_from_private_use: expr, $parse_from_extension: stmt, $parse_2: expr, $parse_3: expr, $parse_4: expr, $parse_5: expr, $parse_6: expr, $parse_7: expr, $parse_8: expr) =>
	{
		match_subtag_length!
		{
			$subtag,
			
			match $subtag.byte_0()
			{
				X | x => return $parse_from_private_use,
				
				$validated_extension_code @ (_0 ..= _9 | A ..= W | Y ..= Z | a ..= w | y ..= z) => { $parse_from_extension },
				
				invalid_extension_code @ _ => return_error!(InvalidExtensionSingleton(invalid_extension_code))
			
			},
			
			$parse_2,
			
			$parse_3,
			
			$parse_4,
			
			$parse_5,
			
			$parse_6,
			
			$parse_7,
			
			$parse_8
		}
	}
}
