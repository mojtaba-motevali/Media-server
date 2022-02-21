import {useState,useEffect, MutableRefObject, Fragment} from 'react';
import SignalRoom from './signal';
import {PeerManager,SelfManager} from './managers/index';
import  {OnChangeType} from './ui_types/onchange';
import {MEDIA_SERVER_URL} from '../../constants';
import { serializePublicChat, serializePrivateChat } from '../../organisms/chat/serialize';
import { Activity } from '../../organisms/activity';
import { Grid } from '@material-ui/core';
import { useStyles } from './styles';
import { MessageCardProps } from '../../molecules/cards/message';
import { useReducer } from 'react';
import { VideoCall } from '../../organisms/video';
import { useRef } from 'react';
import { shouldReRenderContext } from '../../contexts';
import { ActiveParts } from './ui_types/active_parts';
import EventEmitter from 'events';
import { Navigation } from '../../organisms/navigation';
import { SERVICE_TYPE } from './signal_types/app_data';
import { NoFaceAuido } from '../../organisms/audio';
import { useSnackbar } from 'notistack';

const Room = (props:any) => {
    const {userContext,mobileContext,match,history} = props;
    const {name} = userContext;
    const {isMobile} = mobileContext;
    const {room_id} = match.params;
    const peers:MutableRefObject<Map<number,PeerManager>> = useRef(new Map());
    const privateMessages:MutableRefObject<Map<number,MessageCardProps[]>> = useRef(new Map());
    const publicMessages: MutableRefObject<MessageCardProps[]> = useRef([]);
    const unSeenPrivateMessages: MutableRefObject<Map<number,number>> = useRef(new Map());
    const unSeenPublicMessages: MutableRefObject<{count:number}> = useRef({count:0});
    const self = useRef(new SelfManager({id: new Date().getTime(),name})) 
    const shouldReRender = useRef(new EventEmitter());
    const {enqueueSnackbar} = useSnackbar();
    const [activeParts,setActiveParts] = useState([isMobile ? ActiveParts.ACTIVITY:ActiveParts.ACTIVITY,ActiveParts.VIDEO]);
    useEffect(()=>{
        const arr = isMobile ? [ActiveParts.ACTIVITY]:[ActiveParts.ACTIVITY,ActiveParts.VIDEO];
        setActiveParts(arr)
    },[isMobile])
    const [selectedVideos,setSelected] = useReducer((state:number[],{peerId,type}:{peerId:number,type?:string})=>{
        if (state.indexOf(peerId) >= 0 && typeof type === 'undefined'  ){
            return [...state.filter(pId => pId !== peerId)];
        }else if (state.length < 3 && state.indexOf(peerId) < 0){
            return [...state,peerId];
        }else {
            // does not trigger re-render
            return state;
        }
    },[]);    
    const cleanUpRoom = ()=> { 
        self.current.cleanUp();
        for(const peer of peers.current.values()){
            peer.cleanUp();
        }
        enqueueSnackbar('Disconnecting from room...', {variant:'error'});
    }
    const onChange = (message:OnChangeType) => {
        switch(message.event){
            case "PeerManager":
                switch(message.action){
                    case 'DISCONNECT_USER':
                        enqueueSnackbar(`${message.data.name} left!`,{variant:'info'});
                        break;
                    case 'NEW_USER':
                        enqueueSnackbar(`${message.data.name} joined!`,{variant:'info'});
                        break;
                    case 'CLOSE_CONSUMER':{
                        const peer = peers.current.get(message.data.userId);
                        if(peer){
                            enqueueSnackbar(`${peer.name} just stopped sharing ${
                                message.data.sourceType === SERVICE_TYPE.VOICE ? 'microphone':
                                message.data.sourceType === SERVICE_TYPE.CAMERA ? 'camera': 'screen'
                            }.`,{variant:'info'});
                            const shouldRemoveFromSelected = peer.hasConsumerWithType(SERVICE_TYPE.CAMERA)  ||
                             peer.hasConsumerWithType(SERVICE_TYPE.SCREENSHARE);
                            if(!shouldRemoveFromSelected ){
                                setSelected({peerId:peer.id});
                            }
                        }
                    }
                        break;   
                    case 'NEW_CONSUMER':{
                        const peer = peers.current.get(message.data.userId);
                        if(peer)
                            enqueueSnackbar(`${peer.name} just shared ${
                                message.data.sourceType === SERVICE_TYPE.VOICE ? 'microphone':
                                message.data.sourceType === SERVICE_TYPE.CAMERA ? 'camera': 'screen'
                            }.`,{variant:'info'});
                        if(message.data.type === 'video'){
                            if(selectedVideos.length < 3 && selectedVideos.indexOf(message.data.userId) < 0){
                                setSelected({peerId:message.data.userId,type:'video'});
                            }
                        }
                    }
                        break;              
                }
                shouldReRender.current.emit("PeerManager");
                break;
            case 'SelfManager': 
                shouldReRender.current.emit("SelfManager");
                break;
            case 'ActiveSpeaker':
                shouldReRender.current.emit("ActiveSpeaker",message.id);
                break;
            case 'PublicMessage':{
                if(message.data[0].sender_id !== self.current.id){
                    const peer = peers.current.get(message.data[0].sender_id);
                    enqueueSnackbar(`message received from ${peer?.name} in public chat.`, {variant:'info'});
                }
                serializePublicChat({publicMessages:message.data,publicMessagesCards:publicMessages.current,peers:peers.current,selfManager:self.current})
                shouldReRender.current.emit("PublicMessage");
            }
                break;
            case "PrivateMessage":{
                if(message.data[0].sender_id !== self.current.id){
                    const peer = peers.current.get(message.data[0].sender_id);
                    enqueueSnackbar(`message received from ${peer?.name}.`, {variant:'info'});
                }
                serializePrivateChat({privateChats:message.data,privateMessages:privateMessages.current,selfManager:self.current,peers:peers.current});
                shouldReRender.current.emit("PrivateMessage");
            }
                break;
            case 'CloseRoom':{
                cleanUpRoom();
                history.push("/");
            }
                break;
        }
    }; 
    useEffect(()=> {
        const signalRoom = new SignalRoom({
            self:self.current,peers:peers.current,
            unSeenPrivateMessages:unSeenPrivateMessages.current,
            unSeenPublicMessages:unSeenPublicMessages.current,
            room_name:room_id,
            onChange:onChange,
            url:`ws://${MEDIA_SERVER_URL}/ws/`
        });
        try {
            enqueueSnackbar(`Welcome to ${room_id}.`, {variant:'info'});
            signalRoom.run();
        }catch(e){
            console.log(e);
        }
        return () => {
            signalRoom.socket?.close()
            cleanUpRoom();
        }
    },[]);
    const classes = useStyles();
    const showActivity = activeParts.includes(ActiveParts.ACTIVITY);
    const renderInMobile = (parts:ActiveParts[]) => {
        if(parts.includes(ActiveParts.VIDEO)){
            return <VideoCall  peers={peers.current} selected={selectedVideos}
             self={self.current} />
        }else if (parts.includes(ActiveParts.ACTIVITY)){
            return  <Activity 
            unSeenPrivateMessages={unSeenPrivateMessages.current}
            unSeenPublicMessages={unSeenPublicMessages.current}
            selectedVideos={selectedVideos} setSelected={setSelected}  
            onChange={onChange} peers={peers.current} 
            self={self.current} publicMessages={publicMessages.current} 
            privateMessages={privateMessages.current} />;
        }
    }
    return (
        <shouldReRenderContext.Provider value={shouldReRender.current}>

            <div  className={classes.root} >
                <NoFaceAuido peers={peers.current}/>
                {
                    isMobile ? (
                        <Fragment>
                            <Navigation activePars={activeParts} setActiveParts={setActiveParts} />
                            <Grid className={classes.my_grid} container >
                                    <Grid   item xs={12}>
                                       {renderInMobile(activeParts)}
                                    </Grid>
                            </Grid>
                        </Fragment>
                    ):(
                        <Grid spacing={1} className={classes.my_grid} container >
                            <Grid  className={classes.sideBar} item xs={1}>
                                <Navigation activePars={activeParts} setActiveParts={setActiveParts} />
                            </Grid>
                            <Grid  item sm={showActivity ? 6:11} md={showActivity ? 7:11} lg={ showActivity ? 8:11} >
                                <VideoCall  peers={peers.current} selected={selectedVideos} self={self.current} />
                            </Grid>
                            {
                                showActivity &&  <Grid item sm={4} md={4} lg={3}>
                                    <Activity unSeenPrivateMessages={unSeenPrivateMessages.current}
                                    unSeenPublicMessages={unSeenPublicMessages.current}
                                    selectedVideos={selectedVideos} setSelected={setSelected}  
                                        onChange={onChange} peers={peers.current} 
                                        self={self.current} publicMessages={publicMessages.current} 
                                        privateMessages={privateMessages.current} />;
                                </Grid>
                            }
                        </Grid>
                    )
                }
        </div>
        </shouldReRenderContext.Provider>

    );
    
}

export default Room;