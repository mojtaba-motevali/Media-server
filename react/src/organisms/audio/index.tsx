
import { useContext, useState } from 'react';
import { useEffect } from 'react';
import { PeerManager } from '../../pages/room/managers'
import { SERVICE_TYPE } from '../../pages/room/signal_types/app_data';
import { useStyles } from './styles';
import { shouldReRenderContext } from '../../contexts'
interface NoFaceAudioProps {
    peers:Map<number,PeerManager>
}

export const NoFaceAuido = ({peers}:NoFaceAudioProps) => {
    const classes = useStyles();
    const shouldRender = useContext(shouldReRenderContext);
    const [,render] = useState(0);
    useEffect(()=> {
        const NoFaceAuidoRenderer = ()=> {
            render((prev) => prev+1);
        }
        shouldRender.on("PeerManager",NoFaceAuidoRenderer);
        return () => {
            shouldRender.removeListener("PeerManager",NoFaceAuidoRenderer);
        }
    },[shouldRender])
    const audioStreams = [];
    for( const peer of peers.values()){
        const audioConsumer = peer.getConsumerByType(SERVICE_TYPE.VOICE);
        if(audioConsumer){
            peer.resume(SERVICE_TYPE.VOICE)
            const ms = new MediaStream();
            ms.addTrack(audioConsumer.track);
            audioStreams.push(ms)
        }
    }
    return (
        <div className={classes.root}>
            {
                audioStreams.map((stream,i) => <video key={i} muted={false} ref={audio => {
                    if(audio){
                        audio.srcObject = stream;
                        audio.onloadedmetadata = async (e)=> {
                            await audio.play()
                        }
                    }
                }}
                 autoPlay={true} />)
            }
        </div>
    );
}
