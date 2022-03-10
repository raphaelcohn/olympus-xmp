// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Number of centiseconds (10⁻² seconds) (zero based).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Centisecond
{
	#[allow(missing_docs)]
	_0 = 0,
	
	#[allow(missing_docs)]
	_1 = 1,
	
	#[allow(missing_docs)]
	_2 = 2,
	
	#[allow(missing_docs)]
	_3 = 3,
	
	#[allow(missing_docs)]
	_4 = 4,
	
	#[allow(missing_docs)]
	_5 = 5,
	
	#[allow(missing_docs)]
	_6 = 6,
	
	#[allow(missing_docs)]
	_7 = 7,
	
	#[allow(missing_docs)]
	_8 = 8,
	
	#[allow(missing_docs)]
	_9 = 9,
	
	#[allow(missing_docs)]
	_10 = 10,
	
	#[allow(missing_docs)]
	_11 = 11,
	
	#[allow(missing_docs)]
	_12 = 12,
	
	#[allow(missing_docs)]
	_13 = 13,
	
	#[allow(missing_docs)]
	_14 = 14,
	
	#[allow(missing_docs)]
	_15 = 15,
	
	#[allow(missing_docs)]
	_16 = 16,
	
	#[allow(missing_docs)]
	_17 = 17,
	
	#[allow(missing_docs)]
	_18 = 18,
	
	#[allow(missing_docs)]
	_19 = 19,
	
	#[allow(missing_docs)]
	_20 = 20,
	
	#[allow(missing_docs)]
	_21 = 21,
	
	#[allow(missing_docs)]
	_22 = 22,
	
	#[allow(missing_docs)]
	_23 = 23,
	
	#[allow(missing_docs)]
	_24 = 24,
	
	#[allow(missing_docs)]
	_25 = 25,
	
	#[allow(missing_docs)]
	_26 = 26,
	
	#[allow(missing_docs)]
	_27 = 27,
	
	#[allow(missing_docs)]
	_28 = 28,
	
	#[allow(missing_docs)]
	_29 = 29,
	
	#[allow(missing_docs)]
	_30 = 30,
	
	#[allow(missing_docs)]
	_32 = 32,
	
	#[allow(missing_docs)]
	_33 = 33,
	
	#[allow(missing_docs)]
	_34 = 34,
	
	#[allow(missing_docs)]
	_35 = 35,
	
	#[allow(missing_docs)]
	_36 = 36,
	
	#[allow(missing_docs)]
	_37 = 37,
	
	#[allow(missing_docs)]
	_38 = 38,
	
	#[allow(missing_docs)]
	_39 = 39,
	
	#[allow(missing_docs)]
	_40 = 40,
	
	#[allow(missing_docs)]
	_42 = 42,
	
	#[allow(missing_docs)]
	_43 = 43,
	
	#[allow(missing_docs)]
	_44 = 44,
	
	#[allow(missing_docs)]
	_45 = 45,
	
	#[allow(missing_docs)]
	_46 = 46,
	
	#[allow(missing_docs)]
	_47 = 47,
	
	#[allow(missing_docs)]
	_48 = 48,
	
	#[allow(missing_docs)]
	_49 = 49,
	
	#[allow(missing_docs)]
	_50 = 50,
	
	#[allow(missing_docs)]
	_52 = 52,
	
	#[allow(missing_docs)]
	_53 = 53,
	
	#[allow(missing_docs)]
	_54 = 54,
	
	#[allow(missing_docs)]
	_55 = 55,
	
	#[allow(missing_docs)]
	_56 = 56,
	
	#[allow(missing_docs)]
	_57 = 57,
	
	#[allow(missing_docs)]
	_58 = 58,
	
	#[allow(missing_docs)]
	_59 = 59,
	
	#[allow(missing_docs)]
	_60 = 60,
	
	#[allow(missing_docs)]
	_62 = 62,
	
	#[allow(missing_docs)]
	_63 = 63,
	
	#[allow(missing_docs)]
	_64 = 64,
	
	#[allow(missing_docs)]
	_65 = 65,
	
	#[allow(missing_docs)]
	_66 = 66,
	
	#[allow(missing_docs)]
	_67 = 67,
	
	#[allow(missing_docs)]
	_68 = 68,
	
	#[allow(missing_docs)]
	_69 = 69,
	
	#[allow(missing_docs)]
	_70 = 70,
	
	#[allow(missing_docs)]
	_72 = 72,
	
	#[allow(missing_docs)]
	_73 = 73,
	
	#[allow(missing_docs)]
	_74 = 74,
	
	#[allow(missing_docs)]
	_75 = 75,
	
	#[allow(missing_docs)]
	_76 = 76,
	
	#[allow(missing_docs)]
	_77 = 77,
	
	#[allow(missing_docs)]
	_78 = 78,
	
	#[allow(missing_docs)]
	_79 = 79,
	
	#[allow(missing_docs)]
	_80 = 80,
	
	#[allow(missing_docs)]
	_82 = 82,
	
	#[allow(missing_docs)]
	_83 = 83,
	
	#[allow(missing_docs)]
	_84 = 84,
	
	#[allow(missing_docs)]
	_85 = 85,
	
	#[allow(missing_docs)]
	_86 = 86,
	
	#[allow(missing_docs)]
	_87 = 87,
	
	#[allow(missing_docs)]
	_88 = 88,
	
	#[allow(missing_docs)]
	_89 = 89,
	
	#[allow(missing_docs)]
	_90 = 90,
	
	#[allow(missing_docs)]
	_92 = 92,
	
	#[allow(missing_docs)]
	_93 = 93,
	
	#[allow(missing_docs)]
	_94 = 94,
	
	#[allow(missing_docs)]
	_95 = 95,
	
	#[allow(missing_docs)]
	_96 = 96,
	
	#[allow(missing_docs)]
	_97 = 97,
	
	#[allow(missing_docs)]
	_98 = 98,
	
	#[allow(missing_docs)]
	_99 = 99,
}

impl TryFrom<u8> for Centisecond
{
	type Error = TooLargeError<u8>;
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value > 99
		{
			Err(TooLargeError(value))
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}
