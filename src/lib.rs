/*
 * Library to add various vector functions such as adding, subtracting,
 * finding the dot product, and more.
 * Written by Armand Rathgeb.
 * Started 2021/06/27
 */


pub mod vec_func {
    /*
     * Addition. The two vectors must be of the same size for this to work.
     */
    pub fn add(vec_1: Vec<Vec<i32>>, vec_2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut current_vec: (&Vec<i32>,&Vec<i32>);
        let mut return_vec: Vec<Vec<i32>> = Vec::new();

        for i in &mut vec_1.iter().zip(&mut vec_2.iter()) {
            let mut add_val: Vec<i32> = Vec::new();
            let (w,x) = i;
            current_vec = (w, x);
            for j in current_vec.0.iter().zip(current_vec.1) {
                let (y,z) = j;
                &add_val.push(*y + *z);
            }
            return_vec.push(add_val);
        }

        return_vec
    }

    //pub fn mult() -> Vec2d {
             
    //}

    //pub fn dot() -> Vec2d {

    //}

       
    //pub fn sub() -> Vec2d {

    //}

    //pub fn cross() -> Vec2d {

    //}
    
}
