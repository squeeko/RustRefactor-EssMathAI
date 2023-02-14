extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;
extern crate tuple_utils;

use csv::{ReaderBuilder};
use ndarray::{Array2};
use ndarray_csv::{Array2Reader};
use std::error::Error;
use std::fs::File;

use plotters::prelude::*;
use rand_distr::{Distribution, Normal};


const OUT_FILE_NAME: &'static str = "plotters-doc-data/normal-dist.png";

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("/Users/jallgood/RustRefactor-EssMathAI/data/weight-height.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let data_array: Array2<f64> = reader.deserialize_array2_dynamic()?;

    let height_data = data_array.column(0).to_owned();
    let weight_data = data_array.column(1).to_owned();



    
    let mut h_iter = height_data.into_iter();
    let mut w_iter = weight_data.into_iter();

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let sd = 0.13;
   
    let points: Vec<(f64, f64)> = {
        let norm_dist = Normal::new(0.5, sd).unwrap();
        h_iter.zip(w_iter).take(10000).collect()
    };



    



    Ok(())
}
