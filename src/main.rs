mod group_by;
use group_by::group_by;
mod merge_sort;
use merge_sort::merge_sort;

fn  main(){
    let keys = ['A', 'B', 'C', 'A', 'A', 'B', 'C', 'C'];
    let val = [12, 4, 5, 7, 9, 6, 8, 6];
    let mut arr = [5, 9, 1, 3, 4, 6, 6, 3, 2, 19, -5, -89];
    let vec = group_by(&keys, &val);
    println!("Clefs : {:?}", keys);
    println!("Valeurs : {:?}", val);
    println!("Group by sur les clefs : {:?}", vec);
    println!("Liste non triée : {:?}", arr);
    merge_sort(&mut arr);
    println!("Liste triée : {:?}", arr);
}