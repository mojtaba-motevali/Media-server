import { Device } from "mediasoup-client";
import { Transport, TransportOptions } from "mediasoup-client/lib/Transport";
import { ClientToServer } from "./signal_types/room_client";
import { ServerToRoom } from "./signal_types/room_server";
import { PeerManager, SelfManager } from "./managers/index";
import { OnChangeType } from "./ui_types/onchange";
import {
  TURN_CAMERA_OFF,
  TURN_CAMERA_ON,
  TURN_MIC_OFF,
  TURN_MIC_ON,
  PRIVATE_MESSAGE,
  PUBLIC_MESSAGE,
  TURN_SCREENSHARE_ON,
  TURN_SCREENSHARE_OFF,
  UNPAUSE_MIC,
  PAUSE_MIC,
  UNPAUSE_CAMERA,
  PAUSE_CAMERA,
} from "./signal_types/room_media_actions";
import { PrivateMessage } from "../../@types/private_message";
import { PublicMessage } from "../../@types/public_message";
import { SERVICE_TYPE } from "./signal_types/app_data";

interface SignalRoomProps {
  // dispatch:Dispatch,
  self: SelfManager;
  url: string;
  room_name: string;
  onChange: (a: OnChangeType) => void;
  peers: Map<number, PeerManager>;
  unSeenPrivateMessages: Map<number, number>;
  unSeenPublicMessages: { count: number };
}
const WEBCAM_SIMULCAST_ENCODINGS = [
  { scaleResolutionDownBy: 4, maxBitrate: 500000 },
  { scaleResolutionDownBy: 2, maxBitrate: 1000000 },
  { scaleResolutionDownBy: 1, maxBitrate: 5000000 },
];
const SCREEN_SHARING_SIMULCAST_ENCODINGS = [
  { dtx: true, maxBitrate: 1500000 },
  { dtx: true, maxBitrate: 6000000 },
];
export default class SignalRoom {
  /** props  */
  self: SelfManager;
  url: string;
  // dispatch:Dispatch;
  room_name: string;
  peers: Map<number, PeerManager>;
  onChange: (a: OnChangeType) => void;
  unSeenPrivateMessages: Map<number, number>;
  unSeenPublicMessages: { count: number };
  /** class properties  */
  socket: WebSocket | null;
  device: Device;
  events: Map<string, Function>;
  currentCTId: string;
  currentPTId: string;
  producerTransports: Map<string, Transport>;
  consumerTransports: Map<string, Transport>;
  numberOfSocketRetry: number;
  // receiveMediaStream: MediaStream | undefined;

