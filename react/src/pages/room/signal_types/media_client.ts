


import { MediaKind, RtpCapabilities, RtpParameters } from 'mediasoup-client/lib/RtpParameters';
import { DtlsParameters } from 'mediasoup-client/lib/Transport';
import { DataProducer } from './data';



export interface ClientInit {
    event: 'set_rtp_capability';
    user_id:number;
    rtp_capabilities: RtpCapabilities;
}

export interface ClientConnectProducerTransport {
    event: 'connect_producer_transport';
    transport_id:string;
    user_id:number;
	dtls_parameters: DtlsParameters;
}

export interface ClientConnectConsumerTransport {
    event: 'connect_consumer_transport';
    transport_id:string;
    user_id:number;
    dtls_parameters: DtlsParameters;
}

export interface ClientProduce {
    event: 'produce';
    transport_id:string;
    user_id:number;
    kind: MediaKind;
    rtp_parameters: RtpParameters;
    service_type:string;
}

export interface ClientConsume {
    event: 'consume';
    user_id:number;
    transport_id: string,
    data_producer:DataProducer,
}
export interface ClientProducerResume {
    event: 'producer_resume';
    id: string;
    user_id:number;
}
export interface ClientProducerPause {
    event: 'producer_pause';
    id: string;
    user_id:number;
}
export interface ClientProducerClose {
    event: 'producer_close';
    id: string;
    user_id:number;
}
export interface ClientConsumerResume {
    event: 'consumer_resume';
    id: string;
    user_id:number;
}

export interface ClientConsumerPause {
    event: 'consumer_pause';
    id: string;
    user_id:number;
}

