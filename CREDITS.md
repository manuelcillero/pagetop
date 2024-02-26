# 🔃 Dependencies

PageTop is developed in the [Rust programming language](https://www.rust-lang.org/) and stands on
the shoulders of true giants, using some of the most stable and renowned libraries (*crates*) from
the [Rust ecosystem](https://lib.rs), such as:

* [Actix Web](https://actix.rs/) for web services and server management.
* [Tracing](https://github.com/tokio-rs/tracing) for the diagnostic system and structured logging.
* [Fluent templates](https://github.com/XAMPPRocky/fluent-templates) that incorporate
  [Fluent](https://projectfluent.org/) for project internationalization.
* [SeaORM](https://www.sea-ql.org/SeaORM/) which employs [SQLx](https://docs.rs/sqlx/latest/sqlx/)
  for database access and modeling.
* Among others, which you can review in the PageTop
  [`Cargo.toml`](https://github.com/manuelcillero/pagetop/blob/main/Cargo.toml) file.


# ⌨️ Code

* PageTop includes code from version [0.11.0](https://github.com/mehcode/config-rs/tree/0.11.0) of
  [config](https://crates.io/crates/config) by [Ryan Leckey](https://crates.io/users/mehcode). This
  inclusion provides advantages over its more modern versions for reading configuration settings and
  delegating their assignment to safe types according to the requirements of each package, theme, or
  application.

* PageTop incorporates an adapted version of the excellent crate
  [maud](https://crates.io/crates/maud) by [Chris Wong](https://crates.io/users/lambda-fairy)
  (version [0.25.0](https://github.com/lambda-fairy/maud/tree/v0.25.0/maud)), to add its
  functionalities without requiring a reference to `maud` in the `Cargo.toml` file of each project.

* PageTop utilizes the renowned crates [SQLx](https://github.com/launchbadge/sqlx) and
  [SeaQuery](https://github.com/SeaQL/sea-query) for database interactions. However, to restrict
  migrations to packages, a modified version of
  [SeaORM Migration](https://github.com/SeaQL/sea-orm/tree/master/sea-orm-migration) (version
  [0.12.8](https://github.com/SeaQL/sea-orm/tree/0.12.8/sea-orm-migration/src)) has been integrated
  into the code.


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
