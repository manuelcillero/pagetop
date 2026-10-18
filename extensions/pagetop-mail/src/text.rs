// Conversión del HTML de un correo en su texto plano alternativo.
//
// No es un intérprete HTML completo: basta para el HTML de los correos, que normalmente sale de
// `html!` y está bien formado. Los bloques (párrafos, encabezados, listas, filas de tabla...) se
// separan con saltos de línea, los elementos de lista se marcan con "- ", los enlaces añaden su
// dirección entre paréntesis y las imágenes su texto alternativo. Se descartan `head`, `title`,
// `style` y `script`, y los comentarios.

// Elementos que separan párrafos (una línea en blanco) y que separan líneas.
const PARAGRAPHS: &[&str] = &[
    "p",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "ul",
    "ol",
    "table",
    "blockquote",
    "pre",
    "dl",
];
const LINES: &[&str] = &[
    "div", "tr", "li", "dt", "dd", "section", "article", "header", "footer", "body", "html",
];
// Elementos cuyo contenido no es texto del correo.
const SKIPPED: &[&str] = &["head", "title", "style", "script"];

pub(crate) fn html_to_text(html: &str) -> String {
    let mut text = Text::default();
    let mut links: Vec<(usize, Option<String>)> = Vec::new();
    let mut pre = 0usize;
    let mut rest = html;

    while let Some(start) = rest.find('<') {
        text.push(&decode(&rest[..start]), pre > 0);
        rest = &rest[start..];

        if let Some(comment) = rest.strip_prefix("<!--") {
            rest = comment.find("-->").map_or("", |end| &comment[end + 3..]);
            continue;
        }
        let Some(tag) = Tag::parse(rest) else {
            // Un `<` que no abre una etiqueta es texto.
            text.push("<", pre > 0);
            rest = &rest[1..];
            continue;
        };
        rest = &rest[tag.len..];

        if !tag.closing && SKIPPED.contains(&tag.name.as_str()) {
            let close = ["</", &tag.name].concat();
            rest = match find_ignore_case(rest, &close) {
                Some(pos) => {
                    let after = &rest[pos..];
                    after.find('>').map_or("", |gt| &after[gt + 1..])
                }
                None => "",
            };
            continue;
        }

        match (tag.name.as_str(), tag.closing) {
            ("br", _) => text.line_break(),
            ("hr", _) => {
                text.block(2);
                text.emit("----");
                text.block(2);
            }
            ("li", false) => {
                text.block(1);
                text.emit("- ");
            }
            ("td" | "th", true) => text.space(),
            ("img", _) => {
                if let Some(alt) = tag.attr("alt").filter(|alt| !alt.trim().is_empty()) {
                    text.push(&alt, false);
                }
            }
            ("a", false) => links.push((text.len(), tag.attr("href"))),
            ("a", true) => {
                if let Some((start, Some(href))) = links.pop() {
                    let label = text.since(start).trim().to_owned();
                    let target = href.strip_prefix("mailto:").unwrap_or(&href);
                    let is_link = !(href.is_empty()
                        || href.starts_with('#')
                        || href.starts_with("cid:")
                        || href.starts_with("javascript:"));
                    if is_link && label != target && label != href {
                        text.emit_inline(&["(", target, ")"].concat());
                    }
                }
            }
            ("pre", closing) => {
                text.block(2);
                if closing {
                    pre = pre.saturating_sub(1);
                } else {
                    pre += 1;
                }
            }
            (name, _) if PARAGRAPHS.contains(&name) => text.block(2),
            (name, _) if LINES.contains(&name) => text.block(1),
            _ => {}
        }
    }
    text.push(&decode(rest), pre > 0);
    text.finish()
}

// Texto en construcción, con los separadores pendientes hasta que llegue más texto: así no quedan
// espacios ni saltos de línea sobrantes al principio, al final o entre bloques.
#[derive(Default)]
struct Text {
    buf: String,
    newlines: usize,
    space: bool,
}

impl Text {
    fn len(&self) -> usize {
        self.buf.len()
    }

    fn since(&self, start: usize) -> &str {
        self.buf.get(start..).unwrap_or_default()
    }

