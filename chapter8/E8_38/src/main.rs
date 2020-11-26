use std::fmt::{self,Formatter,Display};
struct City{
    name:&'static str,
    lat:f32,
    lon:f32,
}
impl Display for City{
    fn fmt(&self,f:&mut Formatter)->fmt::Result{
        let lat_c=if self.lat >= 0.0 { 'N'} else{'s'};
        let lon_c=if self.lon>=0.0 {'E'}else {'W'};
        write!(f,"{}:{:.3}°{} {:.3}° {}",self.name,self.lat.abs(),lat_c,self.lon.abs(),lon_c)
    }
}
fn main() {
    let city=City{name:"Bei",lat:39.999,lon:-166.555};
    println!("{:?}",format!("{}",city));
}
