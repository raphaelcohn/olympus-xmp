// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Has different names to those used in UN Series M, Nº49.
///
/// Has some oddities:-
///
/// * Contains sub-country definitions:-
/// 	* Tanzania
///		 	* `835`: `Fmr Tanganyika`.
/// 		* `836`: `Fmr Zanzibar and Pemba Isd`.
///		* Malaysia
///			* `457`: `Sarawak`.
///			* `459`: `Peninsula Malaysia`.
///			* `461`: `Sabah`.
/// * Contains a mapping for India and Sikkim that is incompatible:-
///		* Defines `356` for `India, excl. Sikkim` which is officially `India`.
///		* Defines `699` for `India` which is officially `Seychelles`.
/// * Lists the USA twice, but using its customs area.
/// * Includes an obsolete country which dissolved 7 years before N Series M, Nº49 was first published in 1970.
/// 	* `717`: `Fmr Rhodesia Nyas`, which is probably the Federation of Rhodesia and Nyasaland, replaced by Zambia, Malawi and Souther Rhodesia / Rhodesia / Zimbabwe.
pub(super) fn reporter_areas() -> HashMap<M49Code, String>
{
	#[inline(always)]
	fn first(id: &[u8]) -> u8
	{
		let first = id.get_unchecked_value_safe(0);
		match first
		{
			1 ..= _9 => first,
			
			_ => panic!("Initial value is not 1 to 9 but {}", first)
		}
	}
	
	#[inline(always)]
	fn subsequent<const index: u8>(id: &[u8]) -> u8
	{
		let subsequent = id.get_unchecked_value_safe(index);
		match subsequent
		{
			_0 ..= _9 => subsequent,
			
			_ => panic!("Subsequent value is not 0 to 9 but {} at index {}", subsequent, index)
		}
	}
	
	const reporterAreas: &'static str = include_str!("reporterAreas.json");
	let root = parse(reporterAreas).expect("Could not parser reporterAreas.json").as_object_or_panic("root");
	
	let results = root.take_array_or_panic("results");
	let mut mapping = HashMap::with_capacity(results.len() - 1);
	for element in results
	{
		let element = element.as_object_or_panic("element");
		
		let id = element.take_string_or_panic("id");
		let id = id.as_bytes();
		let text = element.take_string_or_panic("text");
		
		if id == b"all"
		{
			assert_eq!(text, "All");
			continue
		}
		
		let code = M49Code::from
		(
			match id.len()
			{
				0 => panic!("length was 0"),
				
				1 => [_0, _0, first(id)],
				
				2 => [_0, first(id), subsequent::<1>(id)],
				
				3 => [first(id), subsequent::<1>(id), subsequent::<2>(id)],
				
				length @ _ => panic!("length was {}, longer than 3", length)
			}
		);
		
		let was = mapping.insert(code, text);
		assert!(was.is_none(), "Duplicate code {}", code);
	}
	mapping
}
