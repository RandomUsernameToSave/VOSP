mod solver;
mod parameters;
mod config;
use solver::element::Element;



fn main() {
    // Some parameters here
    
    let mut solver = solver::Solver::new();
    let electron = Element::new(String::from("electron"),30.,2,1, 1./50.,1,1,solver.get_file_h5());
    let ion = Element::new(String::from("ions"),4.,2, -1,1.,1,1,solver.get_file_h5());

    solver.add_element(electron);
    solver.add_element(ion);
    
    solver.init(); // init conditions
    solver.run(); // run the solver
    


}
