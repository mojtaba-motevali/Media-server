import { UserManager, UserManagerProps } from "./usermanager";

import { Consumer } from "mediasoup-client/lib/Consumer";
import {
  TURN_CAMERA_OFF,
  TURN_MIC_ON,
  TURN_MIC_OFF,
  TURN_CAMERA_ON,
  PRIVATE_MESSAGE,
  TURN_SCREENSHARE_OFF,
  TURN_SCREENSHARE_ON,
} from "../signal_types/room_media_actions";
import { PrivateMessage } from "../../../@types/private_message";
import { SERVICE_TYPE } from "../signal_types/app_data";

export default class PeerManager extends UserManager {
  consumers: Consumer[];
  constructor(props: UserManagerProps) {
    super(props);
    this.consumers = [];
  }
  cleanUp() {
    for (const consumer of this.consumers) {
      consumer.close();
    }
  }
  removeConsumer(id: string) {
    const _removedConsumer = this.consumers.find(
      (consumer) => consumer.id === id
    );
    if (_removedConsumer) {
      this.consumers = this.consumers.filter((consumer) => consumer.id !== id);
      return _removedConsumer;
    }
    return null;
  }
  getConsumerByType(service_type: SERVICE_TYPE): Consumer | null {
    for (const consumer of this.consumers) {
      if (consumer.appData.service_type === service_type) return consumer;
    }
    return null;
  }
  hasConsumerWithType(service_type: SERVICE_TYPE) {
    for (const consumer of this.consumers) {
      if (consumer.appData.service_type === service_type) return true;
    }
    return false;
  }
  setPeerProducer(prod: Consumer) {
    this.consumers.push(prod);
  }
  closeAll = (streamer: MediaStream) => {
    for (const consumer of this.consumers) {
      streamer.removeTrack(consumer.track);
    }
  };
  sendMessage = (message: string, sender_id: number): PrivateMessage => {
    const messageObject = {
      receiver_id: this.id,
      sender_id,
      message: message,
      timestamp: new Date(),
    };
    this.emit(PRIVATE_MESSAGE, messageObject);
    return messageObject;
  };
  /**
   *  Pause tranffic from client.
   *  this method Also informs server to pause transport
   */
  pause(service_type: SERVICE_TYPE): void {
    const consumer = this.getConsumerByType(service_type);
    if (consumer && !consumer.paused) {
      consumer.pause();
      switch (service_type) {
        case SERVICE_TYPE.VOICE:
          this.emit(TURN_MIC_OFF, consumer.id);
          break;
        case SERVICE_TYPE.CAMERA:
          this.emit(TURN_CAMERA_OFF, consumer.id);
          break;
        case SERVICE_TYPE.SCREENSHARE:
          this.emit(TURN_SCREENSHARE_OFF, consumer.id);
          break;
      }
    }
  }

  resume(service_type: SERVICE_TYPE): void {
    const consumer = this.getConsumerByType(service_type);
    if (consumer && consumer.paused) {
      consumer.resume();
      switch (service_type) {
        case SERVICE_TYPE.VOICE:
          this.emit(TURN_MIC_ON, consumer.id);
          break;
        case SERVICE_TYPE.CAMERA:
          this.emit(TURN_CAMERA_ON, consumer.id);
          break;
        case SERVICE_TYPE.SCREENSHARE:
          this.emit(TURN_SCREENSHARE_ON, consumer.id);
          break;
      }
    }
  }
}
