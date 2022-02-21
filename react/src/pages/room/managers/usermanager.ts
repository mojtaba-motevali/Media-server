

import {EventEmitter} from 'events';
import {TURN_MIC_OFF,TURN_MIC_ON,TURN_CAMERA_OFF,TURN_CAMERA_ON,PRIVATE_MESSAGE,PUBLIC_MESSAGE,
     TURN_SCREENSHARE_OFF, TURN_SCREENSHARE_ON, PAUSE_MIC, UNPAUSE_MIC, PAUSE_CAMERA, UNPAUSE_CAMERA} from '../signal_types/room_media_actions';
import { SERVICE_TYPE } from '../signal_types/app_data';

export declare interface UserManager  {
    on(event:typeof TURN_MIC_OFF, listener: ()=>void ): this;
    on(event:typeof TURN_MIC_ON, listener: ()=>void): this;
    on(event:typeof TURN_CAMERA_OFF, listener: ()=>void): this;
    on(event:typeof TURN_CAMERA_ON, listener: ()=>void): this;
    on(event:typeof PAUSE_MIC, listener: (pId:any)=>void ): this;
    on(event:typeof UNPAUSE_MIC, listener: (pId:any)=>void): this;
    on(event:typeof PAUSE_CAMERA, listener: (pId:any)=>void): this;
    on(event:typeof UNPAUSE_CAMERA, listener: (pId:any)=>void): this;
    on(event:typeof TURN_SCREENSHARE_OFF, listener: ()=>void): this;
    on(event:typeof TURN_SCREENSHARE_ON, listener: ()=>void): this;
    on(event:typeof PRIVATE_MESSAGE, listener: (messageObject:any)=>void ): this;
    on(event:typeof PUBLIC_MESSAGE, listener: (messageObject:any)=>void ): this;

    // on(event:'close_all', listener:Function): this;

}
export interface UserManagerProps {
    // producers of peer are consumer to me
    id:number,
    name:string,
}

export abstract class UserManager extends  EventEmitter {
    public id:number;
    public name:string;
    public muted:boolean;
    public videoPaused:boolean;
    public avatar:string | undefined;
    constructor(props:UserManagerProps) {
        super();
        const {id,name} = props;
        this.id = id;
        this.name = name;
        this.muted = true;
        this.videoPaused = true;
        this.avatar = undefined;
    }
    abstract pause(service_stype:SERVICE_TYPE):void;
    abstract resume(service_stype:SERVICE_TYPE):void;
}

