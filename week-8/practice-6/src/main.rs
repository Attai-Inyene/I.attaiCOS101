fn main() {
    
    //crete two vectors
    let v = vec![1,2,3,4,5,6,7,8];
    let x = vec![5,6,7,8,9,10,11];

    //Use a for loop to add elements of the vector
    for index iin 0..6{
        let sum = v[index] + x[index];
        println!("{:?}", sum);
    }
}
