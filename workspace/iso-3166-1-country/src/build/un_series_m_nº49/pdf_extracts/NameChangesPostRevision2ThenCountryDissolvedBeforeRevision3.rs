// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
const NameChangesPostRevision2ThenCountryDissolvedBeforeRevision3: [(M49Code, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str); 1] =
{
	#[inline(always)]
	const fn name_change_post_revision_2_then_country_dissolved_before_revision_3(m49_code: &'static [u8; 3], arabic_name: &'static str, chinese_name: &'static str, english_name: &'static str, french_name: &'static str, russian_name: &'static str, spanish_name: &'static str) -> (M49Code, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str)
	{
		(M49Code::from(m49_code), arabic_name, chinese_name, english_name, french_name, russian_name, spanish_name)
	}
	
	[
		name_change_post_revision_2_then_country_dissolved_before_revision_3(b"890", "جمهورية يوغوسلافيا الاتحادية الاشتراكية", "事斯拉夫社会主义联邦共和嚣", "Socialist Federal Republic of Yugoslavia", "République fédérative socialiste de Yougoslavie", "Социалистическая Федеративная Республика Югославия", "República Socialista Federativa de Yugoslavia"),
	]
};
