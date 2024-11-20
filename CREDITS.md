# 🔃 Dependencies

PageTop is developed using the [Rust programming language](https://www.rust-lang.org/) and stands on
the shoulders of giants, leveraging some of the most stable and renowned libraries (*crates*) from
the [Rust ecosystem](https://lib.rs), including:

  * [Actix Web](https://actix.rs/) for web services and server management.
  * [Tracing](https://github.com/tokio-rs/tracing) for diagnostics and structured logging.
  * [Fluent templates](https://github.com/XAMPPRocky/fluent-templates), which integrate
    [Fluent](https://projectfluent.org/) for internationalization.
  * Additional crates, which you can explore in the `Cargo.toml` files of PageTop and its packages.

# ⌨️ Code

PageTop incorporates code from several well-regarded crates to enhance its functionality:

  * **[Config (v0.11.0)](https://github.com/mehcode/config-rs/tree/0.11.0)**: Includes code from
    [config-rs](https://crates.io/crates/config) by [Ryan Leckey](https://crates.io/users/mehcode),
    chosen for its advantages in reading configuration settings and delegating assignment to safe
    types, tailored to the specific needs of each package, theme, or application.

  * **[Maud (v0.25.0)](https://github.com/lambda-fairy/maud/tree/v0.25.0/maud)**: An adapted version
    of the excellent [maud](https://crates.io/crates/maud) crate by
    [Chris Wong](https://crates.io/users/lambda-fairy) is integrated, enabling its functionalities
    without requiring a direct dependency in the `Cargo.toml` files.

  * **SmartDefault (v0.7.1)**: The [SmartDefault](https://crates.io/crates/smart_default) crate by
    [Jane Doe](https://crates.io/users/jane-doe) has been embedded as `AutoDefault`, simplifying
    `Default` implementations and eliminating the need to explicitly reference `smart_default` in
    the `Cargo.toml` files.

# 🗚 FIGfonts

PageTop uses the [figlet-rs](https://crates.io/crates/figlet-rs) package by *yuanbohan* to display a
presentation banner in the terminal featuring the application's name in
[FIGlet](http://www.figlet.org) characters. The fonts included in `pagetop/src/app` are:

* [slant.flf](http://www.figlet.org/fontdb_example.cgi?font=slant.flf) by *Glenn Chappell*
* [small.flf](http://www.figlet.org/fontdb_example.cgi?font=small.flf) by *Glenn Chappell* (default)
* [speed.flf](http://www.figlet.org/fontdb_example.cgi?font=speed.flf) by *Claude Martins*
* [starwars.flf](http://www.figlet.org/fontdb_example.cgi?font=starwars.flf) by *Ryan Youck*

# 📰 Templates

The default welcome homepage design is inspired by a tutorial for creating a unique
[Neobrutalism](https://www.codewithfaraz.com/content/109/creating-a-unique-neobrutalism-portfolio-page-with-html-css-and-javascript)
portfolio page by [Faraz](https://www.codewithfaraz.com/).

# 🎨 Icon

"The Creature" smiling is a playful creation by [Webalys](https://www.iconfinder.com/webalys). It is
part of their [Nasty Icons](https://www.iconfinder.com/iconsets/nasty) collection, available on
[ICONFINDER](https://www.iconfinder.com).
