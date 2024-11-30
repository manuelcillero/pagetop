# 🔃 Dependencies

PageTop is developed in the [Rust programming language](https://www.rust-lang.org/) and stands on
the shoulders of giants by leveraging some of the most robust and renowned libraries (*crates*) from
the [Rust ecosystem](https://lib.rs), including:

* [Actix Web](https://actix.rs/) for web services and server management.
* [Tracing](https://github.com/tokio-rs/tracing) for diagnostics and structured logging.
* [Fluent templates](https://github.com/XAMPPRocky/fluent-templates), which integrate
  [Fluent](https://projectfluent.org/) for internationalization.
* Additional crates, which you can explore in the `Cargo.toml` files of PageTop and its packages.


# ⌨️ Code

PageTop includes code from [config-rs](https://crates.io/crates/config) (version
[0.11.0](https://github.com/mehcode/config-rs/tree/0.11.0)) by
[Ryan Leckey](https://crates.io/users/mehcode), chosen for its advantages in reading configuration
settings and delegating assignment to safe types, tailored to the specific needs of each package,
theme, or application.


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
