use crate::context::Context;
use crate::error::Error;
use crate::locale::Locale;

/// A `Formatter` trait.
///
/// # Syntax.
///
/// Each `Formatter` implementation defines it's own syntax rules for the message format,
/// this includes the syntax used for accessing context values, and plural message syntax.
///
/// The default message syntax is quite similar to rust's own fmt syntax.
///
/// To include a context value in your message, you can use own of the three following syntax options:
///
/// 1. name: `Hello, {name}! How is {other} doing?`.
/// 2. position: `Hello, {0}! How is {1} doing?`.
/// 3. index: `Hello, {}! How is {} doing?`.
///
/// The plural message syntax is defined using `|`, where each message has it own rule, followed by a
/// default message.
///
/// The special `?` context variable can be used to access the `count` value.
///
/// ```toml
/// apple = "{0} There are no apples | {1} There is one apple | {2..4} There are few apples | There are {?} apples"
/// ```
///
/// A rule can represent a list of numbers, such as:
///
/// - `{1}`
/// - `{1, 2, 3}`
///
/// or a range:
///
/// - `{5..}`
/// - `{..3}`
/// - `{2..4}`
pub trait Formatter: Send + Sync {
    fn format(
        &self,
        locale: &Locale,
        message: &str,
        context: &Context,
    ) -> Result<String, Error>;

    fn box_clone(&self) -> Box<dyn Formatter>;
}

/// Default implementation for `Formatter`.
///
/// # Examples
///
/// ```
/// use tarjama::context;
/// use tarjama::locale::Locale;
/// use tarjama::locale::ArabicVariant;
/// use tarjama::formatter::Formatter;
///
/// let formatter: Box<dyn Formatter> = Default::default();
///
/// let message = formatter.format(
///     &Locale::Arabic(ArabicVariant::Tunisia),
///     "!أهلا {name}",
///     &context!(name = "سيف")
/// ).unwrap();
///
/// assert_eq!(message, "!أهلا سيف".to_string());
/// ```
impl Default for Box<dyn Formatter> {
    // Return a default implementation for `Formatter`.
    fn default() -> Self {
        DefaultFormatter::new().into()
    }
}

impl Clone for Box<dyn Formatter> {
    fn clone(&self) -> Box<dyn Formatter> {
        self.box_clone()
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, Hash)]
struct DefaultFormatter;

unsafe impl Send for DefaultFormatter {}
unsafe impl Sync for DefaultFormatter {}

impl DefaultFormatter {
    pub fn new() -> DefaultFormatter {
        DefaultFormatter
    }
}

impl From<DefaultFormatter> for Box<dyn Formatter> {
    fn from(formatter: DefaultFormatter) -> Self {
        Box::new(formatter)
    }
}

impl Formatter for DefaultFormatter {
    fn format(
        &self,
        _: &Locale,
        message: &str,
        context: &Context,
    ) -> Result<String, Error> {
        let message = if let Some(count) = context.count {
            let plural_messages = parse_plural_messages(message)?;

            plural_messages.matching(count)
        } else {
            message.to_string()
        };

        format_raw(message, context)
    }

    fn box_clone(&self) -> Box<dyn Formatter> {
        Box::new(self.clone())
    }
}

impl Default for DefaultFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)]
#[derive(Debug)]
enum Rule {
    RangeTo { to: i64 },
    RangeFrom { from: i64 },
    Range { from: i64, to: i64 },
    Match { values: Vec<i64> },
}

