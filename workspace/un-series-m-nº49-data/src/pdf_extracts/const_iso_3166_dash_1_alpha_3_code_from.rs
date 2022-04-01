// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Exists because it is impossible to provide a const closure to `Option.map()`.
#[inline(always)]
const fn const_iso_3166_dash_1_alpha_3_code_from(iso_3166_alpha_3_code: StaticIso3166Dash1Alpha3Code) -> Iso3166Dash1Alpha3Code
{
	Iso3166Dash1Alpha3Code::from(iso_3166_alpha_3_code)
}
