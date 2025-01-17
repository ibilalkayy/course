

// Function to calculate BMI
fn calculate_bmi(height: f32, weight: f32) -> f32 {
    // Calculate the square of the height
    let squared = height * height;
    // Calculate BMI
    let bmi = weight / squared;
    bmi
}

fn main() {
    // Example height
    let height: f32 = 1.85; 
    // Example weight
    let weight: f32 = 72.0; 

    // Call the BMI calculation function
    let bmi = calculate_bmi(height, weight);

// Print the BMI result with
println!("Your Body Mass Index (BMI) is: {}", bmi);

}

