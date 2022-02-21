use crate::media::media_struct::DataProducer;
use actix::Message;
use mediasoup::audio_level_observer::AudioLevelObserver;
use mediasoup::consumer::{Consumer, ConsumerId};
use mediasoup::producer::{Producer, ProducerId};
use mediasoup::router::{Router, RouterId};

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewProducer {
    pub data_producer: DataProducer,
}

impl Clone for NewProducer {
    fn clone(&self) -> Self {
        Self {
            data_producer: self.data_producer.clone(),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EnteranceScale {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SaveProducer {
    pub producer: Producer,
    pub user_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SavePipeProducer {
    pub producer_id: ProducerId,
    pub router_id: RouterId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SaveConsumer {
    pub consumer: Consumer,
    pub user_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveProducer {
    pub producer_id: ProducerId,
    pub user_id: usize,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveConsumer {
    pub consumer_id: ConsumerId,
    pub user_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SaveRouter {
    pub router: Router,
    pub is_consumer: bool,
    pub is_current: bool,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct SaveAudioLevelObserver {
    pub router_id: RouterId,
    pub audio_level_observer: AudioLevelObserver,
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum InternalMessage {
    SaveAudioLevelObserver(SaveAudioLevelObserver),
    SavePipeProducer(SavePipeProducer),
    SaveProducer(SaveProducer),
    SaveConsumer(SaveConsumer),
    RemoveProducer(RemoveProducer),
    //RemoveConsumer(RemoveConsumer),
    SaveRouter(SaveRouter),
}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
pub struct ProduceScale {
    pub skip_id: usize,
    pub producer: Producer,
}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
pub struct SynchronizeProducers {
    pub router: Router,
}

impl Clone for SynchronizeProducers {
    fn clone(&self) -> Self {
        Self {
            router: self.router.clone(),
        }
    }
}
