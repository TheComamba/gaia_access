// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the igsl_source_catalog_ids table.

use crate::traits::{Column, Table};

/// SourceId in original  catalogues
#[allow(non_camel_case_types)]
pub struct igsl_source_catalog_ids;

impl Table for igsl_source_catalog_ids {
    fn string(&self) -> String {
        "igsl_source_catalog_ids".to_string()
    }
}

/// The columns in the igsl_source_catalog_ids table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// The IDs in the  Gaia Ecliptic Pole Catalogue Version (GEPC 3.0) given as IAU standards:
    /// JHHMMSS.SS+DDMMSS.S.
    id_epc,
    /// These are the IDs as published in the various input catalogs converted
    /// to a big integer. For the GSC23 we have had to alter slightly the name,specifically:
    /// GSC2.3 objects have original names that are combination of region names and running number,
    /// e.g. S00000001 is the first object in the region S0000000, we have converted
    /// the region names to the ``official' numerical values in the htm level 6
    /// structure, hence the smallest is S0000000 = 32768 and N3333333 = 65535 is the
    /// highest.
    id_gsc23,
    /// If the source is a Hipparcos star this field contains the Hipparcos number.
    id_hip,
    /// These are the running number in the Gaia Initial Quasar Catalog - version 4 - updated September 2011 - GIQC\_4.
    id_lqrf,
    /// The OGLE ID is given by a combination of \\
    ///  A region of sky identifier (RI) equal to  10, 11, 12, 13, 14 corresponding
    /// to map names blg, car, cen, lmc, mus.\\
    /// The file running number ( NNN.N) \\
    /// The running number in the file (RN) \\
    /// The ID is defined as: long(string(RI)+string(NNN.N * 10000))+RN.
    id_ogle,
    /// These are the IDs as published in the PPMXL, the ipix     Identifier (Q3C ipix of the USNO-B 1.0 object) converted to a big integer.
    id_ppmxl,
    /// These are the IDs as published in the SDSS DR9 in  big integer format.  
    id_sdss,
    /// The Two Mass provided the names in a IAU standard fashion of
    /// 2MASS JHHMMSS.S +DDMMSS.S these were codeified as +HHMMSSSDDMMSSS.
    id_tmass,
    /// These are the IDs as published in Tycho2. In  TYCHO-2 objects were identified by
    /// 3 numbers (TYC1,TYC2 and TYC3) and we have  combined these into one complete
    /// number given by (TYC1*1000000.0d0)+(TYC2*10.0d0)+(TYC3*1.0d0)
    ///
    id_tycho,
    /// These are the IDs as published in the UCAC4 specfically a running star ID number.
    id_ucac,
    /// The data in the MDB will be described by means of a "Solution identifier" parameter. This will be a numeric field attached to each table row that can be used to unequivocally identify the version of all the subsystems that where used in the generation of the data as well as the input data used.Each DPC generating the data will have the freedom to choose the Solution identifier number, but they must ensure that given the Solution identifier they can provide detailed information about the "conditions" used to generate the data: versions of the software, version of the data used,...
    solution_id,
    /// The source_id column. (No further description available)
    source_id,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the igsl_source_catalog_ids table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::id_epc.to_string());
    col_strings.push(Col::id_gsc23.to_string());
    col_strings.push(Col::id_hip.to_string());
    col_strings.push(Col::id_lqrf.to_string());
    col_strings.push(Col::id_ogle.to_string());
    col_strings.push(Col::id_ppmxl.to_string());
    col_strings.push(Col::id_sdss.to_string());
    col_strings.push(Col::id_tmass.to_string());
    col_strings.push(Col::id_tycho.to_string());
    col_strings.push(Col::id_ucac.to_string());
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    map.insert(igsl_source_catalog_ids.string(), col_strings);
}