    // Añade texto. Fuera de `pre` cualquier secuencia de espacios en blanco cuenta como un espacio.
    fn push(&mut self, s: &str, preformatted: bool) {
        if preformatted {
            if !s.is_empty() {
                self.flush();
                self.buf.push_str(s);
            }
            return;
        }
        for c in s.chars() {
            if c.is_whitespace() {
                self.space();
            } else {
                self.flush();
                self.buf.push(c);
            }
        }
    }

    // Añade un texto tal cual, tras los separadores pendientes.
    fn emit(&mut self, s: &str) {
        self.flush();
        self.buf.push_str(s);
    }

    // Añade un texto separado por un espacio de lo anterior en la misma línea.
    fn emit_inline(&mut self, s: &str) {
        self.space();
        self.emit(s);
    }

    fn space(&mut self) {
        if self.newlines == 0 {
            self.space = true;
        }
    }

    fn line_break(&mut self) {
        self.newlines = (self.newlines + 1).min(2);
        self.space = false;
    }

    fn block(&mut self, newlines: usize) {
        self.newlines = self.newlines.max(newlines);
        self.space = false;
    }

    fn flush(&mut self) {
        if !self.buf.is_empty() {
            if self.newlines > 0 {
                let trimmed = self.buf.trim_end_matches(' ').len();
                self.buf.truncate(trimmed);
                for _ in 0..self.newlines {
                    self.buf.push('\n');
                }
            } else if self.space && !self.buf.ends_with([' ', '\n']) {
                self.buf.push(' ');
            }
        }
        self.newlines = 0;
        self.space = false;
    }

    fn finish(self) -> String {
        let mut out = String::with_capacity(self.buf.len());
        for line in self.buf.lines() {
            out.push_str(line.trim_end());
            out.push('\n');
        }
        out.trim().to_owned()
    }
}

// Etiqueta HTML leída al principio de un texto que empieza por `<`.
struct Tag<'a> {
    name: String,
    closing: bool,
    attrs: &'a str,
    // Longitud de la etiqueta completa, de `<` a `>`.
    len: usize,
}

impl<'a> Tag<'a> {
    fn parse(s: &'a str) -> Option<Self> {
        let body = s.strip_prefix('<')?;
        let (closing, body) = match body.strip_prefix('/') {
            Some(body) => (true, body),
            None => (false, body),
        };
        // `<!DOCTYPE ...>` y similares se leen como etiquetas sin nombre conocido.
        let first = body.chars().next()?;
        if !(first.is_ascii_alphabetic() || first == '!') {
            return None;
        }
        // Busca el `>` final fuera de los valores entre comillas.
        let mut quote = None;
        let mut end = None;
        for (i, c) in body.char_indices() {
            match (quote, c) {
                (Some(q), c) if c == q => quote = None,
                (None, '"' | '\'') => quote = Some(c),
                (None, '>') => {
                    end = Some(i);
                    break;
                }
                _ => {}
            }
        }
        let end = end?;
        let inner = body[..end].trim_end_matches('/');
        let name_len = inner
            .find(|c: char| c.is_whitespace())
            .unwrap_or(inner.len());
        Some(Tag {
            name: inner[..name_len].to_ascii_lowercase(),
            closing,
            attrs: &inner[name_len..],
            len: s.len() - body.len() + end + 1,
        })
    }

    // Valor del atributo `name`, ya decodificado.
    fn attr(&self, name: &str) -> Option<String> {
        let mut rest = self.attrs;
        loop {
            rest = rest.trim_start();
            if rest.is_empty() {
                return None;
            }
            let key_len = rest
                .find(|c: char| c == '=' || c.is_whitespace())
                .unwrap_or(rest.len());
            let key = &rest[..key_len];
            rest = rest[key_len..].trim_start();
            let value = match rest.strip_prefix('=') {
                Some(after) => {
                    let after = after.trim_start();
                    let (value, next) = match after.chars().next() {
                        Some(q @ ('"' | '\'')) => {
                            let inner = &after[1..];
                            let end = inner.find(q).unwrap_or(inner.len());
                            (&inner[..end], inner.get(end + 1..).unwrap_or_default())
                        }
                        _ => {
                            let end = after.find(char::is_whitespace).unwrap_or(after.len());
                            (&after[..end], &after[end..])
                        }
                    };
                    rest = next;
                    value
                }
                None => "",
            };
            if key.eq_ignore_ascii_case(name) {
                return Some(decode(value));
            }
        }
    }
}