impl Rule {
    pub fn matches(&self, value: i64) -> bool {
        match self {
            Rule::RangeTo { to } => value <= *to,
            Rule::RangeFrom { from } => value >= *from,
            Rule::Range { from, to } => value >= *from && value <= *to,
            Rule::Match { values } => values.contains(&value),
        }
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::RangeTo { to } => write!(f, "{{..{}}}", to),
            Rule::RangeFrom { from } => write!(f, "{{{}..}}", from),
            Rule::Range { from, to } => write!(f, "{{{}..{}}}", from, to),
            Rule::Match { values } => write!(
                f,
                "{{{}}}",
                values
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

#[doc(hidden)]
#[derive(Debug)]
struct PluralMessages {
    pub rules: Vec<(String, Rule)>,
    pub default: String,
}

impl PluralMessages {
    pub fn matching(&self, value: i64) -> String {
        for (message, rule) in &self.rules {
            if rule.matches(value) {
                return message.clone();
            }
        }

        return self.default.clone();
    }
}

fn format_raw(message: String, context: &Context) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut arg_idx = 0;
    let mut position = 0;
    while let Some(mut current_position) =
        message[position..].find(|x| x == '{' || x == '}')
    {
        if message.len() - position < current_position + 1 {
            break;
        }

        current_position += position;

        // Skip escaped }
        if message.get(current_position..=current_position) == Some("}") {
            buffer.push_str(&message[position..=current_position]);

            match message.get(current_position + 1..=current_position + 1) {
                Some("}") => {
                    position = current_position + 2;
                }
                Some(u) => {
                    return Err(Error::FormattingError(format!(
                        "invalid format string: expected `'}}'`, found `'{u}'`."
                    )));
                }
                None => {
                    return Err(Error::FormattingError(
                        "invalid format string: expected `'}'` but string was terminated."
                            .to_string(),
                    ));
                }
            }

            continue;
        }

        // Skip escaped {
        if message.get(current_position + 1..=current_position + 1)
            == Some("{")
        {
            buffer.push_str(&message[position..=current_position]);
            position = current_position + 2;

            continue;
        }

        let left_curly_brackets_position = match message[current_position..]
            .find('}')
        {
            Some(left_curly_brackets_position) => {
                left_curly_brackets_position + current_position
            }
            None => {
                return Err(Error::FormattingError(
                    "invalid format string: expected `'}'` but string was terminated.".to_string(),
                ));
            }
        };

        let argument_name =
            message[current_position + 1..left_curly_brackets_position].trim();
        let argument_value_index = if current_position
            == left_curly_brackets_position - 1
        {
            arg_idx += 1;
            if context.values.len() < arg_idx {
                return Err(Error::FormattingError(format!(
                    "invalid reference to indexed value `'{}'` (there is {} value).",
                    arg_idx - 1,
                    context.values.len()
                )));
            }

            Some(arg_idx - 1)
        } else if let Ok(n) = argument_name.parse::<usize>() {
            Some(n)
        } else if let Some(p) =
            context.values.iter().position(|x| x.0 == argument_name)
        {
            Some(p)
        } else if argument_name == "?" {
            None
        } else {
            return Err(Error::FormattingError(format!(
                "cannot find value `'{argument_name}'` in this context."
            )));
        };

        // push the part before the '{' to the buffer
        buffer.push_str(&message[position..current_position]);
        if let Some(index) = argument_value_index {
            if let Some(a) = context.values.get(index) {
                buffer.push_str(&a.1.to_string());
            } else {
                return Err(Error::FormattingError(format!(
                    "invalid reference to positional value `'{}'` (there is {} value).",
                    index,
                    context.values.len()
                )));
            }
        } else if let Some(count) = context.count {
            buffer.push_str(&count.to_string());
        } else {
            return Err(Error::FormattingError(
                "invalid reference to count ( {?} ) value.".to_string(),
            ));
        }
        position = left_curly_brackets_position + 1;
    }

    buffer.push_str(&message[position..]);

    if context.count.is_some() {
        Ok(buffer.replace("||", "|"))
    } else {
        Ok(buffer)
    }
}

#[doc(hidden)]
fn parse_plural_messages(message: &str) -> Result<PluralMessages, Error> {
    let message = message.trim();
    let mut messages: Vec<_> = vec![];

    if !message.contains('|') {
        if !message.is_empty() {
            messages.push(message.to_string());
        }
    } else {
        use unicode_segmentation::UnicodeSegmentation;
        let graphemes = UnicodeSegmentation::graphemes(message, true)
            .collect::<Vec<&str>>();

        let mut i = 0usize;
        let mut s = 0usize;
        let mut start = 0usize;
        while i < graphemes.len() {
            if graphemes[i] == "|" {
                s += 1;
                // lookahead for ||
                if i + 1 < graphemes.len()
                    && graphemes[i + 1] != "|"
                    && s % 2 != 0
                {
                    messages
                        .push(graphemes[start..i].concat().trim().to_string());
                    start = i + 1;
                }
            } else {
                s = 0;
            }
            i += 1;
        }
        if start < graphemes.len() {
            messages.push(graphemes[start..].concat().trim().to_string()); // Push the remaining graphemes
        }
    }

    if let Some((last, messages)) = messages.split_last() {
        let mut rules: Vec<(String, Rule)> = vec![];
        for message in messages {
            if message.starts_with('{') {
                if let Some(ending_position) = message.find('}') {
                    let rule_string = &message[1..ending_position];
                    let target = &message[ending_position + 1..].trim();
                    let rule = if let Some(sep_position) =
                        rule_string.find("..")
                    {
                        if sep_position == 0 {
                            Rule::RangeTo {
                                to: match rule_string[2..].parse::<i64>() {
                                    Ok(to) => to,
                                    Err(e) => {
                                        return Err(Error::FormattingError(format!(
                                            "formatting: failed to parse `'to'` value in range-to rule for `'{message}'`, {e}.",
                                        )));
                                    }
                                },
                            }
                        } else if sep_position == rule_string.len() - 2 {
                            Rule::RangeFrom {
                                from: match rule_string[..sep_position]
                                    .parse::<i64>()
                                {
                                    Ok(from) => from,
                                    Err(e) => {
                                        return Err(Error::FormattingError(format!(
                                            "formatting: failed to parse `'from'` value in range-from rule for `'{message}'`, {e}.",
                                        )));
                                    }
                                },
                            }
                        } else {
                            Rule::Range {
                                from: match rule_string[..sep_position]
                                    .parse::<i64>()
                                {
                                    Ok(from) => from,
                                    Err(e) => {
                                        return Err( Error::FormattingError(format!(
                                            "formatting: failed to parse `'from'` value in range rule for `'{message}'`, {e}.",
                                        )));
                                    }
                                },
                                to: match rule_string[sep_position + 2..]
                                    .parse::<i64>()
                                {
                                    Ok(to) => to,
                                    Err(e) => {
                                        return Err( Error::FormattingError(format!(
                                            "formatting: failed to parse `'to'` value in range rule for `'{message}'`, {e}.",
                                        )));
                                    }
                                },
                            }
                        }
                    } else {
                        let mut values = vec![];
                        for value_str in
                            rule_string.split(',').map(|s| s.trim())
                        {
                            values.push(match value_str.parse::<i64>() {
                                Ok(value) => value,
                                Err(e) => {
                                    return Err( Error::FormattingError(format!(
                                        "formatting: failed to parse value `'{value_str}'` in match rule for `'{message}'`, {e}.",
                                    )));
                                },
                            });
                        }

                        Rule::Match { values }
                    };

                    rules.push((target.to_string(), rule));
                } else {
                    return Err(Error::FormattingError(format!("formatting: failed to parse rule for `'{message}'`, expected `'}}'` but string was terminated.")));
                }
            } else {
                return Err(Error::FormattingError(format!("formatting: failed to parse rule for `'{message}'`, expected `'{{'` but string was terminated.")));
            }
        }

        Ok(PluralMessages { rules, default: last.to_string() })
    } else {
        Err(Error::FormattingError("formatting: failed to parse plural messages, expected at least a default message but string was terminated.".to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::context;
    use crate::formatter::Formatter;
    use crate::locale::EnglishVariant;
    use crate::locale::Locale;

    macro_rules! assert_ok {
        ($msg:expr, $context:expr, $expected:literal) => {{
            let formatter: Box<dyn Formatter> = Default::default();
            let msg = formatter.format(
                &Locale::English(EnglishVariant::Default),
                $msg,
                &$context,
            );

            assert!(msg.is_ok());
            assert_eq!(msg.unwrap(), $expected.to_string());
        }};
    }

    macro_rules! assert_err {
        ($msg:expr, $context:expr, $expected:literal) => {{
            let formatter: Box<dyn Formatter> = Default::default();
            let msg = formatter.format(
                &Locale::English(EnglishVariant::Default),
                $msg,
                &$context,
            );

            assert!(msg.is_err());
            assert_eq!(msg.unwrap_err().to_string(), $expected.to_string());
        }};
    }

    #[test]
    fn format_syntax() {
        assert_ok!(
            "a = {a}, b = {1}, a = {}, b = {}, c = {}, c = {2}, c = {c}, count = {?}",
            context!(a = 1, b = 2, c = 3, ? = 5),
            "a = 1, b = 2, a = 1, b = 2, c = 3, c = 3, c = 3, count = 5"
        );
    }

    #[test]
    fn access_undefined() {
        assert_err!(
            "a = {a}, b = {1}, a = {}, b = {}, c = {}, c = {2}, c = {c}, count = {?}, d = {d}",
            context!(a = 1, b = 2, c = 3, ? = 5),
            "cannot find value `'d'` in this context."
        );

        assert_err!(
            "a = {a}, b = {1}, a = {}, b = {}, c = {}, c = {2}, c = {c}, count = {?}, d = {d}",
            context!(a = 1, b = 2, c = 3, ? = 5),
            "cannot find value `'d'` in this context."
        );
    }

    #[test]
    fn access_undefined_positioned_context() {
        assert_err!(
            "a = {a}, b = {1}, a = {}, b = {}, c = {}, c = {2}, c = {c}, count = {?}, d = {3}",
            context!(a = 1, b = 2, c = 3, ? = 5),
            "invalid reference to positional value `'3'` (there is 3 value)."
        );
    }

    #[test]
    fn access_undefined_indexed_context() {
        assert_err!(
            "a = {a}, b = {1}, a = {}, b = {}, c = {}, c = {2}, c = {c}, count = {?}, d = {}",
            context!(a = 1, b = 2, c = 3, ? = 5),
            "invalid reference to indexed value `'3'` (there is 3 value)."
        );
    }

    #[test]
    fn access_undefined_count_context() {
        assert_err!(
            "a = {a}, b = {1}, a = {}, b = {}, c = {}, c = {2}, c = {c}, count = {?}",
            context!(a = 1, b = 2, c = 3),
            "invalid reference to count ( {?} ) value."
        );
    }

    #[test]
    fn escape_brackets() {
        assert_ok!("a = {{{a}}}", context!(a = 1), "a = {1}");
        assert_ok!("{{", context!(), "{");
        assert_ok!("}}", context!(), "}");
    }

    #[test]
    fn rule_matching() {
        let message =
            "{0} foo | {1, 2} bar | {..5} baz | {10..} qux | fizz || bizz";

        assert_ok!(message, context!(? = 0), "foo");
        assert_ok!(message, context!(? = 1), "bar");
        assert_ok!(message, context!(? = 2), "bar");
        assert_ok!(message, context!(? = 3), "baz");
        assert_ok!(message, context!(? = 4), "baz");
        assert_ok!(message, context!(? = 5), "baz");
        assert_ok!(message, context!(? = 6), "fizz | bizz");
        assert_ok!(message, context!(? = 7), "fizz | bizz");
        assert_ok!(message, context!(? = 8), "fizz | bizz");
        assert_ok!(message, context!(? = 9), "fizz | bizz");
        assert_ok!(message, context!(? = 10), "qux");
        assert_ok!(message, context!(? = 100), "qux");
    }

    #[test]
    fn message_parse_errors() {
        assert_err!(
            "foo {bar",
            context!(),
            "invalid format string: expected `'}'` but string was terminated."
        );
        assert_err!(
            "foo {",
            context!(),
            "invalid format string: expected `'}'` but string was terminated."
        );
        assert_err!(
            "}{",
            context!(),
            "invalid format string: expected `'}'`, found `'{'`."
        );
        assert_err!(
            "}{",
            context!(),
            "invalid format string: expected `'}'`, found `'{'`."
        );
        assert_err!(
            "{",
            context!(),
            "invalid format string: expected `'}'` but string was terminated."
        );
    }

    #[test]
    fn plural_messages_parse_errors() {
        assert_err!(
            "{0} foo | {1 bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'{1 bar'`, expected `'}'` but string was terminated."
        );

        assert_err!(
            "{0} foo | 1} bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'1} bar'`, expected `'{'` but string was terminated."
        );

        assert_err!(
            "{0} foo | {one} bar | baz",
            context!(? = 2),
            "formatting: failed to parse value `'one'` in match rule for `'{one} bar'`, invalid digit found in string."
        );

        assert_err!(
            "{0} foo | {1, two bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'{1, two bar'`, expected `'}'` but string was terminated."
        );

        assert_err!(
            "{0} foo | 1, two} bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'1, two} bar'`, expected `'{'` but string was terminated."
        );

        assert_err!(
            "{0} foo | {1, two} bar | baz",
            context!(? = 2),
            "formatting: failed to parse value `'two'` in match rule for `'{1, two} bar'`, invalid digit found in string."
        );

        assert_err!(
            "{0} foo | ..two} bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'..two} bar'`, expected `'{'` but string was terminated."
        );

        assert_err!(
            "{0} foo | {..two bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'{..two bar'`, expected `'}'` but string was terminated."
        );

        assert_err!(
            "{0} foo | {..two} bar | baz",
            context!(? = 2),
            "formatting: failed to parse `'to'` value in range-to rule for `'{..two} bar'`, invalid digit found in string."
        );

        assert_err!(
            "{0} foo | two..} bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'two..} bar'`, expected `'{'` but string was terminated."
        );

        assert_err!(
            "{0} foo | {two.. bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'{two.. bar'`, expected `'}'` but string was terminated."
        );

        assert_err!(
            "{0} foo | {two..} bar | baz",
            context!(? = 2),
            "formatting: failed to parse `'from'` value in range-from rule for `'{two..} bar'`, invalid digit found in string."
        );

        assert_err!(
            "{0} foo | {one..5} bar | baz",
            context!(? = 2),
            "formatting: failed to parse `'from'` value in range rule for `'{one..5} bar'`, invalid digit found in string."
        );

        assert_err!(
            "{0} foo | 2..5} bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'2..5} bar'`, expected `'{'` but string was terminated."
        );

        assert_err!(
            "{0} foo | {2..5 bar | baz",
            context!(? = 2),
            "formatting: failed to parse rule for `'{2..5 bar'`, expected `'}'` but string was terminated."
        );

        assert_err!(
            "{0} foo | {2....5} bar | baz",
            context!(? = 2),
            "formatting: failed to parse `'to'` value in range rule for `'{2....5} bar'`, invalid digit found in string."
        );

        assert_err!(
            "{0} foo | {2.5} bar | baz",
            context!(? = 2),
            "formatting: failed to parse value `'2.5'` in match rule for `'{2.5} bar'`, invalid digit found in string."
        );

        assert_err!(
            "",
            context!(? = 2),
            "formatting: failed to parse plural messages, expected at least a default message but string was terminated."
        );
    }
}
