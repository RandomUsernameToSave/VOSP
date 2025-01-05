mod config {
    use std::usize;

    struct Config {
        NX:usize,
        NV:usize,
        LX:f64,
    // LV:f64 = 1.,

        DX:f64,
        DT:f64,
        end_time:f64,
    // DV:f64 = LV/(NV as f64),
        dxi:f64 ,
        epsi:f64 ,
        lambda:f64,

        field_bc_left:u32,
        field_bc_right:u32,
        n_save:u32,
    }

    impl Config {
        pub fn new(NX:usize,NV:usize,LX:f64,DT:f64,end_time:f64,epsi:f64,lambda:f64,field_bc_left:u32,field_bc_right:u32,n_save:u32) -> Config {
            let DX = LX/(NX as f64);
            let dxi = 1./DX;
            Config {NX:NX,NV:NV,LX:LX,DX:DX,DT:DT,end_time:end_time,dxi:dxi,epsi:epsi,lambda:lambda,field_bc_left:field_bc_left,field_bc_right:field_bc_right,n_save:n_save}
        }
    }
}