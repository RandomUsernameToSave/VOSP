mod initialize;
mod operator;
mod moments_calculation;
use std::{fmt::format, ptr::null};

use crate::parameters::{NX,NV};
use hdf5::{File, Group};
use ndarray::Array2;

pub struct Element {
    element_name:String,
    element_grid:Vec<Vec<f64>>,
    lv:f64,
    dv:f64,
    pub z_charge:i32,
    pub mu:f64,
    init_cond:u32,
    lx_boundary:u32,
    ux_boundary:u32,
    group_h5:Group,
}

impl Element {
    pub fn new(element_name:String, lv:f64,init_cond:u32,z_charge:i32,mu:f64,lx_boundary:u32,ux_boundary:u32,file:File) -> Element {
        let element_grid = vec![vec![0.;NV];NX]; 
        let dv = 2.*lv/NV as f64;

        
        let group: Group = file.create_group(&element_name).unwrap();

        Element{element_name:element_name,element_grid:element_grid,lv:lv,dv:dv  , z_charge:z_charge,  mu:mu,   
            init_cond:init_cond,lx_boundary:lx_boundary,ux_boundary:ux_boundary,group_h5:group}
    }

    pub fn init(&mut self) {
        self.initialize_grid();
 
    }



    pub fn save_h5(&self,file:File, t:f64) {

        fn vec_to_array<T: Clone>(v: &Vec<Vec<T>>) -> Array2<T> {
            if v.is_empty() {
                return Array2::from_shape_vec((0, 0), Vec::new()).unwrap();
            }
            let nrows = v.len();
            let ncols = v[0].len();
            let mut data = Vec::with_capacity(nrows * ncols);
            for row in v {
                assert_eq!(row.len(), ncols);
                data.extend_from_slice(&row);
            }
            Array2::from_shape_vec((nrows, ncols), data).unwrap()
        }
        let data = vec_to_array(&self.element_grid);

        
        let dataset = self.group_h5.new_dataset::<f64>().shape([NX,NV]).create(t.to_string().as_str()).unwrap();
    
    
        dataset.write(&data);
        //println!("Saved successfully");
    }

}