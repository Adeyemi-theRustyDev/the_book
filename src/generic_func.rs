fn largest<T>(coll: &[T]) -> T {
    let mut largest = coll[0];
    for item in coll.iter(){
        if item > largest {
            largest = item;
        }
    }
    largest
}