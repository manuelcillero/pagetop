# 🔃 Dependencies

PageTop is developed in the [Rust programming language](https://www.rust-lang.org/) and stands on
the shoulders of true giants, using some of the most stable and renowned libraries (*crates*) from
the [Rust ecosystem](https://lib.rs), such as:

* [Actix Web](https://actix.rs/) for web services and server management.
* [Tracing](https://github.com/tokio-rs/tracing) for the diagnostic system and structured logging.
* [Fluent templates](https://github.com/XAMPPRocky/fluent-templates) that incorporate
  [Fluent](https://projectfluent.org/) for project internationalization.
* Among others, which you can review in the PageTop
  [`Cargo.toml`](https://github.com/manuelcillero/pagetop/blob/main/Cargo.toml) file.


# ⌨️ Code

PageTop integrates code from various renowned crates to enhance functionality:

* [**Config (v0.11.0)**](https://github.com/mehcode/config-rs/tree/0.11.0): Includes code from
  [config-rs](https://crates.io/crates/config) by [Ryan Leckey](https://crates.io/users/mehcode),
  chosen for its advantages in reading configuration settings and delegating assignment to safe
  types, tailored to the specific needs of each package, theme, or application.

* [**Maud (v0.25.0)**](https://github.com/lambda-fairy/maud/tree/v0.25.0/maud): An adapted version
  of the excellent [maud](https://crates.io/crates/maud) crate by
  [Chris Wong](https://crates.io/users/lambda-fairy) is incorporated to leverage its functionalities without requiring a reference to `maud` in the `Cargo.toml` files.

* **SmartDefault (v0.7.1)**: Embedded [SmartDefault](https://crates.io/crates/smart_default) by
  [Jane Doe](https://crates.io/users/jane-doe) as `AutoDefault`to simplify the documentation of
  Default implementations and also removes the need to explicitly list `smart_default` in the
  `Cargo.toml` files.


# 🗚 FIGfonts

PageTop uses the [figlet-rs](https://crates.io/crates/figlet-rs) package by *yuanbohan* to display a
presentation banner in the terminal with the application's name using
[FIGlet](http://www.figlet.org) characters. The fonts included in `src/app` are:

* [slant.flf](http://www.figlet.org/fontdb_example.cgi?font=slant.flf) by *Glenn Chappell*
* [small.flf](http://www.figlet.org/fontdb_example.cgi?font=small.flf) by *Glenn Chappell* (default)
* [speed.flf](http://www.figlet.org/fontdb_example.cgi?font=speed.flf) by *Claude Martins*
* [starwars.flf](http://www.figlet.org/fontdb_example.cgi?font=starwars.flf) by *Ryan Youck*


# 📰 Templates

* The default welcome homepage design is based on the
  [Zinc](https://themewagon.com/themes/free-bootstrap-5-html5-business-website-template-zinc)
  template created by [inovatik](https://inovatik.com/) and distributed by
  [ThemeWagon](https://themewagon.com).


# 🎨 Icon

"The creature" smiling is a fun creation by [Webalys](https://www.iconfinder.com/webalys). It can be
found in their [Nasty Icons](https://www.iconfinder.com/iconsets/nasty) collection available on
[ICONFINDER](https://www.iconfinder.com).
