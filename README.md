# rckive-genpdf-rs

> This is a fork of [genpdf-rs][upstream] by [RCKIVE](rckive.com).

A user-friendly PDF generator written in pure Rust.

[Documentation](https://docs.rs/rckive-genpdf)

`genpdf` is a high-level PDF generator built on top of [`printpdf`][] and
[`rusttype`][]. It takes care of the page layout and text alignment and
renders a document tree into a PDF document. All of its dependencies are
written in Rust, so you don’t need any pre-installed libraries or tools.

[`printpdf`]: https://lib.rs/crates/printpdf
[`rusttype`]: https://lib.rs/crates/rusttype

<!-- Keep in sync with src/lib.rs -->

```rust
// Load a font from the file system
let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)
    .expect("Failed to load font family");
// Create a document and set the default font family
let mut doc = genpdf::Document::new(font_family);
// Change the default settings
doc.set_title("Demo document");
// Customize the pages
let mut decorator = genpdf::SimplePageDecorator::new();
decorator.set_margins(10);
doc.set_page_decorator(decorator);
// Add one or more elements
doc.push(genpdf::elements::Paragraph::new("This is a demo document."));
// Render the document and write it to a file
doc.render_to_file("output.pdf").expect("Failed to write PDF file");
```

For a complete example with all supported elements, see the
[`examples/demo.rs`][] file.

<!-- For more information, see the [API documentation](https://docs.rs/rckive-genpdf). -->

[upstream]: https://git.sr.ht/~ireas/genpdf-rs

## Features

- PDF generation in pure Rust
- Text rendering with support for setting the font family, style and size as
  well as the text color and text effects (bold or italic) and with kerning
- Text wrapping at word boundaries and optional hyphenation
- Layout of elements sequentially or in tables
- Rudimentary support for shapes
- Page headers and custom page decorations
- Embedding images (scale, position, rotate).

## Cargo Features

This crate has the following Cargo features (deactivated per default):

- `images`: Adds support for embedding images using the [`image`][] crate.
- `hyphenation`: Adds support for hyphenation using the [`hyphenation`][] crate.

[`hyphenation`]: https://lib.rs/crates/hyphenation
[`image`]: https://lib.rs/crates/image

## Alternatives

- [`printpdf`][] is the low-level PDF library used by `genpdf`. It provides
  more control over the generated document, but you have to take care of all
  details like calculating the width and height of the rendered text, arranging
  the elements and distributing them on multiple pages.
- [`latex`][] generates LaTeX documents from Rust. It requires a LaTex
  installation to generate the PDF files. Also, escaping user input is a
  non-trivial problem and not supported by the crate.
- [`tectonic`][] is a TeX engine based on XeTeX. It is partly written in C and
  has some non-Rust dependencies.
- [`wkhtmltopdf`][] generates PDF documents from HTML using the `wkhtmltox`
  library. It requires a pre-installed library and does not support custom
  elements.

[`latex`]: https://lib.rs/crates/latex
[`tectonic`]: https://lib.rs/crates/tectonic
[`wkhtmltopdf`]: https://lib.rs/crates/wkhtmltopdf

## Minimum Supported Rust Version

This crate supports at least Rust 1.45.0 or later.

## Contributing

Contributions to this project are welcome! Please submit pull requests on
[Github][gh-pulls].

[~ireas/public-inbox@lists.sr.ht]: mailto:~ireas/public-inbox@lists.sr.ht
[archive]: https://lists.sr.ht/~ireas/public-inbox
[contributing guide]: https://man.sr.ht/~ireas/guides/contributing.md

If you are looking for a good starting point, have a look at the [issues with
the label “good first issue”][issues] in `genpdf-rs`’s issue tracker.

[issues]: https://todo.sr.ht/~ireas/genpdf-rs?search=label:%22good%20first%20issue%22%20status%3Aopen

## Contact

For bug reports or feature requests, please look at [the Github Issues board][gh-issues]
if your issue exists, and create a new one if it doesn't. For questions, check
out [Github Discussions][gh-discussions]

## License

This repository is licensed under EUPL-1.2 or later, except for code that is
unchanged from the upstream repository as described below.

This code was forked, find the original repository here:
https://git.sr.ht/~ireas/genpdf-rs. All code that has not been changed is
licensed under Apache-2.0 and MIT, and documentation under CC-0, as described in
the original repository README.

As of the time of this writing, no merges have been made from the original
repository since its forking. This means that the prime way to find pre-fork
code is using Git: `git checkout 38dd11e8`.

[gh-pulls]: https://github.com/RCKIVE/rckive-genpdf-rs/pulls
[gh-issues]: https://github.com/RCKIVE/rckive-genpdf-rs/issues
[gh-discussions]: https://github.com/RCKIVE/rckive-genpdf-rs/discussions
