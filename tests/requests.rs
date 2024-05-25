#[cfg(feature = "gaiadr3_gaia_source")]
#[cfg(test)]
mod test {
    use gaia_access::{
        data::gaiadr3::{gaia_source::*, gaiadr3},
        query::GaiaQueryBuilder,
        result::{get_float, get_string},
    };

    #[test]
    fn request_a_string() {
        let col = Col::designation;
        let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
            .top(1)
            .select(vec![col])
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        assert!(result.data[0].contains_key(&col));
        assert!(get_string(result.data[0].get(&col).unwrap()).is_some());
    }

    #[test]
    fn request_a_float() {
        let col = Col::ecl_lon;
        let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
            .top(1)
            .select(vec![col])
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        assert!(result.data[0].contains_key(&col));
        assert!(get_float(result.data[0].get(&col).unwrap()).is_some());
    }
}
