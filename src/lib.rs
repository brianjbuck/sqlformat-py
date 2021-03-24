//! This crate is a port of https://github.com/kufii/sql-formatter-plus
//! written in Rust. It is intended to be usable as a pure-Rust library
//! for formatting SQL queries.

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use sqlformat as sf;

#[pymodule]
fn sqlformat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(format, m)?)?;
    // m.add_function(wrap_pyfunction!(format_with_options, m)?)?;
    Ok(())
}

#[pyfunction]
fn format(query: &str) -> String {
    let options = sf::FormatOptions::default();
    return sf::format(query, &sf::QueryParams::None, options);
}

// #[pyfunction]
// fn format_with_options(query: &str, options: sm::FormatOptions) -> String {
//     return sm::format(query, &sm::QueryParams::None, options);
// }

// /// Formats whitespace in a SQL string to make it easier to read.
// /// Optionally replaces parameter placeholders with `params`.
// fn format_rs(query: &str, params: &QueryParams, options: FormatOptions) -> String {
//     let tokens = tokenizer::tokenize(query);
//     formatter::format(&tokens, params, options)
// }

// /// Options for controlling how the library formats SQL
// #[derive(Debug, Clone, Copy)]
// pub struct FormatOptions {
//     /// Controls the type and length of indentation to use
//     ///
//     /// Default: 2 spaces
//     pub indent: Indent,
//     /// When set, changes reserved keywords to ALL CAPS
//     ///
//     /// Default: false
//     pub uppercase: bool,
//     /// Controls the number of line breaks after a query
//     ///
//     /// Default: 1
//     pub lines_between_queries: u8,
// }

// impl Default for FormatOptions {
//     fn default() -> Self {
//         FormatOptions {
//             indent: Indent::Spaces(2),
//             uppercase: false,
//             lines_between_queries: 1,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Indent {
//     Spaces(u8),
//     Tabs,
// }

// #[derive(Debug, Clone)]
// pub enum QueryParams {
//     Named(Vec<(String, String)>),
//     Indexed(Vec<String>),
//     None,
// }

// impl Default for QueryParams {
//     fn default() -> Self {
//         QueryParams::None
//     }
// }