// Posición de `needle` en `haystack` sin distinguir mayúsculas (ASCII).
fn find_ignore_case(haystack: &str, needle: &str) -> Option<usize> {
    haystack
        .to_ascii_lowercase()
        .find(&needle.to_ascii_lowercase())
}

// Decodifica las entidades HTML habituales y las numéricas; deja tal cual las desconocidas.
fn decode(s: &str) -> String {
    if !s.contains('&') {
        return s.to_owned();
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        let decoded = rest.find(';').filter(|&end| end <= 10).and_then(|end| {
            let entity = &rest[1..end];
            let c = match entity {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" => Some('\''),
                "nbsp" => Some(' '),
                _ => entity.strip_prefix('#').and_then(|num| {
                    let code = match num.strip_prefix(['x', 'X']) {
                        Some(hex) => u32::from_str_radix(hex, 16).ok(),
                        None => num.parse().ok(),
                    };
                    code.and_then(char::from_u32)
                }),
            };
            c.map(|c| (c, end))
        });
        match decoded {
            Some((c, end)) => {
                out.push(c);
                rest = &rest[end + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::html_to_text;

    #[test]
    fn blocks_lists_and_line_breaks_become_lines() {
        let html = "<h1>Title</h1><p>Hello <strong>Ana</strong>,<br>welcome.</p>\
                    <ul><li>One</li><li>Two</li></ul><p>Bye</p>";
        assert_eq!(
            html_to_text(html),
            "Title\n\nHello Ana,\nwelcome.\n\n- One\n- Two\n\nBye"
        );
    }

    #[test]
    fn whitespace_is_collapsed_outside_pre() {
        let html = "<p>  Lots   of\n\n   space  </p><pre>  keep\n    this</pre>";
        assert_eq!(html_to_text(html), "Lots of space\n\n  keep\n    this");
    }

    #[test]
    fn links_show_their_target_unless_it_is_the_label() {
        let html = r##"<p><a href="https://x.com/a?b=1&amp;c=2">Open</a></p>
                      <p><a href="https://x.com">https://x.com</a></p>
                      <p><a href="mailto:ana@example.com">ana@example.com</a></p>
                      <p><a href="mailto:ana@example.com">Write to Ana</a></p>
                      <p><a href="#top">Top</a></p>"##;
        assert_eq!(
            html_to_text(html),
            "Open (https://x.com/a?b=1&c=2)\n\nhttps://x.com\n\nana@example.com\n\n\
             Write to Ana (ana@example.com)\n\nTop"
        );
    }

    #[test]
    fn entities_are_decoded() {
        assert_eq!(
            html_to_text("<p>a &amp; b &lt;c&gt; &quot;d&quot; &#39;e&#x27; &copy;</p>"),
            "a & b <c> \"d\" 'e' &copy;"
        );
    }

    #[test]
    fn head_styles_scripts_and_comments_are_dropped() {
        let html = "<!DOCTYPE html><html><head><title>T</title><style>p{}</style></head>\
                    <body><!-- hidden --><script>x()</script><p>Visible</p></body></html>";
        assert_eq!(html_to_text(html), "Visible");
    }

    #[test]
    fn tables_and_images_keep_their_text() {
        let html = r#"<table><tr><td>A</td><td>B</td></tr><tr><td>C</td><td>D</td></tr></table>
                      <p><img src="cid:logo" alt="Logo"> <img src="x.png" alt=""></p>"#;
        assert_eq!(html_to_text(html), "A B\nC D\n\nLogo");
    }

    #[test]
    fn stray_less_than_is_text() {
        assert_eq!(html_to_text("<p>1 < 2</p>"), "1 < 2");
    }
}
