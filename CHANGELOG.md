<a name="v0.3.0"></a>
## v0.3.0 (2017-11-23)


#### Bug Fixes

* **examples:**  use `hyper-native-tls` for TLS ([d342be5c](https://github.com/indiv0/xkcd-rs/commit/d342be5cc1b7f13da9cf98c461fb87a4359e6ccf), closes [#18](https://github.com/indiv0/xkcd-rs/issues/18))

#### Improvements

*   switch to an async API ([aec9d494](https://github.com/indiv0/xkcd-rs/commit/aec9d4944e509471d09461a3f572b9a2f6782488))
*   update for latest serde ([d2c53e37](https://github.com/indiv0/xkcd-rs/commit/d2c53e376f5b89b2b71e435b3fc31bb12ada2821))
*   clean up tests ([348824ed](https://github.com/indiv0/xkcd-rs/commit/348824ede5f67a3cca0a8568551c0135e4380c57))
* **examples:**  "all" example output more verbose ([adc03ae2](https://github.com/indiv0/xkcd-rs/commit/adc03ae27f8a81b97841282b2e74c42b57d6b44c))



<a name="v0.2.2"></a>
## v0.2.2 (2017-02-16)


#### Improvements

*   remove needless borrows ([07926ed2](https://github.com/indiv0/xkcd-rs/commit/07926ed2c595553177d574b61f2949a1d5d74f87))
*   update `serde`/`serde_json`/`serde_derive`, use the `?` operator instead of
  try. ([f8c20c0](https://github.com/indiv0/xkcd-rs/commit/f8c20c0efbfa6341578d63fb1b6e2296cf719032))



<a name="v0.2.1"></a>
## v0.2.1 (2016-12-01)


#### Documentation

* **XkcdResponse:**  fix doc string for the `link` field. ([356ac25e](https://github.com/indiv0/xkcd-rs/commit/356ac25ee18a2a40aa2547c5bbef04cd258392dd))



<a name="v0.2.0"></a>
## v0.2.0 (2016-06-25)


#### Bug Fixes

* **XkcdResponse:**  fix `link` field deserialization ([825017bf](https://github.com/indiv0/xkcd-rs/commit/825017bfc12c07e4bfff94eafd754507442ee21c), breaks [#](https://github.com/indiv0/xkcd-rs/issues/))

#### Improvements

* **XkcdResponse:**  change `img` field type from `String` to `Url` ([0c2f2299](https://github.com/indiv0/xkcd-rs/commit/0c2f22994cbe714902334959bcb0be200ecd6d69), breaks [#](https://github.com/indiv0/xkcd-rs/issues/))

#### Breaking Changes

* **XkcdResponse:**
  *  change `img` field type from `String` to `Url` ([0c2f2299](https://github.com/indiv0/xkcd-rs/commit/0c2f22994cbe714902334959bcb0be200ecd6d69), breaks [#](https://github.com/indiv0/xkcd-rs/issues/))
  *  fix `link` field deserialization ([825017bf](https://github.com/indiv0/xkcd-rs/commit/825017bfc12c07e4bfff94eafd754507442ee21c), breaks [#](https://github.com/indiv0/xkcd-rs/issues/))



<a name="v0.1.0"></a>
## v0.1.0 (2016-06-22)


#### Features

*   initial functionality ([01b0962d](https://github.com/indiv0/xkcd-rs/commit/01b0962d3ef035e40f148a5502d85b7f44a5c197))
* **examples:**  add examples ([c257d38e](https://github.com/indiv0/xkcd-rs/commit/c257d38e53972ea4a69b7433fad1f2ab4e8d5d2b))

#### Documentation

* **README:**  update `README.md` ([bc8467bc](https://github.com/indiv0/xkcd-rs/commit/bc8467bcd154104f48a2426caead3b765cffebc5))
