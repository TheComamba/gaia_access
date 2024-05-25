# GaiaAccess

A simple, type-safe crate to access data from the [Gaia ESA Archive](https://gea.esac.esa.int/archive/), which contains data on hundreds of thousands of stars.

## Usage

To keep the code that needs to be compiled at a manageable amount, every data table is hidden behind a feature flag.

See this project's `Cargo.toml` for a list of all features.

In your `Cargo.toml`, you need to enable the tables you want to access, e.g.:

```toml
[dependencies]
gaia_access = { version = "0.1.0", features = ["gaiadr3_gaia_source"] }
```

There are cumulative feature flags available for metatbles. If you want to use all or most of the gaiadr3 tables, you could depend on this crate via:

```toml
[dependencies]
gaia_access = { version = "0.1.0", features = ["gaiadr3"] }
```

The query in your code is created via a builder pattern.

```rust
use gaia_access::{
    condition::GaiaCondition,
    data::gaiadr3::{
        gaia_source::{gaia_source, Col},
        gaiadr3,
    },
    query::GaiaQueryBuilder,
    result::{get_float, get_string, GaiaCellData, GaiaResult},
};
...
let query_result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
        .select(vec![
            Col::designation,
            Col::ecl_lon,
            Col::ecl_lat,
            Col::phot_g_mean_mag,
            Col::teff_gspphot,
        ])
        .where_clause(GaiaCondition::LessThan(
            Col::phot_g_mean_mag,
            magnitude_threshold,
        ))
        .do_query()
```

## License

This software is distributed under the [MIT](https://choosealicense.com/licenses/mit/) license. In a nutshell this means that all code is made public, and you are free to use it without any charge.

See the [Gaia Data License](https://gaia.aip.de/cms/credit/) for the license of the data itself.
