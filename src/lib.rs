// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! WIP

#![doc(html_root_url = "https://docs.rs/ruby-file-expand-path/0.1.0")]

mod sys;

#[cfg(test)]
mod tests {}
