
import {PublicMessage as PMessageType} from '../../../@types/public_message';
import {User} from '../../../@types/user';
import { ServerInit,
        ServerConsumed,
        ServerProduced,
        ActiveSpeader,
        NewWebrtcTransport,
        ServerConsumerClosed,
    } from './media_server';

import {PrivateMessage,PublicMessage} from './chat';
import { DataProducer } from './data';

import { RtpCapabilities } from 'mediasoup-client/lib/RtpParameters';


interface JoinRoom {
    event:'join_room';
    users:User[];
    data_producers:DataProducer[],
    messages:PMessageType[];
    router_rtp_capabilities: RtpCapabilities;
}

export interface INewWebrtcTransport extends NewWebrtcTransport {
    event:"new_webrtc_transport"
}

export interface ConnectedConsumerTransport {
    event:"connected_consumer_transport";
    transport_id:string;
}

export interface ConnectedProducerTransport {
    event:"connected_producer_transport";
    transport_id:string;
}

export interface Consumed extends ServerConsumed {
    event:'consumed';
    id:string;
}

export interface AudioProduced extends ServerProduced {
    event:'audio_produced';
}
export interface VideoProduced extends ServerProduced {
    event:'video_produced';
}
export interface NewUser {
    event:'new_user';
    user:User;
}

export interface ConsumerClosed extends ServerConsumerClosed {
    event:'consumer_close'
}
export interface ActiveSpeaderDetected extends ActiveSpeader {
    event:'active_speaker'
}
interface UserDisconnect {
    event:'user_disconnect';
    user_id:number;
}

export type ServerToRoom = JoinRoom | ConnectedConsumerTransport | ConsumerClosed |
                        ConnectedProducerTransport | Consumed | INewWebrtcTransport |
                        AudioProduced | VideoProduced | PrivateMessage | PublicMessage |
                        NewUser | UserDisconnect | ActiveSpeaderDetected; 
