# strutl
String matching utilities

As of now, only following is supported:
1. Jaro-Winkler distance
2. Soundex

_Implementation note_: The Jaro-Winkler Distance implementation works on bytes instead of chars. So, the results match for ASCII text with results of other implementations such as strsim but not for text-containing non-ascii chars.
