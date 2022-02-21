import { PeerManager, SelfManager } from '../../../pages/room/managers';
import { SERVICE_TYPE } from '../../../pages/room/signal_types/app_data';
import { Grid } from '@material-ui/core';
import { SelfVideo } from '../self';
import { PeerVideo } from '../peer';
import { Fragment, useContext, useEffect, useState } from 'react';
import { isMobileViewContext, shouldReRenderContext } from '../../../contexts';
import { useStyles } from './styles';
interface PeerVideoProps {
    peers:Map<number,PeerManager>,
    self:SelfManager,
    selected:number[],
}

const findActiveSpeaker = (peers:Map<number,PeerManager>,activeSpeakerId:number,selected:number[]):PeerManager | undefined => {
    const activeSpeaker = peers.get(activeSpeakerId);
    if(activeSpeaker){
        return activeSpeaker;
    }else {
        return peers.get(selected[0]);
    }
}

export const ActiveSpeakerView = ({self,peers,selected}:PeerVideoProps) => {
    const classes=  useStyles();
    const selfProduced = self.getProducers().filter(producer => producer.appData.service_type !== SERVICE_TYPE.VOICE);
    const shouldRender = useContext(shouldReRenderContext);
    const isMobile = useContext(isMobileViewContext);
    const [,render] = useState(0);
    const [activeSpeakerId,setActiveSpeakerId] = useState(-1);
    useEffect(()=> {
        shouldRender.on('ActiveSpeaker', (id)=> {
            if (id !== activeSpeakerId)
                setActiveSpeakerId(id);
        })
        const GridViewPeerRenderer = ()=> {
            render((prev) => prev+1);
        }
        shouldRender.on("PeerManager",GridViewPeerRenderer);
        return () => {
            shouldRender.removeListener("PeerManager",GridViewPeerRenderer);
        }
    },[]);
    const activeSpaeker = findActiveSpeaker(peers,activeSpeakerId,selected);
    return (
       <Grid style={{justifyContent:'center',flexWrap:'wrap'}} item container spacing={2}>
           {    activeSpaeker && <Grid  item xs={12}>
                    <div className={classes.activeSpeaker}>
                        <PeerVideo selectedVideos={selected} isActiveSpeaker={true} peer={activeSpaeker}   />
                    </div>
                </Grid>
            }
          { !isMobile && selfProduced.map((producer,i) => <Grid style={{paddingTop:'50px'}}  key={i} 
          item xs={4}> <div  style={{width:'240px',height:'120px'}}> <SelfVideo producer={producer} /> </div> </Grid>) }
          {!isMobile && selected.map((id,i)=> {
                    const peer = peers.get(id);
                    if(peer && activeSpaeker?.id !== peer.id){
                        return (
                            <Grid  style={{paddingTop:'50px'}} key={i}  xs={4}  item> 
                                <div style={{width:'240px',height:'120px'}}>
                                    <PeerVideo selectedVideos={selected} peer={peer}   />
                                </div>

                            </Grid>
                        )
                    }else {
                        return <></>
                    }
                })
            }
                
       </Grid>
    );
}
