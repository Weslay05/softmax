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

    // Vector with all probabilities of vec
    let vec_probs:Vec<f32> = softmax(vec.clone(), constant);

    // Prints input and output vector
    let mut i:usize = 0;
    while i < vec.len() {
        print!("[{i}]{} -> ", vec[i]);
        print!("Softmax -> {} -> ", vec_probs[i]);
        // if softmax_result < 0.10 {print!("0")};
        print!("{:.1}%\n", vec_probs[i] * 100.0);
        i += 1;
    }
}

// Softmax Function with a Custom Constant besides e
fn softmax(ivector:Vec<f32>, iconstant:f32) -> Vec<f32> { // i stands for input
    // New Vector for Constant^value results | rvec stands for Results Vector
    let mut rvec:Vec<f32> = Vec::new(); 

    // Calculation loop for rvec
    let mut i:usize = 0;
    while i < ivector.len() {
        let res:f32 = f32::powf(iconstant, ivector[i]);
        rvec.push(res);
        i += 1;
    }

    // sum of rvec for the divisor
    let divisor:f32 = rvec.iter().copied().sum();

    // New vec for all probabilities
    let mut prob_vec:Vec<f32> = Vec::new();
    // Probability for each object in ivec
    let mut i:usize = 0;
    while i < ivector.len() {
        let res:f32 = rvec[i] / divisor;
        prob_vec.push(res);
        i += 1;
    }

    return prob_vec;
}