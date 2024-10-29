pub mod MenuIcon{
  pub fn round(width:u8, height:u8, weight:u8, color:&str)->String{
    let rx:f64 = (weight as f64)/2.0;
    return format!("<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" fill=\"{}\" xmlns=\"http://www.w3.org/2000/svg\"><rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\"/><rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\"/><rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\"/></svg>", width, height, width, height, color, 0, width, weight, rx, ((height-weight) as f64)/2.0, width, weight, rx, height-weight, width, weight, rx);
  }
}