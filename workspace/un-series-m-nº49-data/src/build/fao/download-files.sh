#!/usr/bin/env sh
# This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


set -e
set -f
set -u


index_to_m49_code()
{
	local index="$1"
	
	if [ "$index" -lt 10 ]; then
		printf '00'
	elif [ "$index" -lt 100 ]; then
		printf '0'
	fi
	printf '%d' "$index"
}

download_file_for_potential_m49_code()
{
	local file_extension="$1"
	local file_type="$2"
	local constant="$3"
	local index="$4"
	
	local m49_code="$(index_to_m49_code "$index")"
	
	local output_file="$file_type"/"$m49_code"."$file_extension"
	local exit_code=-1
	if [ ! -s "$output_file" ]; then
		set +e
			curl --location --max-redirs 3 --fail --silent "https://stats-class.fao.uniroma2.it/geo/m49/$m49_code.nt" --output "$output_file"
			exit_code=$?
		set -e
	fi
	
	if [ $exit_code -lt 1 ]; then
		count=$((count + 1))
		printf '\t(b"%s", include_bytes!("%s")),\n' "$m49_code" "$output_file"
		
		if [ $exit_code -eq -1 ]; then
			printf 'Processed %s\n' "$m49_code" 1>&2
		else
			printf 'Downloaded %s\n' "$m49_code" 1>&2
		fi
	fi
}

download_files_for_m49_codes()
{
	local file_extension="$1"
	local file_type="$2"
	local constant="$3"
	
	cat <<EOF
// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


const ${constant}: [(StaticM49Code, &'static str); COUNT] =
[
EOF
	
	local index=0
	while [ $index -ne 1000 ]
	do
		download_file_for_potential_m49_code "$file_extension" "$file_type" "$constant" "$index"
		index=$((index + 1))
	done

	cat <<EOF
];
EOF
}

generate_rust_code()
{
	local file_extension="$1"

	local file_type
	local constant
	case "$file_extension" in
		
		'nt')
			file_type='n_triples'
			constant='NTriplesFiles'
		;;
		
		'ttl')
			file_type='turtle'
			constant='TurtleFiles'
		;;
		
		'rdf')
			file_type='rdf'
			constant='RdfFiles'
		;;
		
		'html')
			file_type='html'
			constant='HtmlFiles'
		;;
		
		*)
			printf 'Unsupported file_extension %s\n' "$file_extension"
			exit 1
		;;
			
	esac
	
	mkdir -m 0700 -p "$file_type"
	
	local file_name="$file_type"/"$constant".rs
	local count=0
	download_files_for_m49_codes "$file_extension" "$file_type" "$constant" 1>"$file_name"
	sed -i -e 's/COUNT/'$count'/g' "$file_name"
}

main()
{
	# Other supported extensions are:-
	# ttl	Turtle
	# rdf	RDF/XML
	# html	HTML
	local file_extension='nt'
	generate_rust_code "$file_extension"
}

main "$@"
