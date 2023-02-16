// Take two arrays and multiply element by element up to the smallest array and then return the value
fn dot(array1: &mut [f64], array2: &mut [f64]) -> f64{
    let mut _sum: f64 = 0.0;
    let mut _size: usize;
    if (array1.len() == 0) || (array2.len() == 0) {
        return 0.0
    }
    if array1.len() > array2.len() {
        _size = array2.len()
        
    }
    else {
        _size = array1.len()
    }
    for _i in 0.._size {
        _sum += array1[_i]*array2[_i];
    }
    return _sum;

}

fn main() {
    let mut array1: [f64;2] = [1.0,3.0];
    let mut array2: [f64;3] = [1.2,3.4,1.0];
    let dot_result = dot(&mut array1,&mut array2);
    println!("{}", dot_result);
}