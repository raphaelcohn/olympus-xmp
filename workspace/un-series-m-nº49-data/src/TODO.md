

* Write a parser for the historic developed / developing region coding xlsx
* Write a screen scraper using taiko for the ISO online browsing platform to extract all the ISO 3166 data
* Create static arrays (collections) for the various types of UN M.49 codes and ISO codes
  * eg Regions
  * eg Deleted
  * eg Name changed
  * etc
  * Make them sorted and use Rust's binary find (perhaps wrap it up)
* Use perfect hash functions (phf_map) to create static hashsets for the static arrays above
* Generate functions for enums that do a yes / no if an enum is in one of the (collections) above
* When finished parsing mapping, check Names has no optional records (alternatively, use static pointers to empty strings and check no empty string records - probabl more relevant as empty strins can still occur)
* Extract the names of the various hierarchies from the 6 Rev4 documents
* Rename historic copies of M49 Rev 3
  * Extract historic country names
* Review the history of country changes from UN Trade
  * This includes things like Indonesia re-numbering, which ?seems to be missing? from UN Stats list.
* Design functions to
  * Create successor states (splits)
  * Create successor states (merged into)
* Successor for sub-region South-Central Asia
* When parsing M.49 CSV
  * Include parent M.49 code
    * Include type of parent
    * Have functions to allow finding parent 
* Look at including Unicode CLDR country / region names, too.

The Unicode Common Locale Data Repository (CLDR) has an official JSON port which can be used directly from git (<https://github.com/unicode-org/cldr-json/tree/main/cldr-json>). This contains translations of terriority names into many, many languages, but by BCP 47 language tag. (e.g. <https://github.com/unicode-org/cldr-json/tree/main/cldr-json/cldr-localenames-full/main>). It supports alternative names.
