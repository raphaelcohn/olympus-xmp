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
fn reporter_areas() -> BTreeMap<M49CodeArray, String>
{
	let mut root = parse(reporterAreas_json_without_byte_order_mark()).expect("Could not parse reporterAreas.json").as_object_or_panic("root");
	
	let results = root.take_array_or_panic("results");
	let mut mapping = BTreeMap::new();
	for element in results
	{
		let mut element = element.as_object_or_panic("element");
		
		let id = element.take_string_or_panic("id");
		let id = id.as_bytes();
		let text = element.take_string_or_panic("text");
		
		if id == b"all"
		{
			assert_eq!(text, "All");
			continue
		}
		
		let m49_code_array = match id.len()
		{
			0 => panic!("length was 0"),
			
			1 => [_0, _0, first(id)],
			
			2 => [_0, first(id), subsequent::<1>(id)],
			
			3 => [first(id), subsequent::<1>(id), subsequent::<2>(id)],
			
			length @ _ => panic!("length was {}, longer than 3", length)
		};
		
		let was = mapping.insert(m49_code_array, text);
		assert!(was.is_none(), "Duplicate M49 code {:?}", m49_code_array);
	}
	mapping
}
