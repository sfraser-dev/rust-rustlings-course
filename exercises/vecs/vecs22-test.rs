fn main() {
    let v_in: Vec<i32> = [1, 2, 3]
        .iter()
        .map(|e| 
            e * 2
        )
        .collect();
    println!("{:?}", v_in)
}
