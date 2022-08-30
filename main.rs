// Marvish Chandra

use std::env::{args, Args}

fn wein_displacement_law(u,m,K,T) {
    let maxWavelength = (2898 * u * m * K) / T
    let calculatedWavelength = maxWavelength.parse::<f32>().unwrap();

    println!("Calculated Wavelength: {}", calculatedWavelength);
}



fn main() {
    if calculatedWavelength == 500:
    println!("Calculated Wavelength: {}, produces a green color.", calculatedWavelength)
    if calculatedWavelength == 11:
    println!("Calculated Wavelength: {}, produces a red color.", calculatedWavelength)
    else: println!("The calculated wavelength does not produce either a green or red color. Color is based on how big the wavelength is.")
}
  