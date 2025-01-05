mod solver;
mod parameters;
mod config;
use config::Config;
use solver::element::Element;

pub const NX:usize = 256;
pub const NV:usize = 100;
pub const LX:f64 = 1.;
//pub const LV:f64 = 1.;
pub const DT:f64 = 0.000005;
pub const DX:f64 = LX/(NX as f64);
//pub const DV:f64 = LV/(NV as f64);
pub const dxi:f64 = 1./DX ;
pub const epsi:f64 = 1.0e-20;
pub const T:f64 = 0.01;
pub const lambda:f64=0.01;

pub const field_bc_left:u32=0;
pub const field_bc_right:u32=1;
pub const n_save:u32 = 200;

fn main() {


    let config = Config::new(NX,NV,LX,DT,T,epsi,lambda,field_bc_left,field_bc_right,n_save);
    let mut solver = solver::Solver::new(config.clone());
    let electron = Element::new(String::from("electron"),30.,2,1, 1./50.,1,1,solver.get_file_h5(),config.clone());
    let ion = Element::new(String::from("ions"),4.,2, -1,1.,1,1,solver.get_file_h5(),config.clone());

    solver.add_element(electron);
    solver.add_element(ion);
    
    solver.init(); // init conditions
    solver.run(); // run the solver
    


}
