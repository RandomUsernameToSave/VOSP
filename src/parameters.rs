use hdf5::File;


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
// pub const mu:f64 = 100.;