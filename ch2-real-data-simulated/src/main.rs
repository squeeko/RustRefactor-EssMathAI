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

    /*
    Finished dev [unoptimized + debuginfo] target(s) in 1.09s
     Running `target/debug/ch2-real-data-simulated`
[src/main.rs:24] df_hw_biased.as_ref().expect("No data available").with_row_count("Id", None) = Ok(
    shape: (500, 5)
    ┌─────┬────────┬────────┬────────┬───────┐
    │ Id  ┆ Gender ┆ Height ┆ Weight ┆ Index │
    │ --- ┆ ---    ┆ ---    ┆ ---    ┆ ---   │
    │ u32 ┆ str    ┆ i64    ┆ i64    ┆ i64   │
    ╞═════╪════════╪════════╪════════╪═══════╡
    │ 0   ┆ Male   ┆ 174    ┆ 96     ┆ 4     │
    │ 1   ┆ Male   ┆ 189    ┆ 87     ┆ 2     │
    │ 2   ┆ Female ┆ 185    ┆ 110    ┆ 4     │
    │ 3   ┆ Female ┆ 195    ┆ 104    ┆ 3     │
    │ ... ┆ ...    ┆ ...    ┆ ...    ┆ ...   │
    │ 496 ┆ Female ┆ 184    ┆ 121    ┆ 4     │
    │ 497 ┆ Female ┆ 141    ┆ 136    ┆ 5     │
    │ 498 ┆ Male   ┆ 150    ┆ 95     ┆ 5     │
    │ 499 ┆ Male   ┆ 173    ┆ 131    ┆ 5     │
    └─────┴────────┴────────┴────────┴───────┘,
)
    */
    

   
    
}
