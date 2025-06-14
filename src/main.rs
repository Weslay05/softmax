fn main() {
    // Create a new vector | This is for the Input Vector of the softmax function
    let mut vec: Vec<f32> = Vec::new();
    let e:f32 = 2.7182818284590452; // 7 digits for fp32
    let constant:f32 = e;

    // Constant used in softmax function
    println!("Variable for Softmax used : {constant}");

    // Push val array to vector
    let val:[f32; 4] = [1.3, 5.1, 0.7, 1.1];
    // Push Loop
    let mut i:usize = 0;
    while i < val.len() {
        vec.push(val[i]);
        i += 1;
    }
    /* // Push all ints from 1-5
    let mut i:f32 = 1.0;
    while i <= 5.0 {
        vec.push(i);
        i += 1.0;
    } */

    // Print Vector and Softmax Results
    let mut i:usize = 0;
    while i < vec.len() {
        let softmax_result:f32 = softmax(i, vec.clone(), constant);
        print!("[{i}]{} -> ", vec[i]);
        print!("Softmax -> {} -> ", softmax_result);
        // if softmax_result < 0.10 {print!("0")};
        print!("{:.1}%\n", softmax_result * 100.0);
        i += 1;
    }
}

// Softmax Function with a Custom Constant besides e
fn softmax(index:usize, vector:Vec<f32>, constant:f32) -> f32 {
    // Create new Vector with all values after Constant^value
    let mut constant_vec: Vec<f32> = Vec::new();
    let mut i:usize = 0;
    while i < vector.len() {
        let res:f32 = f32::powf(constant, vector[i]);
        constant_vec.push(res);
        i += 1;
    }

    // Calculate the divisor for softmax
    let divisor:f32 = constant_vec.iter().copied().sum();

    // Calculate and return the Final Probabilities
    let probability:f32 = f32::powf(constant, vector[index]) / divisor;
    return probability;
}