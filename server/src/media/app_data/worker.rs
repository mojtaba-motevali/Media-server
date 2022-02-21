use super::app_traits::{AppDataOperation, WorkerAppDataOperation};

///
/// This struct is used to maintain number of consumers and number of routers that are existed in
/// this worker.
/// this typically is used for consumers
///
#[derive(Copy)]
pub struct ConsumerWorkerAppData {
    number_of_consumers: usize,
    number_of_routers: usize,
}

impl ConsumerWorkerAppData {
    pub fn new() -> Self {
        Self {
            number_of_consumers: 0,
            number_of_routers: 0,
        }
    }
}
impl Clone for ConsumerWorkerAppData {
    fn clone(&self) -> ConsumerWorkerAppData {
        Self {
            number_of_consumers: self.number_of_consumers,
            number_of_routers: self.number_of_routers,
        }
    }
}
impl WorkerAppDataOperation for ConsumerWorkerAppData {
    fn add_router(&mut self) {
        self.number_of_routers += 1;
    }
    fn subtract_router(&mut self) {
        if self.number_of_routers > 0 {
            self.number_of_routers -= 1;
        }
    }
    fn get_number_of_routers(&self) -> usize {
        self.number_of_routers
    }
}

impl AppDataOperation for ConsumerWorkerAppData {
    fn add(&mut self, number: usize) {
        self.number_of_consumers += number;
    }
    fn subtract(&mut self, number: usize) {
        if self.number_of_consumers > 0 {
            self.number_of_consumers -= number;
        }
    }
    fn get(&self) -> usize {
        return self.number_of_consumers;
    }
}
///
/// This struct is used to maintain number of consumers and number of routers that are existed in
/// this worker.
/// this typically is used for producers.
///
#[derive(Copy)]
pub struct ProducerWorkerAppData {
    number_of_consumers: usize,
    number_of_routers: usize,
}

impl Clone for ProducerWorkerAppData {
    fn clone(&self) -> ProducerWorkerAppData {
        Self {
            number_of_consumers: self.number_of_consumers.clone(),
            number_of_routers: self.number_of_routers,
        }
    }
}
impl WorkerAppDataOperation for ProducerWorkerAppData {
    fn add_router(&mut self) {
        self.number_of_routers += 1;
    }
    fn subtract_router(&mut self) {
        if self.number_of_routers > 0 {
            self.number_of_routers -= 1;
        }
    }
    fn get_number_of_routers(&self) -> usize {
        self.number_of_routers
    }
}

impl ProducerWorkerAppData {
    pub fn new() -> Self {
        Self {
            number_of_consumers: 0,
            number_of_routers: 0,
        }
    }
}
impl AppDataOperation for ProducerWorkerAppData {
    fn add(&mut self, number: usize) {
        self.number_of_consumers += number;
    }
    fn subtract(&mut self, number: usize) {
        if self.number_of_consumers > 0 {
            self.number_of_consumers -= number;
        }
    }
    fn get(&self) -> usize {
        return self.number_of_consumers;
    }
}
