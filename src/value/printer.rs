use crate::value::Value;
use std::{fmt, fmt::Write, str::Chars};

#[derive(Copy, Clone)]
pub struct ValuePrinter<'indent> {
    indentation_level: usize,
    indentation: &'indent str,
    pretty: bool,
}

impl Default for ValuePrinter<'static> {
    fn default() -> Self {
        ValuePrinter::pretty()
    }
}

impl<'indent> ValuePrinter<'indent> {
    pub fn new(indentation: &'indent str, pretty: bool) -> Self {
        ValuePrinter {
            indentation_level: 0,
            indentation,
            pretty,
        }
    }

    pub fn pretty() -> Self {
        ValuePrinter::new("    ", true)
    }
    pub fn compact() -> Self {
        ValuePrinter::new("", false)
    }

    fn indent(self) -> Self {
        let mut new = self;
        new.indentation_level += 1;
        new
    }

    pub fn write<W>(self, value: &Value, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        match value {
            Value::Unit => write!(w, "()"),
            Value::Bool(b) => write!(w, "{}", b),
            Value::Char(c) => write!(w, "'{}'", escape_char(*c)),
            Value::String(s) => write!(w, "\"{}\"", escape_string(&s)),
            Value::Number(v) => write!(w, "{}", v),
            Value::Identifier(v) => write!(w, "{}", v),
            Value::List(list) => {
                write!(w, "[")?;
                self.write_items_list(w, &list)?;
                write!(w, "]")
            },
            Value::Tuple(tuple) => {
                write!(w, "(")?;
                self.write_items_list(w, &tuple)?;
                write!(w, ")")
            },
            Value::Map(map) => {
                write!(w, "{{")?;

                if !map.is_empty() {
                    self.write_newline(w)?;

                    self.indent().write_items(
                        w,
                        map,
                        |inner, (key, value), w| {
                            inner.write(key, w)?;

                            write!(w, ":")?;
                            if inner.pretty {
                                write!(w, " ")?;
                            }

                            inner.write(value, w)
                        },
                    )?;

                    self.write_indent(w)?;
                }

                write!(w, "}}")
            },
            Value::Option(option) => match option.as_ref() {
                None => write!(w, "None"),
                Some(value) => {
                    write!(w, "Some(")?;
                    self.write_newline(w)?;
                    {
                        let inner = self.indent();
                        inner.write_indent(w)?;
                        inner.write(value, w)?;
                    }
                    self.write_newline(w)?;
                    self.write_indent(w)?;
                    write!(w, ")")
                },
            },
            Value::Struct(identifier, items) => {
                write!(w, "{}(", identifier)?;

                if !items.is_empty() {
                    self.write_newline(w)?;

                    self.indent().write_items(
                        w,
                        items,
                        |inner, (key, value), w| {
                            write!(w, "{}", key)?;

                            write!(w, ":")?;
                            if inner.pretty {
                                write!(w, " ")?;
                            }

                            inner.write(value, w)
                        },
                    )?;

                    self.write_indent(w)?;
                }

                write!(w, ")")
            },
            Value::TupleStruct(identifier, tuple) => {
                write!(w, "{}(", identifier)?;
                self.write_items_list(w, &tuple)?;
                write!(w, ")")
            },
        }
    }

    fn write_indent<W>(self, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        if self.pretty {
            for _ in 0..self.indentation_level {
                write!(w, "{}", self.indentation)?;
            }
        }
        Ok(())
    }

    fn write_newline<W>(self, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        if self.pretty {
            write!(w, "\n")?;
        }
        Ok(())
    }

    fn write_items_list<W>(self, w: &mut W, items: &[Value]) -> fmt::Result
    where
        W: Write,
    {
        if !items.is_empty() {
            self.write_newline(w)?;

            self.indent()
                .write_items(w, items, |inner, it, w| inner.write(it, w))?;

            self.write_indent(w)?;
        }

        Ok(())
    }

    fn write_items<W, It, T, F>(
        self,
        w: &mut W,
        items: It,
        mut function: F,
    ) -> fmt::Result
    where
        W: Write,
        It: IntoIterator<Item = T>,
        F: FnMut(Self, T, &mut W) -> fmt::Result,
    {
        for it in items {
            self.write_indent(w)?;

            function(self, it, w)?;
            write!(w, ",")?;
            self.write_newline(w)?;
        }

        Ok(())
    }
}

enum StrOrCharIterator<'lt> {
    Str(Chars<'lt>),
    Char(Option<char>),
}

impl<'lt> Iterator for StrOrCharIterator<'lt> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            StrOrCharIterator::Str(chars) => chars.next(),
            StrOrCharIterator::Char(char) => char.take(),
        }
    }
}

impl<'lt> From<&'lt str> for StrOrCharIterator<'lt> {
    fn from(s: &'lt str) -> Self {
        StrOrCharIterator::Str(s.chars())
    }
}
impl From<char> for StrOrCharIterator<'static> {
    fn from(c: char) -> Self {
        StrOrCharIterator::Char(Some(c))
    }
}
impl From<EscapeResult> for StrOrCharIterator<'static> {
    fn from(r: EscapeResult) -> Self {
        match r {
            Ok(s) => s.into(),
            Err(c) => c.into(),
        }
    }
}

type EscapeResult = Result<&'static str, char>;

/// Common escape codes between strings and chars
fn escape_char_generic(input: char) -> EscapeResult {
    Ok(match input {
        '\\' => "\\\\",

        '\n' => "\\n",
        '\r' => "\\r",
        '\t' => "\\t",

        '\0' => "\\0",

        c => return Err(c),
    })
}

fn escape_char(c: char) -> String {
    let iterator: StrOrCharIterator = escape_char_generic(c)
        .or_else(|c| match c {
            '\'' => Ok("\\'"),
            c => Err(c),
        })
        .into();
    iterator.collect()
}

fn escape_string(s: &str) -> String {
    s.chars()
        .map(escape_char_generic)
        .map(|result| {
            result.or_else(|c| match c {
                '"' => Ok("\\\""),
                c => Err(c),
            })
        })
        .map(StrOrCharIterator::from)
        .flatten()
        .collect()
}
