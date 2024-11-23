use chumsky::{ Parser, extra::ParserExtra };

/**
 * Parser for whitespace char (including newline.)
 */
pub(crate) fn whitespace_char_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, (), E>
  where E: ParserExtra<'a, &'a str>
{
    use chumsky::prelude::one_of;
    one_of(" \t\n\r").ignored()
}

/**
 * Parser for whitespace (including newline.)
 */
pub(crate) fn whitespace_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, (), E>
  where E: ParserExtra<'a, &'a str>
{
    whitespace_char_parser().repeated().ignored()
}

/**
 * Lowercase letter parser.
 */
pub(crate) fn lowercase_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, char, E>
  where E: ParserExtra<'a, &'a str>
{
    use chumsky::prelude::*;
    one_of("abcdefghijklmnopqrstuvwxyz")
}

/**
 * Uppercase letter parser.
 */
pub(crate) fn uppercase_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, char, E>
  where E: ParserExtra<'a, &'a str>
{
    use chumsky::prelude::*;
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
}

/**
 * Decimal digit parser.
 */
pub(crate) fn dec_digit_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, char, E>
  where E: ParserExtra<'a, &'a str>
{
    use chumsky::prelude::*;
    one_of("0123456789")
}

/**
 * Binary digit parser.
 */
pub(crate) fn bin_digit_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, char, E>
  where E: ParserExtra<'a, &'a str>
{
    use chumsky::prelude::*;
    one_of("01")
}

/**
 * Octal digit parser.
 */
pub(crate) fn oct_digit_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, char, E>
  where E: ParserExtra<'a, &'a str>
{
    use chumsky::prelude::*;
    one_of("01234567")
}

/**
 * Hexadecimal digit parser.
 */
pub(crate) fn hex_digit_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, char, E>
  where E: ParserExtra<'a, &'a str>
{
    use chumsky::prelude::*;
    one_of("0123456789abcdefABCDEF")
}

/**
 * A helper to terminate a statement with a semicolon.
 */
pub(crate) fn terminal_semicolon_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, (), E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;
  just(';').padded_by(whitespace_parser()).map(|_| ())
}
