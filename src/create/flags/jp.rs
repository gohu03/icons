pub mod JPFlags{
  const color:[&str;2] = ["#FFFFFF", "#FE0000"];

  pub fn rectangular(width: u8)->String{
    return format!("<svg width=\"{}\" viewBox=\"0 0 30 20\" xmlns=\"http://www.w3.org/2000/svg\"><rect width=\"30\" height=\"20\" fill=\"{}\" /><circle cx=\"15\" cy=\"10\" r=\"6\" fill=\"{}\" /></svg>", width, color[0], color[1]);
  }

  pub fn circular(width:u8)->String{
    let c:f64 = (width as f64)/2.0;
    let r:f64 = (( width*3 ) as f64)/10.0;
    return format!("<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\"><circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\"/><circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\"/></svg>", width, width, width, width, c, c, c, color[0], c, c, r, color[1]);
  }
}
