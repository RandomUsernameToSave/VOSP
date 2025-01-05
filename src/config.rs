
use std::usize;

pub struct Config {
    pub NX:usize,
    pub NV:usize,
    pub LX:f64,
    pub DX:f64,
    pub DT:f64,
    pub end_time:f64,
    pub dxi:f64 ,
    pub epsi:f64 ,
    pub lambda:f64,

    pub field_bc_left:u32,
    pub field_bc_right:u32,
    pub n_save:u32,
}

impl Config {
    pub fn new(NX:usize,NV:usize,LX:f64,DT:f64,end_time:f64,epsi:f64,lambda:f64,field_bc_left:u32,field_bc_right:u32,n_save:u32) -> Config {
        let DX = LX/(NX as f64);
        let dxi = 1./DX;
        Config {NX:NX,NV:NV,LX:LX,DX:DX,DT:DT,end_time:end_time,dxi:dxi,epsi:epsi,lambda:lambda,field_bc_left:field_bc_left,field_bc_right:field_bc_right,n_save:n_save}
    }
    pub fn new_init() -> Config {
        Config {NX:0,NV:0,LX:0.,DX:0.,DT:0.,end_time:0.,dxi:0.,epsi:0.,lambda:0.,field_bc_left:0,field_bc_right:0,n_save:0}
    }
    pub fn clone(&self) -> Config {
        Config {NX:self.NX,NV:self.NV,LX:self.LX,DX:self.DX,DT:self.DT,end_time:self.end_time,dxi:self.dxi,epsi:self.epsi,lambda:self.lambda,field_bc_left:self.field_bc_left,field_bc_right:self.field_bc_right,n_save:self.n_save}
    }
}
