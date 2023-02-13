// extern crate csv;
// extern crate ndarray;
// extern crate ndarray_csv;

// use csv::ReaderBuilder;
// use std::error::Error;
// use std::fs::File;
// use ndarray::{s, Array2};
// use ndarray_csv::Array2Reader;

use polars::prelude::CsvReader;
use polars_core::prelude::*;
use polars_io::prelude::*;
use polars::frame::DataFrame;



fn main() {
    
    let df_hw_biased: PolarsResult<DataFrame> = CsvReader::from_path("/Users/jallgood/RustRefactor-EssMathAI/data/height-weight-biased.csv").expect("Cannot load csv")
        .has_header(true)
        .finish();

    dbg!(df_hw_biased.as_ref().expect("No data available").with_row_count("Id", None));
    

   
    
}
