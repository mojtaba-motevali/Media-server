import { MediaKind, RtpCapabilities, RtpParameters } from 'mediasoup-client/lib/RtpParameters';
import { DtlsParameters, TransportOptions, Transport } from 'mediasoup-client/lib/Transport';
import { AppData } from './app_data';

export interface ServerInit {
}
export interface NewWebrtcTransport {
	transport_type:string;
    webrtc_transport: TransportOptions;
}
export interface ServerConnectedProducerTransport {
}

export interface ServerProduced {
	id: string;
}
export interface ServerConsumerClosed {
	id:string;
	user_id:number;
}

export interface ServerConnectedConsumerTransport {
}

export interface ServerConsumed {
	producer_id: string;
	kind: MediaKind;
	rtp_parameters: RtpParameters;
	user_id:number;
	app_data:AppData;
}

export interface ActiveSpeader {
	producer_id:number,
	volume:number,
	user_id:number
}