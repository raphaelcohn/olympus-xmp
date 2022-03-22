// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// BCP 47, 2.1 Syntax: "irregular tags do not match the 'langtag' production and would not otherwise be considered 'well-formed'.
/// These tags are all valid, but most are deprecated in favor of more modern subtags or subtag combination".
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum IrregularGrandfathered
{
	#[allow(missing_docs)]
	en_GB_oed,
	
	#[allow(missing_docs)]
	i_ami,
	
	#[allow(missing_docs)]
	i_bnn,
	
	#[allow(missing_docs)]
	i_default,
	
	#[allow(missing_docs)]
	i_enochian,
	
	#[allow(missing_docs)]
	i_hak,
	
	#[allow(missing_docs)]
	i_klingon,
	
	#[allow(missing_docs)]
	i_lux,
	
	#[allow(missing_docs)]
	i_mingo,
	
	#[allow(missing_docs)]
	i_navajo,
	
	#[allow(missing_docs)]
	i_pwn,
	
	#[allow(missing_docs)]
	i_tao,
	
	#[allow(missing_docs)]
	i_tay,
	
	#[allow(missing_docs)]
	i_tsu,
	
	#[allow(missing_docs)]
	sgn_BE_FR,
	
	#[allow(missing_docs)]
	sgn_BE_NL,
	
	#[allow(missing_docs)]
	sgn_CH_DE,
}

impl IrregularGrandfathered
{
	#[inline(always)]
	fn parse_irregular_i(mut subtags: Subtags) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		use GrandfatheredIrregularISubtagParseError::*;
		let subtag = next_or_error!(subtags, MissingSecondSubtag);
		finished!(subtags, MoreThanTwoSubtags);
		
		match subtag.len()
		{
			0 => return_error_is_zero!(),
			
			1 => return_error!(IsOne),
			
			2 => return_error!(IsTwo),
			
			3 => Self::parse_irregular_i_3(subtag),
			
			4 => Self::unregistered::<4>(subtag),
			
			5 => Self::parse_irregular_i_5(subtag),
			
			6 => Self::parse_irregular_i_6(subtag),
			
			7 => Self::parse_irregular_i_7(subtag),
			
			8 => Self::parse_irregular_i_8(subtag),
			
			length @ _ => return_error_is_greater_than_eight!(length),
		}
	}
	
	#[inline(always)]
	fn unregistered<const length: usize>(subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		debug_assert_eq!(length, subtag.len());
		
		Err(GrandfatheredIrregularISubtagParseError::Unregistered(array_vec_u8::<8>(subtag)))
	}
	
	/// * `i-ami`.
	/// * `i-bnn`.`
	/// * `i-hak`.`
	/// * `i-lux`.`
	/// * `i-pwn`.`
	/// * `i-tao`.`
	/// * `i-tay`.`
	/// * `i-tsu`.`
	#[inline(always)]
	fn parse_irregular_i_3(subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const length: usize = 3;
		debug_assert_eq!(subtag.len(), length);
		
		const i_ami: IrregularGrandfathered = IrregularGrandfathered::i_ami;
		const i_bnn: IrregularGrandfathered = IrregularGrandfathered::i_bnn;
		const i_hak: IrregularGrandfathered = IrregularGrandfathered::i_hak;
		const i_lux: IrregularGrandfathered = IrregularGrandfathered::i_lux;
		const i_pwn: IrregularGrandfathered = IrregularGrandfathered::i_pwn;
		const i_tao: IrregularGrandfathered = IrregularGrandfathered::i_tao;
		const i_tay: IrregularGrandfathered = IrregularGrandfathered::i_tay;
		const i_tsu: IrregularGrandfathered = IrregularGrandfathered::i_tsu;
		match Alpha::lower_case_alpha::<0>(subtag)
		{
			a => Self::parse_irregular_i_n_slice::<length, b"ami", i_ami, 1>(subtag),
			
			b => Self::parse_irregular_i_n_slice::<length, b"bnn", i_bnn, 1>(subtag),
			
			h => Self::parse_irregular_i_n_slice::<length, b"hak", i_hak, 1>(subtag),
			
			i => Self::parse_irregular_i_n_slice::<length, b"lux", i_lux, 1>(subtag),
			
			p => Self::parse_irregular_i_n_slice::<length, b"pwn", i_pwn, 1>(subtag),
			
			t => match Alpha::lower_case_alpha::<1>(subtag)
			{
				a => match Alpha::lower_case_alpha::<2>(subtag)
				{
					o => Ok(i_tao),
					
					y => Ok(i_tay),
					
					_ => Self::unregistered::<length>(subtag),
				},
				
				s => match Alpha::lower_case_alpha::<2>(subtag)
				{
					u => Ok(i_tsu),
					
					_ => Self::unregistered::<length>(subtag),
				}
				
				_ => Self::unregistered::<length>(subtag),
			},
			
			_ => Self::unregistered::<length>(subtag)
		}
	}
	
	/// * `i-mingo`.
	#[inline(always)]
	fn parse_irregular_i_5(subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const i_mingo: IrregularGrandfathered = IrregularGrandfathered::i_mingo;
		Self::parse_irregular_i_n::<5, b"mingo", i_mingo>(subtag)
	}
	
	/// * `i-navajo`.
	#[inline(always)]
	fn parse_irregular_i_6(subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const i_navajo: IrregularGrandfathered = IrregularGrandfathered::i_navajo;
		Self::parse_irregular_i_n::<6, b"navajo", i_navajo>(subtag)
	}
	
	/// * `i-default`.
	/// * `i-klingon`.
	#[inline(always)]
	fn parse_irregular_i_7(subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const length: usize = 7;
		debug_assert_eq!(subtag.len(), length);
		
		const i_default: IrregularGrandfathered = IrregularGrandfathered::i_default;
		const i_klingon: IrregularGrandfathered = IrregularGrandfathered::i_klingon;
		let first_byte = Alpha::lower_case_alpha::<0>(subtag);
		match first_byte
		{
			d => Self::parse_irregular_i_n_slice::<length, b"default", i_default, 1>(subtag),
			
			k => Self::parse_irregular_i_n_slice::<length, b"klingon", i_klingon, 1>(subtag),
			
			_ => Self::unregistered::<length>(subtag)
		}
	}
	
	/// * `i-enochian`.
	#[inline(always)]
	fn parse_irregular_i_8(subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const i_enochian: IrregularGrandfathered = IrregularGrandfathered::i_enochian;
		Self::parse_irregular_i_n::<8, b"enochian", i_enochian>(subtag)
	}
	
	#[inline(always)]
	fn parse_irregular_i_n<const length: usize, const pattern: &'static [u8], const ok: IrregularGrandfathered>(subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		Self::parse_irregular_i_n_slice::<length, pattern, ok, 0>(subtag)
	}
	
	#[unroll_for_loops]
	#[inline(always)]
	fn parse_irregular_i_n_slice<const length: usize, const pattern: &'static [u8], const ok: IrregularGrandfathered, const subtag_from_index: usize>(subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		debug_assert_eq!(subtag.len(), length);
		debug_assert_eq!(pattern.len(), length);
		
		for index in subtag_from_index .. length
		{
			let actual_byte = Alpha::lower_case_alpha_indexed(subtag, index);
			let expected_byte = pattern.get_unchecked_value_safe(index);
			
			if actual_byte != expected_byte
			{
				return Self::unregistered::<length>(subtag)
			}
		}
		
		Ok(ok)
	}
}
