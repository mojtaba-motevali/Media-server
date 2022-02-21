import { createRef, useEffect } from 'react';
import { Consumer } from 'mediasoup-client/lib/Consumer';
import { PeerManager } from '../../../pages/room/managers';
import { useStyles } from './styles';
import { SERVICE_TYPE } from '../../../pages/room/signal_types/app_data';

interface PeerVideoProps {
    peer:PeerManager;
    isActiveSpeaker?:boolean;
    selectedVideos:number[];
}

export const PeerVideo = ({peer,isActiveSpeaker,selectedVideos}:PeerVideoProps) => {

    const classes = useStyles();
    const videoRef = createRef<HTMLVideoElement>();
    const divRef = createRef<HTMLDivElement>();
    const addStream = (element:HTMLVideoElement | null,consumer:Consumer) => {
        const mediaStream = new MediaStream();
        mediaStream.addTrack(consumer.track);
        if(element){
            element.srcObject = mediaStream;
            element.onloadedmetadata = async () => {
                    // const isPlaying = element.currentTime > 0 && !element.paused && !element.ended 
                    // && element.readyState > element.HAVE_CURRENT_DATA;
                    // if(!isPlaying)
                    await element.play();
            }
        }
    }
    useEffect(() => {
        setTimeout(()=>{
            const shareScreenConsumer = peer.getConsumerByType(SERVICE_TYPE.SCREENSHARE);
            const cameraConsumer = peer.getConsumerByType(SERVICE_TYPE.CAMERA);
            if(shareScreenConsumer && videoRef){
                peer.resume(SERVICE_TYPE.SCREENSHARE);
                addStream(videoRef.current,shareScreenConsumer)
            }else if (cameraConsumer && videoRef){
                peer.resume(SERVICE_TYPE.CAMERA);
                addStream(videoRef.current,cameraConsumer)
            }
        },100)
    },[addStream])
    useEffect(()=>{
        return () => {
            if((isActiveSpeaker && !selectedVideos.includes(peer.id) )|| !isActiveSpeaker){
                peer.pause(SERVICE_TYPE.CAMERA);
                peer.pause(SERVICE_TYPE.SCREENSHARE);
            }
        }
    },[])
    return (
        <div ref={divRef} className={classes.root} >
            <video controls={peer.hasConsumerWithType(SERVICE_TYPE.SCREENSHARE)} 
            
            autoPlay={true} width="100%" height="auto" ref={videoRef} />
        </div>
    );
}
