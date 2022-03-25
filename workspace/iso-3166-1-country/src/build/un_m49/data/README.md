The files in this folder have to be downloaded from the website <https://unstats.un.org/unsd/methodology/m49/overview/> by choosing a language and clicking on the `CSV` button; there are no files to download directly, as the contents are generated on the fly by javascript as unique, one-time-use, URLs.

The project <https://github.com/datasets/country-codes> has an useful README.md containing links to other sources.

The Unicode Common Locale Data Repository (CLDR) has an official JSON port which can be used directly from git (<https://github.com/unicode-org/cldr-json/tree/main/cldr-json>). This contains translations of terriority names into many, many languages, but by BCP 47 language tag. (e.g. <https://github.com/unicode-org/cldr-json/tree/main/cldr-json/cldr-localenames-full/main>). It supports alternative names.

The files present here were downloaded on the 24th March 2022 between 1132 and 1137 GMT.

Despite this, data in the English file does not use double quotes, but that in the Arabic file does. That's poor, UN!
