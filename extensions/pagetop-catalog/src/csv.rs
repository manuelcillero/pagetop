//! Archivos CSV que se abren y se guardan con una hoja de cálculo.
//!
//! [`encode()`] escribe una tabla con la marca de orden de bytes de UTF-8 (sin ella, Excel en
//! Windows lee el archivo con la página de códigos local y estropea los acentos), el separador
//! indicado y saltos de línea `\r\n` (RFC 4180). Los campos que empiezan por `=`, `+`, `-`, `@`, un
//! tabulador o un retorno de carro llevan delante un apóstrofo, para que la hoja de cálculo no los
//! tome por una fórmula (alguien podría haber guardado `=HYPERLINK(...)` en un nombre).
//!
//! [`decode()`] lee lo mismo de vuelta: detecta el separador (`,` o `;`) en la cabecera, admite
//! campos entre comillas con separadores, comillas dobladas y saltos de línea, ignora la marca de
//! orden de bytes y las líneas vacías, y quita el apóstrofo que añade `encode()`.
//!
//! ```rust
//! use pagetop_catalog::csv;
//!
//! let rows = vec![vec!["code".to_string(), "name".to_string()], vec!["A".into(), "x; y".into()]];
//! let bytes = csv::encode(&rows, ';');
//! assert_eq!(csv::decode(&String::from_utf8(bytes).unwrap()).unwrap(), rows);
//! ```

const BOM: &str = "\u{FEFF}";
const EOL: &str = "\r\n";

// Caracteres con los que una hoja de cálculo empieza una fórmula.
fn looks_like_formula(field: &str) -> bool {
    field.starts_with(['=', '+', '-', '@', '\t', '\r'])
}

/// Codifica las filas (la primera, normalmente, la cabecera) con el separador `separator`.
pub fn encode(rows: &[Vec<String>], separator: char) -> Vec<u8> {
    let mut out = String::from(BOM);
    for row in rows {
        for (i, field) in row.iter().enumerate() {
            if i > 0 {
                out.push(separator);
            }
            let guarded = looks_like_formula(field);
            let quoted = field.contains([separator, '"', '\n', '\r']);
            if quoted {
                out.push('"');
            }
            if guarded {
                out.push('\'');
            }
            out.push_str(&field.replace('"', "\"\""));
            if quoted {
                out.push('"');
            }
        }
        out.push_str(EOL);
    }
    out.into_bytes()
}

/// Error al leer un CSV: unas comillas sin cerrar en la línea indicada.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("unclosed quotes in the CSV, starting at line {0}")]
pub struct DecodeError(pub usize);

/// Lee las filas de `text`, con el separador de su primera línea: `;` si la tiene, si no `,`.
pub fn decode(text: &str) -> Result<Vec<Vec<String>>, DecodeError> {
    let text = text.strip_prefix(BOM).unwrap_or(text);
    let first_line = text.lines().next().unwrap_or_default();
    let separator = if first_line.contains(';') { ';' } else { ',' };

    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut line = 1;
    let mut quote_line = 1;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if in_quotes && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' if in_quotes => in_quotes = false,
            '"' if field.is_empty() => {
                in_quotes = true;
                quote_line = line;
            }
            '\n' if !in_quotes => {
                line += 1;
                end_row(&mut rows, &mut row, &mut field);
            }
            '\r' if !in_quotes && chars.peek() == Some(&'\n') => {}
            c if c == separator && !in_quotes => row.push(unguard(std::mem::take(&mut field))),
            c => {
                if c == '\n' {
                    line += 1;
                }
                field.push(c);
            }
        }
    }
    if in_quotes {
        return Err(DecodeError(quote_line));
    }
    end_row(&mut rows, &mut row, &mut field);
    Ok(rows)
}

fn end_row(rows: &mut Vec<Vec<String>>, row: &mut Vec<String>, field: &mut String) {
    row.push(unguard(std::mem::take(field)));
    let row = std::mem::take(row);
    if !(row.len() == 1 && row[0].is_empty()) {
        rows.push(row);
    }
}

// Quita el apóstrofo que `encode()` pone delante de lo que parece una fórmula.
fn unguard(field: String) -> String {
    match field.strip_prefix('\'') {
        Some(rest) if looks_like_formula(rest) => rest.to_owned(),
        _ => field,
    }
}

#[cfg(test)]
mod tests {
    use super::{DecodeError, decode, encode};

    fn rows(rows: &[&[&str]]) -> Vec<Vec<String>> {
        rows.iter()
            .map(|r| r.iter().map(|f| (*f).to_owned()).collect())
            .collect()
    }

    #[test]
    fn plain_fields_round_trip() {
        let data = rows(&[&["code", "name"], &["A", "Alpha"], &["B", ""]]);
        let text = String::from_utf8(encode(&data, ',')).unwrap();
        assert_eq!(text, "\u{FEFF}code,name\r\nA,Alpha\r\nB,\r\n");
        assert_eq!(decode(&text).unwrap(), data);
    }

    #[test]
    fn special_fields_are_quoted_and_round_trip() {
        let data = rows(&[
            &["code", "name"],
            &["A", "semi; colon"],
            &["B", "say \"hi\""],
            &["C", "two\nlines"],
        ]);
        for separator in [';', ','] {
            let text = String::from_utf8(encode(&data, separator)).unwrap();
            assert_eq!(decode(&text).unwrap(), data, "{separator}");
        }
    }

    #[test]
    fn formulas_are_neutralised_and_restored() {
        let data = rows(&[&["name"], &["=HYPERLINK(\"x\")"], &["-5"], &["'quoted"]]);
        let text = String::from_utf8(encode(&data, ';')).unwrap();
        assert!(text.contains("'=HYPERLINK"));
        assert!(text.contains("'-5"));
        assert_eq!(decode(&text).unwrap(), data);
    }

    #[test]
    fn the_separator_comes_from_the_header() {
        assert_eq!(
            decode("a;b\r\n1,5;2\r\n").unwrap(),
            rows(&[&["a", "b"], &["1,5", "2"]])
        );
        assert_eq!(
            decode("a,b\n1;5,2\n").unwrap(),
            rows(&[&["a", "b"], &["1;5", "2"]])
        );
    }

    #[test]
    fn blank_lines_are_skipped_and_unclosed_quotes_fail() {
        assert_eq!(decode("a\n\nb\n").unwrap(), rows(&[&["a"], &["b"]]));
        assert_eq!(decode("a\nb\n\"open\nmore"), Err(DecodeError(3)));
    }
}
