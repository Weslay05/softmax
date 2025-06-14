fn main() {
    // Create a new vector | This is for the Input Vector of the softmax function
    let mut vec: Vec<f32> = Vec::new();
    let e:f32 = 2.7182818284590452; // 7 digits for fp32
    let constant:f32 = e;

    // Constant used in softmax function
    println!("Variable for Softmax used : {constant}");

    // Push Vector numbers
    vec.push(1.3);
    vec.push(5.1);
    vec.push(0.7);
    vec.push(1.1);
    vec.push(5.0);

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
    // Calculate the Entire Vector to a divisor for softmax
    let mut divisor:f32 = 0.0;
    let mut i:usize = 0;
    while i < vector.len() {
        divisor += f32::powf(constant, vector[i]);
        i += 1;
    }

    // Calculate and return the Final Probabilities
    let probability:f32 = f32::powf(constant, vector[index]) / divisor;
    return probability;
}