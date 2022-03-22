// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Calculates the population count of 64 `u8`s pointed to be source pointer; loads directly from memory.
///
/// Similar to the intrinsic `_mm512_maskz_popcnt_epi8()` but loads directly from memory.
#[cfg(all(target_feature = "avx512bitalg", target_feature = "avx512bw"))]
#[inline(always)]
pub unsafe fn _mm512_maskz_popcnt_epi8_load_unaligned(source_pointer: *const __m512i, k: __mmask64) -> __m512i
{
	let mut population_counts: __m512i;
	asm!
	(
		"vpopcntb {zmm_out} {{{k}}}, zmmword ptr [{memory}]",
	
		zmm_out = lateout(zmm_reg) population_counts,
		memory = in(reg) source_pointer,
		k = in(kreg) k,
	
		options
		(
			pure,readonly,
			preserves_flags,
			nostack,
		),
	);
	population_counts
}
