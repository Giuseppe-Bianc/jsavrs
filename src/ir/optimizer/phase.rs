use crate::ir::Module;
pub trait Phase {
    fn name(&self) -> &'static str;
    fn run(&mut self, ir: &mut Module);
}

pub fn run_pipeline(ir: &mut Module, phases: Vec<Box<dyn Phase>>) {
    for mut phase in phases {
        println!("Running phase: {}", phase.name());
        phase.run(ir);
    }
}
