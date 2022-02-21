import { PeerManager, SelfManager } from '../../../pages/room/managers';
import { SERVICE_TYPE } from '../../../pages/room/signal_types/app_data';
import { Grid, GridSize } from '@material-ui/core';
import { SelfVideo } from '../self';
import { PeerVideo } from '../peer';
import { Fragment, useContext, useEffect, useState } from 'react';
import { shouldReRenderContext } from '../../../contexts';

interface PeerVideoProps {
    peers:Map<number,PeerManager>,
    self:SelfManager,
    selected:number[],
}
const resolutions:{width:string,height:string,xs:GridSize}[] = [
    {width:'720px',height:'650px',xs:12},
    {width:'480px',height:'360px',xs:4},
    {width:'352px',height:'240px',xs:4},
];
export const GridView = ({self,peers,selected}:PeerVideoProps) => {
    const selfProduced = self.getProducers().filter(producer => producer.appData.service_type !== SERVICE_TYPE.VOICE);
    const shouldRender = useContext(shouldReRenderContext);
    const [,render] = useState(0);
    useEffect(()=> {
        const GridViewPeerRenderer = ()=> {
            render((prev) => prev+1);
        }
        shouldRender.on("PeerManager",GridViewPeerRenderer);
        return () => {
            shouldRender.removeListener("PeerManager",GridViewPeerRenderer);
        }
    },[shouldRender]);
    const total = selfProduced.length + selected.length;
    const index = total > 3 ? 2:(total)-1
    const {xs,width,height} = typeof resolutions[index] !== 'undefined' ? resolutions[index]:{xs:12,width:'1280px',height:'720px'} ;
    return (
       <Grid style={{justifyContent:'center',flexWrap:'wrap'}} item container spacing={2}>
          { selfProduced.map((producer,i) => <Grid style={{display:'flex',justifyContent:'center'}}  key={i} item xs={xs as GridSize}> 
            <div style={{width,height}} >
                <SelfVideo producer={producer} /> 
            </div>
          
          </Grid>) }
          {selected.map((id,i)=> {
                    const peer = peers.get(id);
                    if(peer){
                        return (
                            <Grid key={i} style={{display:'flex',justifyContent:'center'}}  xs={xs as GridSize}  item> 
                                <div style={{maxWidth:width,width:'100%',height}} >
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
