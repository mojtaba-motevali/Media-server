pub trait AppDataOperation {
    fn add(&mut self, number: usize);
    fn subtract(&mut self, number: usize);
    fn get(&self) -> usize;
}

pub trait WorkerAppDataOperation {
    fn add_router(&mut self);
    fn subtract_router(&mut self);
    fn get_number_of_routers(&self) -> usize;
}
