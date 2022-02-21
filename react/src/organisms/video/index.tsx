import { Grid, IconButton } from '@material-ui/core';
import { useContext, useState } from 'react';
import { isMobileViewContext, shouldReRenderContext } from '../../contexts';
import { PeerManager,SelfManager } from '../../pages/room/managers';
import { useStyles } from './styles';
import { MicOffOutlined,MicOutlined,VideocamOffOutlined,VideocamOutlined,
        ScreenShareOutlined,StopScreenShareOutlined } from '@material-ui/icons/';
import { SERVICE_TYPE } from '../../pages/room/signal_types/app_data';
import { TURN_CAMERA_OFF, TURN_CAMERA_ON, TURN_MIC_OFF, TURN_MIC_ON, TURN_SCREENSHARE_OFF,
     TURN_SCREENSHARE_ON } from '../../pages/room/signal_types/room_media_actions';
import { useEffect } from 'react';
import { GridType } from './grid';
import { Navbar } from '../navbar';
import { GridView } from './grid_view';
import { ActiveSpeakerView } from './active_speaker';

interface VideoCallProps {
    selected:number[],
    peers:Map<number,PeerManager>,
    self:SelfManager,
}

export const VideoCall = ({selected,peers,self}:VideoCallProps) => {
    const classes = useStyles();
    const [activeGrid,setActiveGrid] = useState(GridType.GRID_VIEW);
    const shouldRender = useContext(shouldReRenderContext);
    const isMobile = useContext(isMobileViewContext);
    const [,render] = useState(0);
    useEffect(()=> {
        const videoCallPeerRenderer = ()=> {
            render((prev) => prev+1);
        }
        const videoCallSelfRenderer = ()=> {
            render((prev) => prev+1);
        }
        shouldRender.on("PeerManager",videoCallPeerRenderer);
        shouldRender.on("SelfManager",videoCallSelfRenderer)
        return () => {
            shouldRender.removeListener("PeerManager",videoCallPeerRenderer);
            shouldRender.removeListener("SelfManager",videoCallSelfRenderer);
        }
    },[selected,shouldRender]);
    const isMicEnabled = self.getSelfProducerByType(SERVICE_TYPE.VOICE) ? true : false;
    const isCameraEnabled = self.getSelfProducerByType(SERVICE_TYPE.CAMERA) ? true:false;
    const isScreenShareEnabled = self.getSelfProducerByType(SERVICE_TYPE.SCREENSHARE) ? true:false;
    const buttons = [
        {
            onClick:()=> isMicEnabled ?  self.emit(TURN_MIC_OFF): self.emit(TURN_MIC_ON),
            icon:!isMicEnabled ? <MicOutlined className={classes.svg} />:  <MicOffOutlined className={classes.svg} /> 
        },
        {
            onClick:()=> isCameraEnabled ?  self.emit(TURN_CAMERA_OFF): self.emit(TURN_CAMERA_ON),
            icon:!isCameraEnabled ? <VideocamOutlined  className={classes.svg} /> : <VideocamOffOutlined  className={classes.svg}  /> 
        }
    ];
    if(!isMobile)
        buttons.push({
            onClick:()=> isScreenShareEnabled ?  self.emit(TURN_SCREENSHARE_OFF): self.emit(TURN_SCREENSHARE_ON),
            icon:!isScreenShareEnabled ?<ScreenShareOutlined  className={classes.svg} />:<StopScreenShareOutlined  className={classes.svg}  />
        });
    return (
        <div className={classes.root} >
            <Navbar setGrid={setActiveGrid} currentGrid={activeGrid} />
            <Grid style={{height:'70vh',flexShrink:1}} container>
                {
                    activeGrid === GridType.GRID_VIEW ? (
                        <GridView selected={selected} peers={peers} self={self} />
                    ):(<ActiveSpeakerView peers={peers} self={self} selected={selected}  />)
                }
                <Grid  xs={12} item className={classes.buttonsContainer}>
                    { buttons.map((button,i)=> <IconButton key={i} className={classes.iconButton} onClick={button.onClick} >
                         {button.icon} </IconButton> ) }
                </Grid>
            </Grid>

        </div>
    );
}
                