  constructor(props: SignalRoomProps) {
    this.socket = null;
    this.room_name = props.room_name;
    this.self = props.self;
    this.peers = props.peers;
    this.numberOfSocketRetry = 3;
    this.producerTransports = new Map();
    this.consumerTransports = new Map();
    this.unSeenPrivateMessages = props.unSeenPrivateMessages;
    this.unSeenPublicMessages = props.unSeenPublicMessages;
    this.currentCTId = "";
    this.currentPTId = "";
    // this.dispatch = props.dispatch;
    this.url = props.url;
    this.device = new Device();
    this.events = new Map();
    this.onChange = props.onChange;
    // this.receiveMediaStream = undefined;
    this.self.on(UNPAUSE_MIC, (pid: string) => {
      this.send_message({
        event: "producer_resume",
        user_id: this.self.id,
        id: pid,
      });
    });
    this.self.on(PAUSE_MIC, (pid: string) => {
      this.send_message({
        event: "producer_pause",
        user_id: this.self.id,
        id: pid,
      });
    });
    this.self.on(UNPAUSE_CAMERA, (pid: string) => {
      this.send_message({
        event: "producer_resume",
        user_id: this.self.id,
        id: pid,
      });
    });
    this.self.on(PAUSE_CAMERA, (pid: string) => {
      this.send_message({
        event: "producer_pause",
        user_id: this.self.id,
        id: pid,
      });
    });
    this.self.on(TURN_MIC_ON, async () => {
      await this.produce(SERVICE_TYPE.VOICE, { audio: true }, null);
    });
    this.self.on(TURN_MIC_OFF, async () => {
      this.removeProducer(SERVICE_TYPE.VOICE);
    });
    this.self.on(TURN_CAMERA_ON, async () => {
      await this.produce(
        SERVICE_TYPE.CAMERA,
        {
          video: {
            aspectRatio: {
              exact: 4 / 3,
            },
            width: {
              ideal: 1280,
            },
            height: {
              ideal: 720,
            },
            frameRate: {
              ideal: 60,
            },
          },
        },
        WEBCAM_SIMULCAST_ENCODINGS
      );
    });
    this.self.on(TURN_CAMERA_OFF, async () => {
      this.removeProducer(SERVICE_TYPE.CAMERA);
    });
    this.self.on(TURN_SCREENSHARE_ON, async () => {
      await this.produce(
        SERVICE_TYPE.SCREENSHARE,
        {
          video: {
            sampleRate: 4 / 3,
            width: {
              ideal: 1280,
            },
            height: {
              ideal: 720,
            },
            frameRate: {
              ideal: 60,
            },
          },
        },
        SCREEN_SHARING_SIMULCAST_ENCODINGS
      );
    });
    this.self.on(TURN_SCREENSHARE_OFF, async () => {
      this.removeProducer(SERVICE_TYPE.SCREENSHARE);
    });
    this.onChange({
      event: "SelfManager",
      data: this.self,
      action: "",
    });
  }
  removeProducer(service_type: SERVICE_TYPE) {
    const producerId = this.self.removeProducerByType(service_type);
    if (producerId) {
      this.onChange({
        event: "SelfManager",
        data: this.self,
        action: "",
      });
      this.send_message({
        event: "producer_close",
        user_id: this.self.id,
        id: producerId,
      });
    }
  }
  async produce(
    service_type: SERVICE_TYPE,
    constraints: MediaStreamConstraints,
    encodings: object[] | null
  ) {
    const producerTransport = this.producerTransports.get(this.currentPTId);
    if (producerTransport) {
      const mediaStream =
        service_type === SERVICE_TYPE.SCREENSHARE
          ? // @ts-ignore
            await navigator.mediaDevices.getDisplayMedia({ ...constraints })
          : await navigator.mediaDevices.getUserMedia({ ...constraints });
      const params = encodings
        ? {
            track: mediaStream.getTracks()[0],
            appData: { service_type },
            encodings,
          }
        : { track: mediaStream.getTracks()[0], appData: { service_type } };
      const producer = await producerTransport.produce(params);
      if (producer) {
        this.self.addSelfProducer(producer);
        this.onChange({
          event: "SelfManager",
          data: this.self,
          action: "UPDATE",
        });
      }
    }
  }
  createRecvTransport = (transport: TransportOptions) => {
    const _transport = this.device.createRecvTransport(transport);
    _transport.on("connect", ({ dtlsParameters }, success) => {
      this.send_message({
        event: "connect_consumer_transport",
        user_id: this.self.id,
        transport_id: _transport.id,
        dtls_parameters: dtlsParameters,
      });
      this.events.set("connected_consumer_transport", () => {
        success();
        console.log("Consumer transport connected");
      });
    });
    this.currentCTId = _transport.id;
    this.consumerTransports.set(_transport.id, _transport);
  };
  createSendTransport = (transport: TransportOptions) => {
    const _producerTransport = this.device.createSendTransport(transport);
    _producerTransport.on("connect", ({ dtlsParameters }, success, err) => {
      try {
        console.log("producer connect sent");
        this.send_message({
          event: "connect_producer_transport",
          transport_id: _producerTransport.id,
          user_id: this.self.id,
          dtls_parameters: dtlsParameters,
        });
        this.events.set("connected_producer_transport", success);
      } catch (e) {
        console.log(e);
        err(e);
      }
    });
    _producerTransport.on(
      "produce",
      ({ kind, rtpParameters, appData }, success, err) => {
        try {
          this.send_message({
            event: "produce",
            kind,
            user_id: this.self.id,
            transport_id: _producerTransport.id,
            rtp_parameters: rtpParameters,
            service_type: appData.service_type,
          });
          this.events.set(`${kind}_produced`, ({ id }: { id: string }) => {
            success({ id });
          });
        } catch (e) {
          console.error(e);
          err(e);
        }
      }
    );
    this.producerTransports.set(_producerTransport.id, _producerTransport);
    this.currentPTId = _producerTransport.id;
  };
  send_message = (msg: ClientToServer) => {
    this.socket?.send(JSON.stringify(msg));
  };
  setPeerEvents = (peer: PeerManager) => {
    peer.on(PRIVATE_MESSAGE, (message: PrivateMessage) => {
      this.send_message({
        event: "private_chat",
        message: message.message,
        sender_id: message.sender_id,
        receiver_id: message.receiver_id,
        timestamp: message.timestamp.toISOString(),
      });
    });
    peer.on(TURN_SCREENSHARE_ON, (pid: string) => {
      this.send_message({
        event: "consumer_resume",
        user_id: this.self.id,
        id: pid,
      });
    });
    peer.on(TURN_SCREENSHARE_OFF, (pid: string) => {
      this.send_message({
        event: "consumer_pause",
        user_id: this.self.id,
        id: pid,
      });
    });
    peer.on(TURN_MIC_ON, (pid: string) => {
      this.send_message({
        event: "consumer_resume",
        user_id: this.self.id,
        id: pid,
      });
    });
    peer.on(TURN_MIC_OFF, (pid: string) => {
      this.send_message({
        event: "consumer_pause",
        user_id: this.self.id,
        id: pid,
      });
    });
    peer.on(TURN_CAMERA_ON, (pid: string) => {
      this.send_message({
        event: "consumer_resume",
        user_id: this.self.id,
        id: pid,
      });
    });
    peer.on(TURN_CAMERA_OFF, (pid: string) => {
      this.send_message({
        event: "consumer_pause",
        user_id: this.self.id,
        id: pid,
      });
    });
  };
  run = (): void => {
    this.socket = new WebSocket(this.url);
    this.socket.onopen = (ev: Event) => {
      console.log("Websocket connected");
      this.send_message({
        room_id: this.room_name,
        user_name: this.self.name,
        user_id: this.self.id,
        event: "join_room",
      });
    };
    this.socket.onclose = () => {
      this.numberOfSocketRetry += 1;
      if (this.numberOfSocketRetry > 3) {
        this.onChange({
          event: "CloseRoom",
          reason: "Something went wrong with server",
        });
        return;
      } else {
        setTimeout(() => {
          this.run();
        }, 1000);
      }
    };
    this.socket.onmessage = async (ev) => {
      let message: ServerToRoom = JSON.parse(ev.data);
      console.log(message);
      switch (message.event) {
        case "join_room":
          this.self.on(PUBLIC_MESSAGE, (message: PublicMessage) => {
            this.send_message({
              event: "broadcast_message",
              sender_id: message.sender_id,
              message: message.message,
              user_name: message.user_name,
              timestamp: message.timestamp.toISOString(),
            });
          });
          for (const user of message.users) {
            if (user.id !== this.self.id) {
              const peer = new PeerManager({ ...user });
              this.setPeerEvents(peer);
              this.unSeenPrivateMessages.set(peer.id, 0);
              this.peers.set(peer.id, peer);
            }
          }
          if (this.peers.size > 0) {
            this.onChange({
              event: "PeerManager",
              action: "JOIN",
              data: undefined,
            });
          }

          try {
            await this.device.load({
              routerRtpCapabilities: message.router_rtp_capabilities,
            });
            if (message.messages.length > 0) {
              this.onChange({
                event: "PublicMessage",
                action: "",
                data: message.messages,
              });
            }
            this.send_message({
              event: "set_rtp_capability",
              user_id: this.self.id,
              rtp_capabilities: this.device.rtpCapabilities,
            });
          } catch (e) {
            console.log("ERROR", e);
          }
          this.onChange({ event: "SelfManager", action: "", data: this.self });
          break;
        case "connected_producer_transport":
          {
            const func = this.events.get("connected_producer_transport");
            if (func) func();
            this.events.delete("connected_producer_transport");
          }
          break;
        case "connected_consumer_transport":
          {
            const func = this.events.get("connected_consumer_transport");
            if (func) func();
            this.events.delete("connected_consumer_transport");
          }
          break;
        case "video_produced":
          {
            const func = this.events.get("video_produced");
            if (func) func({ id: message.id });
            this.events.delete("video_produced");
          }
          break;
        case "audio_produced":
          {
            const func = this.events.get("audio_produced");
            if (func) func({ id: message.id });
            this.events.delete("audio_produced");
          }
          break;
        case "consumed":
          {
            const ct = this.consumerTransports.get(this.currentCTId);
            const consumer = await ct?.consume({
              id: message.id,
              kind: message.kind,
              producerId: message.producer_id,
              rtpParameters: message.rtp_parameters,
              appData: message.app_data,
            });
            const peer = this.peers.get(message.user_id);
            if (peer && consumer) {
              peer.setPeerProducer(consumer);
              peer.pause(message.app_data.service_type);
              this.onChange({
                event: "PeerManager",
                action: "NEW_CONSUMER",
                data: {
                  userId: peer.id,
                  type: consumer.kind,
                  sourceType: consumer.appData.service_type,
                },
              });
            }
          }
          break;
        case "consumer_close":
          {
            const peer = this.peers.get(message.user_id);
            if (peer) {
              const _removedConsumer = peer.removeConsumer(message.id);
              console.log("_removedConsumer", _removedConsumer);
              if (_removedConsumer) {
                this.onChange({
                  event: "PeerManager",
                  action: "CLOSE_CONSUMER",
                  data: {
                    userId: peer.id,
                    type: _removedConsumer.kind,
                    sourceType: _removedConsumer.appData.service_type,
                  },
                });
              }
            }
          }
          break;
        case "new_user":
          const user = message.user;
          const peerManager = new PeerManager({ ...user });
          this.setPeerEvents(peerManager);
          this.peers.set(user.id, peerManager);
          this.unSeenPrivateMessages.set(user.id, 0);
          this.onChange({
            event: "PeerManager",
            action: "NEW_USER",
            data: peerManager,
          });
          break;
        case "user_disconnect":
          const id = message.user_id;
          const peer = this.peers.get(id);
          if (peer) {
            this.peers.delete(id);
            this.onChange({
              event: "PeerManager",
              action: "DISCONNECT_USER",
              data: {
                id: peer.id,
                name: peer.name,
              },
            });
          }
          break;
        case "new_webrtc_transport":
          const transportType = message.transport_type;
          if (transportType === "consumer") {
            this.createRecvTransport(message.webrtc_transport);
          } else {
            this.createSendTransport(message.webrtc_transport);
          }
          break;
        case "active_speaker":
          this.onChange({ event: "ActiveSpeaker", id: message.user_id });
          break;
        case "broadcast_message":
          const broad_message: PublicMessage = {
            sender_id: message.sender_id,
            message: message.message,
            user_name: message.user_name,
            timestamp: new Date(message.timestamp),
          };
          this.unSeenPublicMessages.count += 1;
          this.onChange({
            event: "PublicMessage",
            action: "",
            data: [broad_message],
          });
          break;
        case "private_chat":
          const private_msg: PrivateMessage = {
            message: message.message,
            sender_id: message.sender_id,
            receiver_id: this.self.id,
            timestamp: new Date(message.timestamp),
          };
          const number = this.unSeenPrivateMessages.get(message.sender_id);
          if (typeof number !== "undefined")
            this.unSeenPrivateMessages.set(message.sender_id, number + 1);
          this.onChange({
            event: "PrivateMessage",
            action: "",
            data: [private_msg],
          });
          break;
      }
    };
  };
}
