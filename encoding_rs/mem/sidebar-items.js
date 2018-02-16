initSidebarItems({"enum":[["Latin1Bidi","Classification of text as Latin1 (all code points are below U+0100), left-to-right with some non-Latin1 characters or as containing at least some right-to-left characters."]],"fn":[["check_str_for_latin1_and_bidi","Checks whether a valid UTF-8 buffer contains code points that trigger right-to-left processing or is all-Latin1."],["check_utf16_for_latin1_and_bidi","Checks whether a potentially invalid UTF-16 buffer contains code points that trigger right-to-left processing or is all-Latin1."],["check_utf8_for_latin1_and_bidi","Checks whether a potentially invalid UTF-8 buffer contains code points that trigger right-to-left processing or is all-Latin1."],["convert_latin1_to_str","Converts bytes whose unsigned value is interpreted as Unicode code point (i.e. U+0000 to U+00FF, inclusive) to UTF-8 such that the validity of the output is signaled using the Rust type system."],["convert_latin1_to_utf16","Converts bytes whose unsigned value is interpreted as Unicode code point (i.e. U+0000 to U+00FF, inclusive) to UTF-16."],["convert_latin1_to_utf8","Converts bytes whose unsigned value is interpreted as Unicode code point (i.e. U+0000 to U+00FF, inclusive) to UTF-8."],["convert_str_to_utf16","Converts valid UTF-8 to valid UTF-16."],["convert_utf16_to_latin1_lossy","If the input is valid UTF-16 representing only Unicode code points from U+0000 to U+00FF, inclusive, converts the input into output that represents the value of each code point as the unsigned byte value of each output byte."],["convert_utf16_to_str","Converts potentially-invalid UTF-16 to valid UTF-8 with errors replaced with the REPLACEMENT CHARACTER such that the validity of the output is signaled using the Rust type system."],["convert_utf16_to_utf8","Converts potentially-invalid UTF-16 to valid UTF-8 with errors replaced with the REPLACEMENT CHARACTER."],["convert_utf8_to_latin1_lossy","If the input is valid UTF-8 representing only Unicode code points from U+0000 to U+00FF, inclusive, converts the input into output that represents the value of each code point as the unsigned byte value of each output byte."],["convert_utf8_to_utf16","Converts potentially-invalid UTF-8 to valid UTF-16 with errors replaced with the REPLACEMENT CHARACTER."],["copy_ascii_to_ascii","Copies ASCII from source to destination up to the first non-ASCII byte (or the end of the input if it is ASCII in its entirety)."],["copy_ascii_to_basic_latin","Copies ASCII from source to destination zero-extending it to UTF-16 up to the first non-ASCII byte (or the end of the input if it is ASCII in its entirety)."],["copy_basic_latin_to_ascii","Copies Basic Latin from source to destination narrowing it to ASCII up to the first non-Basic Latin code unit (or the end of the input if it is Basic Latin in its entirety)."],["ensure_utf16_validity","Replaces unpaired surrogates in the input with the REPLACEMENT CHARACTER."],["is_ascii","Checks whether the buffer is all-ASCII."],["is_basic_latin","Checks whether the buffer is all-Basic Latin (i.e. UTF-16 representing only ASCII characters)."],["is_char_bidi","Checks whether a code point triggers right-to-left processing."],["is_str_bidi","Checks whether a valid UTF-8 buffer contains code points that trigger right-to-left processing."],["is_str_latin1","Checks whether the buffer represents only code point less than or equal to U+00FF."],["is_utf16_bidi","Checks whether a UTF-16 buffer contains code points that trigger right-to-left processing."],["is_utf16_code_unit_bidi","Checks whether a UTF-16 code unit triggers right-to-left processing."],["is_utf16_latin1","Checks whether the buffer represents only code point less than or equal to U+00FF."],["is_utf8_bidi","Checks whether a potentially-invalid UTF-8 buffer contains code points that trigger right-to-left processing."],["is_utf8_latin1","Checks whether the buffer is valid UTF-8 representing only code points less than or equal to U+00FF."],["utf16_valid_up_to","Returns the index of the first unpaired surrogate or, if the input is valid UTF-16 in its entirety, the length of the input."]]});