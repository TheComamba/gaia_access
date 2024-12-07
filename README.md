# Gaia Access

A simple, type-safe crate to access data from the [Gaia ESA Archive](https://gea.esac.esa.int/archive/), which contains data on over one billion stars.

## Usage

To keep the code that needs to be compiled at a manageable amount, every data table is hidden behind a feature flag. They follow the naming convention `<schema>_<table>`. Check the [feature flags tab in the docs](https://docs.rs/crate/gaia_access/latest/features) for a list of all features.

In your `Cargo.toml`, you need to enable the tables you want to access, e.g.:

```toml
[dependencies]
gaia_access = { version = "0.1.0", features = ["gaiadr3_gaia_source"] }
```

There are cumulative feature flags available for schemas. If you want to use all or most of the gaiadr3 tables for example, you could depend on this crate via:

```toml
[dependencies]
gaia_access = { version = "0.1.0", features = ["gaiadr3"] }
```

The data query in your code is created via a builder pattern. The returned object contains a data Vec, which for all table rows contains a HashMap from column to value. The type of that value is a union of string, float and null. Knowing what type is expected for a column, it can be extracted.

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

// Get the designation and temperature of the first three stars that have a visible magnitude brighter than 5.
let magnitude_threshold = 5.0;
let query_result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
        .top(3)
        .select(vec![
            Col::designation,
            Col::teff_gspphot,
        ])
        .where_clause(GaiaCondition::LessThan(
            Col::phot_g_mean_mag,
            magnitude_threshold,
        ))
        .do_query()
        .unwrap();

// From the first entry of the returned data, get the temperature value.
let temperature: &GaiaCellData = query_result
    .data[0]
    .get(&Col::teff_gspphot)
    .unwrap();

// Convert the data to a type that can be used by the rest of rust. There is no guarantee that the data exists, hence the Option return type.
let temperature: Option<f64> = get_float(temperature);
```

## Contributing

Contributions are very welcome. I wrote this crate with my specific use case in mind, so it is far from feature complete, and I will probably only extend it if I learn that someone besides me is using it. Feature requests in form of issues are therefore just as welcome.

Support for new Gaia tables will probably not require any requests, because the implementation is fully automated, and I've created a weekly scheduled GitHub Workflow to check for updates.

## Development

The truly numerous types of schemas, tables and columns are handled automatically. The heart of this process is `generate_code.py`, with its wrapper script `generate_code.sh`. This scipt loads the XML of all available tables from the Gaia Archive and turns them into Rust code.

This means that manually modifying any code within `src/data` is futile. If you think something should be improved there, modify the Python script instead.

The same is true for the list of features in `Cargo.toml`.

Because the crate offers no default functionality, a mere `cargo test` will fail. You have to run `cargo test --all-features`, which is exactly what the `test.sh` script does.

## License

This software is distributed under the [MIT](https://choosealicense.com/licenses/mit/) license. In a nutshell this means that all code is made public, and you are free to use it without any charge.

See the [Gaia Data License](https://gaia.aip.de/cms/credit/) for the license of the data itself.
