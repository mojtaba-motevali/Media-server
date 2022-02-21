import { UserManager,UserManagerProps } from "./usermanager";
import { Producer } from 'mediasoup-client/lib/Producer';

import {TURN_CAMERA_OFF,TURN_MIC_ON,TURN_MIC_OFF,TURN_CAMERA_ON, PUBLIC_MESSAGE, TURN_SCREENSHARE_OFF, TURN_SCREENSHARE_ON} from '../signal_types/room_media_actions';
import { PublicMessage } from "../../../@types/public_message";
import { SERVICE_TYPE } from "../signal_types/app_data";

export default class SelfManager extends UserManager {
    producers:Producer[];

    constructor(props:UserManagerProps) {
        super(props);
        this.producers = [];
    }
    cleanUp(){
        for(const producer of this.producers){
            producer.close();
        }
    }
    getProducers():Producer[]{
        return this.producers;
    }
    getSelfProducerById = (id:string) => {
        return this.producers.filter(p=> p.id === id)[0];
    }
    getSelfProducerByType = (service_type:SERVICE_TYPE):Producer| null => {
        for(const producer of this.producers){
            if(producer.appData.service_type === service_type)
                return producer;
        }
        return null;
    }
    removeProducerByType(service_type:SERVICE_TYPE):string|null {
        const producer = this.getSelfProducerByType(service_type);
        console.log("producer'",producer);
        if(producer){
            producer.track?.stop();
            producer.close();
            this.producers = this.producers.filter(p => p.id !== producer.id);
            return producer.id;
        }
        return null;
    }
    sendMessage = (message:string):PublicMessage =>  {
        const publicMessage:PublicMessage = {
            sender_id:this.id,
            user_name:this.name,
            message,timestamp:new Date()
        }
        this.emit(PUBLIC_MESSAGE,publicMessage);
        return publicMessage;
    }

      /**
     * @description adding producer to list
     * @param id Producer Object 
     * 
     */
    addSelfProducer = (producer:Producer) =>{
        this.producers.push(producer);
    }
     /**
     * @description Used to stop sending media to server , also informing server about it.
     * @param id producer id 
     * 
     */
    pause (service_type:SERVICE_TYPE):void  {
        const producer = this.getSelfProducerByType(service_type);
        if(producer && !producer.paused) {
            switch(service_type){
                case SERVICE_TYPE.VOICE:
                    this.emit(TURN_MIC_OFF,producer.id);
                    break;
                case SERVICE_TYPE.CAMERA:
                    this.emit(TURN_CAMERA_OFF,producer.id);
                    break;
                case SERVICE_TYPE.SCREENSHARE:
                    this.emit(TURN_SCREENSHARE_OFF,producer.id);
                    break;
            }
            producer.pause();
        }
    }
    /**
     * 
     * @description Used to start sending media to server, also informing server about it
     * @param id producer id 
     * 
     */
    resume (service_type:SERVICE_TYPE) {
        const producer = this.getSelfProducerByType(service_type);
        if(producer && producer.paused) {
            switch(service_type){
                case SERVICE_TYPE.VOICE:
                    this.emit(TURN_MIC_ON,producer.id);
                    break;
                case SERVICE_TYPE.CAMERA:
                    this.emit(TURN_CAMERA_ON,producer.id);
                    break;
                case SERVICE_TYPE.SCREENSHARE:
                    this.emit(TURN_SCREENSHARE_ON,producer.id);
                    break;
            }
            producer.resume();
        }
    }
}