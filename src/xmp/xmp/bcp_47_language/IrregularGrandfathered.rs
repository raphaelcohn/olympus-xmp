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
