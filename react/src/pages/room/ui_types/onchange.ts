


import {PrivateMessage} from '../../../@types/private_message';
import {PublicMessage} from '../../../@types/public_message';

import {SelfManager,PeerManager} from '../managers';

interface ChangePublicMessage  {
    event:'PublicMessage';
    action:string;
    data:PublicMessage[];
}
interface ChangePrivateMessage  {
    event:'PrivateMessage';
    action:string;
    data:PrivateMessage[]
}
interface ChangePeerManager {
    event:'PeerManager';
    action: "JOIN" |"NEW_USER" | "NEW_CONSUMER" | "CLOSE_CONSUMER" | "NEW_PRIVATE_MESSAGE" | "DISCONNECT_USER";
    // TODO: create more interfaces 
    data:any;
}
interface ChangeRoomState {
    event:'CloseRoom',
    reason:string;
}
interface ChangeActiveSpeaker {
    event:'ActiveSpeaker',
    id:number
}
interface ChangeSelf {
    event:"SelfManager",
    action:string;
    data:SelfManager
}

export type OnChangeType = ChangeRoomState | ChangePublicMessage | ChangePrivateMessage  | ChangePeerManager | ChangeSelf | ChangeActiveSpeaker;