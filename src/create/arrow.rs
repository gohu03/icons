fn round_arrow(width: u8, height: u8, weight: u8, color:&str, degree: u8) -> String{
  let r:f64 = (weight as f64) / 2.0;
  let cx:f64 = (width as f64) / 2.0;
  let angle:f64 = ((((90 - degree) as f64)*std::f64::consts::PI)/180.0).tan();
  let py:f64 = -angle*(r-cx)+r;
  
  let points:[[f64;2];4] = [
      [cx, r],
      [cx, (height as f64) - r],
      [r, py],
      [(width as f64) - r, py]
  ];

  return format!("<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"round\"><path d=\"M{} {} V{} M{} {} L{} {} M{} {} L{} {}\"/></svg>", 
    width, height, width, height, color, weight, 
    cx, r, 
    (height as f64) - r, 
    cx, r, 
    r, py, 
    cx, r, 
    (width as f64) - r, py);
}