# dash2html — Publish your Dash snippets online

`dash2html` is a tool that generates a HTML file from your [Dash] snippets.

## Installation

`dash2html` is written in [Rust]. If you don't already have the Rust toolchain
installed visit the [Rust homepage][Rust] and click the Install button.

Once you have Rust installed run the following to install the `dash2html` tool:

    cargo install dash2html

Ensure `$HOME/.cargo/bin` is in your PATH.

## Usage

`dash2html` generates a HTML page that includes all snippets tagged with
`public` tag. Run it as follows:

    dash2html ~/Library/Application\ Support/Dash/library.dash > ~/Desktop/dash-snippets.html

Substitute the path to `library.dash` with the location of your snippets
library if you have moved it in the Dash preferences. If you're unsure where
your snippets library is located check the Snippets tab in the Dash
preferences.

[Dash]: https://kapeli.com/dash
[Rust]: https://www.rust-lang.org
