import { createRef, useEffect } from 'react';
import { useStyles } from './styles';
import { useCallback } from 'react';
import { Producer } from 'mediasoup-client/lib/Producer';
import { SERVICE_TYPE } from '../../../pages/room/signal_types/app_data';

interface selfVideoProps {
    producer:Producer,
}

export const SelfVideo = ({producer}:selfVideoProps) => {
    const classes = useStyles();
    const activeSpeakerVideoRef = createRef<HTMLVideoElement>();
    const addStream = useCallback((producer:Producer) => {
        if(producer.track){
            const mediaStream = new MediaStream();
            mediaStream.addTrack(producer.track);
            if( activeSpeakerVideoRef.current)
                activeSpeakerVideoRef.current.srcObject = mediaStream;
        }
    },[activeSpeakerVideoRef]);
    useEffect(()=>{
        addStream(producer);
    },[addStream,producer])
    const service_type = producer.appData.service_type;
    return (
        <div className={classes.root} >
            <video  controls={service_type === SERVICE_TYPE.SCREENSHARE} ref={activeSpeakerVideoRef} autoPlay={true} style={{width:'100%',height:'100%'}} />            
        </div>
    );
}
