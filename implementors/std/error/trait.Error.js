(function() {var implementors = {};
implementors["arrayvec"] = ["impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"arrayvec/struct.CapacityError.html\" title=\"struct arrayvec::CapacityError\">CapacityError</a>&lt;T&gt;",];
implementors["ascii"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"ascii/struct.ToAsciiCharError.html\" title=\"struct ascii::ToAsciiCharError\">ToAsciiCharError</a>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"ascii/struct.AsAsciiStrError.html\" title=\"struct ascii::AsAsciiStrError\">AsAsciiStrError</a>",];
implementors["base64"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"base64/enum.DecodeError.html\" title=\"enum base64::DecodeError\">DecodeError</a>",];
implementors["bincode"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">StdError</a> for <a class=\"enum\" href=\"bincode/internal/enum.ErrorKind.html\" title=\"enum bincode::internal::ErrorKind\">ErrorKind</a>",];
implementors["combine"] = ["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">StdError</a> for <a class=\"struct\" href=\"combine/primitives/struct.ParseError.html\" title=\"struct combine::primitives::ParseError\">ParseError</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"combine/trait.Stream.html\" title=\"trait combine::Stream\">Stream</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"combine/trait.StreamOnce.html#associatedtype.Range\" title=\"type combine::StreamOnce::Range\">Range</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"combine/trait.StreamOnce.html#associatedtype.Item\" title=\"type combine::StreamOnce::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"combine/trait.StreamOnce.html#associatedtype.Position\" title=\"type combine::StreamOnce::Position\">Position</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>,&nbsp;</span>",];
implementors["conv"] = ["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"conv/errors/enum.GeneralError.html\" title=\"enum conv::errors::GeneralError\">GeneralError</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>,&nbsp;</span>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"conv/errors/enum.GeneralErrorKind.html\" title=\"enum conv::errors::GeneralErrorKind\">GeneralErrorKind</a>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"conv/errors/enum.NoError.html\" title=\"enum conv::errors::NoError\">NoError</a>","impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"conv/errors/struct.Unrepresentable.html\" title=\"struct conv::errors::Unrepresentable\">Unrepresentable</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>,&nbsp;</span>","impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"conv/errors/struct.NegOverflow.html\" title=\"struct conv::errors::NegOverflow\">NegOverflow</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>,&nbsp;</span>","impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"conv/errors/struct.PosOverflow.html\" title=\"struct conv::errors::PosOverflow\">PosOverflow</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>,&nbsp;</span>","impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"conv/errors/enum.FloatError.html\" title=\"enum conv::errors::FloatError\">FloatError</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>,&nbsp;</span>","impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"conv/errors/enum.RangeError.html\" title=\"enum conv::errors::RangeError\">RangeError</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a>,&nbsp;</span>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"conv/errors/enum.RangeErrorKind.html\" title=\"enum conv::errors::RangeErrorKind\">RangeErrorKind</a>",];
implementors["darling_core"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">StdError</a> for <a class=\"struct\" href=\"darling_core/error/struct.Error.html\" title=\"struct darling_core::error::Error\">Error</a>",];
implementors["getopts"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"getopts/enum.Fail.html\" title=\"enum getopts::Fail\">Fail</a>",];
implementors["gif"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"gif/enum.DecodingError.html\" title=\"enum gif::DecodingError\">DecodingError</a>",];
implementors["hashglobe"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"hashglobe/struct.FailedAllocationError.html\" title=\"struct hashglobe::FailedAllocationError\">FailedAllocationError</a>",];
implementors["image"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"image/enum.ImageError.html\" title=\"enum image::ImageError\">ImageError</a>",];
implementors["jpeg_decoder"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">StdError</a> for <a class=\"enum\" href=\"jpeg_decoder/enum.Error.html\" title=\"enum jpeg_decoder::Error\">Error</a>",];
implementors["log"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"log/struct.SetLoggerError.html\" title=\"struct log::SetLoggerError\">SetLoggerError</a>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"log/struct.ShutdownLoggerError.html\" title=\"struct log::ShutdownLoggerError\">ShutdownLoggerError</a>",];
implementors["num_bigint"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"num_bigint/enum.ParseBigIntError.html\" title=\"enum num_bigint::ParseBigIntError\">ParseBigIntError</a>",];
implementors["num_complex"] = ["impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"num_complex/struct.ParseComplexError.html\" title=\"struct num_complex::ParseComplexError\">ParseComplexError</a>&lt;E&gt;",];
implementors["num_rational"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"num_rational/struct.ParseRatioError.html\" title=\"struct num_rational::ParseRatioError\">ParseRatioError</a>",];
implementors["ordered_float"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"ordered_float/struct.FloatIsNaN.html\" title=\"struct ordered_float::FloatIsNaN\">FloatIsNaN</a>",];
implementors["png"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"png/enum.DecodingError.html\" title=\"enum png::DecodingError\">DecodingError</a>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"png/enum.EncodingError.html\" title=\"enum png::EncodingError\">EncodingError</a>",];
implementors["rand"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"rand/jitter/enum.TimerError.html\" title=\"enum rand::jitter::TimerError\">TimerError</a>",];
implementors["rayon_core"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"rayon_core/struct.ThreadPoolBuildError.html\" title=\"struct rayon_core::ThreadPoolBuildError\">ThreadPoolBuildError</a>",];
implementors["regex"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"regex/enum.Error.html\" title=\"enum regex::Error\">Error</a>",];
implementors["regex_syntax"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"regex_syntax/struct.Error.html\" title=\"struct regex_syntax::Error\">Error</a>",];
implementors["rustc_serialize"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"rustc_serialize/base64/enum.FromBase64Error.html\" title=\"enum rustc_serialize::base64::FromBase64Error\">FromBase64Error</a>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"rustc_serialize/hex/enum.FromHexError.html\" title=\"enum rustc_serialize::hex::FromHexError\">FromHexError</a>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">StdError</a> for <a class=\"enum\" href=\"rustc_serialize/json/enum.DecoderError.html\" title=\"enum rustc_serialize::json::DecoderError\">DecoderError</a>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">StdError</a> for <a class=\"enum\" href=\"rustc_serialize/json/enum.ParserError.html\" title=\"enum rustc_serialize::json::ParserError\">ParserError</a>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">StdError</a> for <a class=\"enum\" href=\"rustc_serialize/json/enum.EncoderError.html\" title=\"enum rustc_serialize::json::EncoderError\">EncoderError</a>",];
implementors["serde"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"serde/de/value/struct.Error.html\" title=\"struct serde::de::value::Error\">Error</a>",];
implementors["serde_json"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>",];
implementors["time"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"time/struct.OutOfRangeError.html\" title=\"struct time::OutOfRangeError\">OutOfRangeError</a>","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"time/enum.ParseError.html\" title=\"enum time::ParseError\">ParseError</a>",];
implementors["url"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"url/enum.ParseError.html\" title=\"enum url::ParseError\">ParseError</a>",];
implementors["uuid"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"uuid/enum.ParseError.html\" title=\"enum uuid::ParseError\">ParseError</a>",];
implementors["xdg"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"xdg/struct.BaseDirectoriesError.html\" title=\"struct xdg::BaseDirectoriesError\">BaseDirectoriesError</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
