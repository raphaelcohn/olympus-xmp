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
	fn parse_irregular_i(mut subtags: MemchrIterator<Hyphen>) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		use GrandfatheredIrregularISubtagParseError::*;
		let second_subtag = next_or_error!(subtags, MissingSecondSubtag);
		finished!(subtags, MoreThanTwoSubtags);
		
		match second_subtag.len()
		{
			length @ 0 ..= 2 => Err(LengthIsLessThanThree { length }),
			
			3 => Self::parse_irregular_i_3(second_subtag),
			
			4 => Self::unregistered::<4>(second_subtag),
			
			5 => Self::parse_irregular_i_5(second_subtag),
			
			6 => Self::parse_irregular_i_6(second_subtag),
			
			7 => Self::parse_irregular_i_7(second_subtag),
			
			8 => Self::parse_irregular_i_8(second_subtag),
			
			length @ _ => Err(LengthIsGreaterThanEight { length })
		}
	}
	
	#[inline(always)]
	fn unregistered<const _length: usize>(second_subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		Err(GrandfatheredIrregularISubtagParseError::Unregistered(array_vec_u8::<8>(second_subtag)))
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
	fn parse_irregular_i_3(second_subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const length: usize = 3;
		debug_assert_eq!(second_subtag.len(), length);
		
		const i_ami: IrregularGrandfathered = IrregularGrandfathered::i_ami;
		const i_bnn: IrregularGrandfathered = IrregularGrandfathered::i_bnn;
		const i_hak: IrregularGrandfathered = IrregularGrandfathered::i_hak;
		const i_lux: IrregularGrandfathered = IrregularGrandfathered::i_lux;
		const i_pwn: IrregularGrandfathered = IrregularGrandfathered::i_pwn;
		const i_tao: IrregularGrandfathered = IrregularGrandfathered::i_tao;
		const i_tay: IrregularGrandfathered = IrregularGrandfathered::i_tay;
		const i_tsu: IrregularGrandfathered = IrregularGrandfathered::i_tsu;
		match Alpha::lower_case_alpha::<0>(second_subtag)
		{
			a => Self::parse_irregular_i_n_slice::<length, b"ami", i_ami, 1>(second_subtag),
			
			b => Self::parse_irregular_i_n_slice::<length, b"bnn", i_bnn, 1>(second_subtag),
			
			h => Self::parse_irregular_i_n_slice::<length, b"hak", i_hak, 1>(second_subtag),
			
			i => Self::parse_irregular_i_n_slice::<length, b"lux", i_lux, 1>(second_subtag),
			
			p => Self::parse_irregular_i_n_slice::<length, b"pwn", i_pwn, 1>(second_subtag),
			
			t => match Alpha::lower_case_alpha::<1>(second_subtag)
			{
				a => match Alpha::lower_case_alpha::<2>(second_subtag)
				{
					o => Ok(i_tao),
					
					y => Ok(i_tay),
					
					_ => Self::unregistered::<length>(second_subtag),
				},
				
				s => match Alpha::lower_case_alpha::<2>(second_subtag)
				{
					u => Ok(i_tsu),
					
					_ => Self::unregistered::<length>(second_subtag),
				}
				
				_ => Self::unregistered::<length>(second_subtag),
			},
			
			_ => Self::unregistered::<length>(second_subtag)
		}
	}
	
	/// * `i-mingo`.
	#[inline(always)]
	fn parse_irregular_i_5(second_subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const i_mingo: IrregularGrandfathered = IrregularGrandfathered::i_mingo;
		Self::parse_irregular_i_n::<5, b"mingo", i_mingo>(second_subtag)
	}
	
	/// * `i-navajo`.
	#[inline(always)]
	fn parse_irregular_i_6(second_subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const i_navajo: IrregularGrandfathered = IrregularGrandfathered::i_navajo;
		Self::parse_irregular_i_n::<6, b"navajo", i_navajo>(second_subtag)
	}
	
	/// * `i-default`.
	/// * `i-klingon`.
	#[inline(always)]
	fn parse_irregular_i_7(second_subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const length: usize = 7;
		debug_assert_eq!(second_subtag.len(), length);
		
		const i_default: IrregularGrandfathered = IrregularGrandfathered::i_default;
		const i_klingon: IrregularGrandfathered = IrregularGrandfathered::i_klingon;
		let first_byte = Alpha::lower_case_alpha::<0>(second_subtag);
		match first_byte
		{
			d => Self::parse_irregular_i_n_slice::<length, b"default", i_default, 1>(second_subtag),
			
			k => Self::parse_irregular_i_n_slice::<length, b"klingon", i_klingon, 1>(second_subtag),
			
			_ => Self::unregistered::<length>(second_subtag)
		}
	}
	
	/// * `i-enochian`.
	#[inline(always)]
	fn parse_irregular_i_8(second_subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		const i_enochian: IrregularGrandfathered = IrregularGrandfathered::i_enochian;
		Self::parse_irregular_i_n::<8, b"enochian", i_enochian>(second_subtag)
	}
	
	#[inline(always)]
	fn parse_irregular_i_n<const length: usize, const pattern: &'static [u8], const ok: IrregularGrandfathered>(second_subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		Self::parse_irregular_i_n_slice::<length, pattern, ok, 0>(second_subtag)
	}
	
	#[unroll_for_loops]
	#[inline(always)]
	fn parse_irregular_i_n_slice<const length: usize, const pattern: &'static [u8], const ok: IrregularGrandfathered, const second_subtag_from_index: usize>(second_subtag: &[u8]) -> Result<Self, GrandfatheredIrregularISubtagParseError>
	{
		debug_assert_eq!(second_subtag.len(), length);
		debug_assert_eq!(pattern.len(), length);
		
		for index in second_subtag_from_index .. length
		{
			let actual_byte = Alpha::lower_case_alpha_indexed(second_subtag, index);
			let expected_byte = pattern.get_unchecked_value_safe(index);
			
			if actual_byte != expected_byte
			{
				return Self::unregistered::<length>(second_subtag)
			}
		}
		
		Ok(ok)
	}
}
