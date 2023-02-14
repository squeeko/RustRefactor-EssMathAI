use polars::{prelude::CsvReader};
use polars_core::prelude::*;
use polars_io::prelude::*;
use polars::frame::DataFrame;



fn main() {
    
    let df_hw_biased: PolarsResult<DataFrame> = CsvReader::from_path("/Users/jallgood/RustRefactor-EssMathAI/data/height-weight-biased.csv").expect("Cannot load csv")
        .has_header(true)
        .finish();

println!("Standard Deviation {} ", df_hw_biased.as_ref().expect("cannnot").mean());
println!("Mean {} ", df_hw_biased.as_ref().expect("No std found").std(1));
println!("Min {} ", df_hw_biased.as_ref().expect("No minimum value found").min());
println!("Max {} ", df_hw_biased.as_ref().expect("No maximum value found").max());
println!("Median {} ", df_hw_biased.as_ref().expect("No median value found").median());
println!("Head {} ", df_hw_biased.as_ref().expect("No head values found").head(Some(5)));
println!("Number of Rows -  {} ", df_hw_biased.as_ref().expect("No rows found").height());
println!("Number of Columns -  {} ", df_hw_biased.as_ref().expect("No maximum value found").width());



/*
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/ch2-real-data-simulated`
Standard Deviation shape: (1, 4)
┌────────┬─────────┬────────┬───────┐
│ Gender ┆ Height  ┆ Weight ┆ Index │
│ ---    ┆ ---     ┆ ---    ┆ ---   │
│ str    ┆ f64     ┆ f64    ┆ f64   │
╞════════╪═════════╪════════╪═══════╡
│ null   ┆ 169.944 ┆ 106.0  ┆ 3.748 │
└────────┴─────────┴────────┴───────┘
Mean shape: (1, 4)
┌────────┬───────────┬───────────┬──────────┐
│ Gender ┆ Height    ┆ Weight    ┆ Index    │
│ ---    ┆ ---       ┆ ---       ┆ ---      │
│ str    ┆ f64       ┆ f64       ┆ f64      │
╞════════╪═══════════╪═══════════╪══════════╡
│ null   ┆ 16.375261 ┆ 32.382607 ┆ 1.355053 │
└────────┴───────────┴───────────┴──────────┘
Min shape: (1, 4)
┌────────┬────────┬────────┬───────┐
│ Gender ┆ Height ┆ Weight ┆ Index │
│ ---    ┆ ---    ┆ ---    ┆ ---   │
│ str    ┆ i64    ┆ i64    ┆ i64   │
╞════════╪════════╪════════╪═══════╡
│ Female ┆ 140    ┆ 50     ┆ 0     │
└────────┴────────┴────────┴───────┘
Max shape: (1, 4)
┌────────┬────────┬────────┬───────┐
│ Gender ┆ Height ┆ Weight ┆ Index │
│ ---    ┆ ---    ┆ ---    ┆ ---   │
│ str    ┆ i64    ┆ i64    ┆ i64   │
╞════════╪════════╪════════╪═══════╡
│ Male   ┆ 199    ┆ 160    ┆ 5     │
└────────┴────────┴────────┴───────┘
Median shape: (1, 4)
┌────────┬────────┬────────┬───────┐
│ Gender ┆ Height ┆ Weight ┆ Index │
│ ---    ┆ ---    ┆ ---    ┆ ---   │
│ str    ┆ f64    ┆ f64    ┆ f64   │
╞════════╪════════╪════════╪═══════╡
│ null   ┆ 170.5  ┆ 106.0  ┆ 4.0   │
└────────┴────────┴────────┴───────┘
Head shape: (5, 4)
┌────────┬────────┬────────┬───────┐
│ Gender ┆ Height ┆ Weight ┆ Index │
│ ---    ┆ ---    ┆ ---    ┆ ---   │
│ str    ┆ i64    ┆ i64    ┆ i64   │
╞════════╪════════╪════════╪═══════╡
│ Male   ┆ 174    ┆ 96     ┆ 4     │
│ Male   ┆ 189    ┆ 87     ┆ 2     │
│ Female ┆ 185    ┆ 110    ┆ 4     │
│ Female ┆ 195    ┆ 104    ┆ 3     │
│ Male   ┆ 149    ┆ 61     ┆ 3     │
└────────┴────────┴────────┴───────┘
Number of Rows -  500
Number of Columns -  4
*/



    

    
}
