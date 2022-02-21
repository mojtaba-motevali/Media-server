import { ClientInit,ClientConnectConsumerTransport
    ,ClientConnectProducerTransport,ClientConsume,ClientProducerClose,
    ClientConsumerResume,ClientProduce,ClientConsumerPause,ClientProducerResume,ClientProducerPause } from "./media_client";

import {PrivateMessage,PublicMessage} from './chat';

interface ClientJoinRoom {
    event:'join_room',
    room_id:string,
    user_name:string,
    user_id:number
}

interface UserList {
    event:'user_list'
}

export type ClientToServer = PrivateMessage | PublicMessage | ClientJoinRoom | ClientConsumerPause | 
    UserList | ClientInit | ClientConnectConsumerTransport | ClientProducerResume | ClientProducerPause |
    ClientConnectProducerTransport | ClientConsume | ClientConsumerResume | ClientProduce | ClientProducerClose;
