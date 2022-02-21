use super::app_traits::AppDataOperation;

use mediasoup::worker::WorkerId;
///
/// This struct is used to maintain number of consumers that are existed in this router.
/// also maintains worker pid that this router created in it.
/// this typically is used for consumers.
///
#[derive(Copy)]
pub struct ConsumerRouterAppData {
    pub number_of_consumers: usize,
    pub worker_pid: WorkerId,
}

impl Clone for ConsumerRouterAppData {
    fn clone(&self) -> Self {
        Self {
            number_of_consumers: self.number_of_consumers.clone(),
            worker_pid: self.worker_pid.clone(),
        }
    }
}

impl ConsumerRouterAppData {
    pub fn new(worker_pid: WorkerId) -> Self {
        Self {
            number_of_consumers: 0,
            worker_pid: worker_pid,
        }
    }
}
impl AppDataOperation for ConsumerRouterAppData {
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
/// This struct is used to maintain number of consumers that are existed in this router.
/// also maintains worker pid that this router created in it.
/// this typically is used for producers.
///
#[derive(Copy)]
pub struct ProducerRouterAppData {
    pub number_of_consumers: usize,
    pub worker_pid: WorkerId,
}
impl Clone for ProducerRouterAppData {
    fn clone(&self) -> Self {
        Self {
            number_of_consumers: self.number_of_consumers,
            worker_pid: self.worker_pid.clone(),
        }
    }
}
impl ProducerRouterAppData {
    pub fn new(worker_pid: WorkerId) -> Self {
        Self {
            number_of_consumers: 0,
            worker_pid: worker_pid,
        }
    }
}
impl AppDataOperation for ProducerRouterAppData {
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
