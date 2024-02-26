mod group_by;
use group_by::group_by;
mod merge_sort;
use merge_sort::merge_sort;
mod group_by_native;
use group_by_native::group_by_native;
mod sort_native;
use sort_native::sort_native;
use polars::prelude::*;



fn  main(){
    let keys = ['A', 'B', 'C', 'A', 'A', 'B', 'C', 'C'];
    let val = [12, 4, 5, 7, 9, 6, 8, 6];
    let mut arr = [5, 9, 1, 3, 4, 6, 6, 3, 2, 19, -5, -89];
    let df: DataFrame = df!(
        "x" => 0..8,
        "y"=> &["A", "A", "A", "B", "B", "C", "X", "X"],
    ).expect("Should not fail");
    let vec_native = group_by_native(df);
    let vec = group_by(&keys, &val);
    println!("Clefs : {:?}", keys);
    println!("Valeurs : {:?}", val);
    println!("Group by sur les clefs : {:?}", vec);
    println!("Goup by sur les clefs : {:?}", vec_native);
    println!("Liste non triée : {:?}", arr);
    merge_sort(&mut arr);
    println!("Liste triée (merge_sort) : {:?}", arr);
    arr = [5, 9, 1, 3, 4, 6, 6, 3, 2, 19, -5, -89];
    sort_native(&mut arr);
    println!("Liste triée (native_sort) : {:?}", arr);
}