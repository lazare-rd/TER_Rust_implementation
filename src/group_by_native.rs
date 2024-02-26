use polars::prelude::*;

pub fn group_by_native(df: DataFrame) -> Result<DataFrame, PolarsError>{
    return df.clone().lazy().group_by(["y"]).agg([col("*").sum().alias("sum")]).collect();
}