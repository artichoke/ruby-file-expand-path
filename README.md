# ruby-file-expand-path

[![GitHub Actions](https://github.com/artichoke/ruby-file-expand-path/workflows/CI/badge.svg)](https://github.com/artichoke/ruby-file-expand-path/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/ruby-file-expand-path.svg)](https://crates.io/crates/ruby-file-expand-path)
[![API](https://docs.rs/ruby-file-expand-path/badge.svg)](https://docs.rs/ruby-file-expand-path)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/ruby-file-expand-path/ruby_file_expand_path/)

Implements file path expansion and normalization routines from MRI Ruby.

> A new function (`normalize` or `make_absolute` or something, bikeshed away)
> should be added that will turn a relative path into an absolute path without
> touching the filesystem. ([rust-lang/rust#59117][rust-59117]).

[rust-59117]: https://github.com/rust-lang/rust/issues/59117

This crate normalizes and absolutizes paths according to the logic in Ruby
v3.0.2 for [POSIX][mri-3.0.2-posix] and [Win32][mri-3.0.2-win32].

[mri-3.0.2-posix]: https://github.com/ruby/ruby/blob/v3_0_2/file.c#L3690-L4037
[mri-3.0.2-win32]:
  https://github.com/ruby/ruby/blob/v3_0_2/win32/file.c#L276-L616

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ruby-file-expand-path = "0.1.0"
```

Then normalize paths like:

```rust
assert_eq!(
    ruby_file_expand_path::expand("/home/artichoke/scripts/../run.sh"),
    Ok(b"/home/artichoke/run.sh")
);
```

## License

`ruby-file-expand-path` is licensed under the [MIT License](LICENSE) (c) Ryan
Lopopolo.

`ruby-file-expand-path` is derived from `ruby` @ [v3.0.2][mri-3.0.2]. `ruby` is
dual licensed under the [Ruby License or 2-clause BSD
License][mri-3.0.2-license] Copyright (c) Yukihiro Matsumoto \<matz@netlab.jp\>.
A copy of the Ruby License can be found in [`COPYING`](COPYING) in this
repository.

[mri-3.0.2]: https://github.com/ruby/ruby/tree/v3_0_2
[mri-3.0.2-license]: https://github.com/ruby/ruby/blob/v3_0_2/COPYING
