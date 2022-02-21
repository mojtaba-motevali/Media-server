import { Badge, Container } from "@material-ui/core";
import {  AppPaper, AppTypography } from "../../atoms";
import { ProfileCard } from '../../molecules/cards/profile';
import {useStyles} from './styles';
import { PeerManager, SelfManager } from "../../pages/room/managers";
import { MessageCardProps } from "../../molecules/cards/message";
import { UserList } from './user_list';
import { Dispatch, SetStateAction, useMemo, useState } from "react";
import { VideoSelectionList } from './video_selection';
import { ThemeChanger } from '../../molecules/theme'
import { UserListPreview } from '../../molecules/userlist_preview';
import {ActiveComponent,ActiveComponentType} from './active_components';
import { Chat as ChatComponent } from '../chat';
import { OnChangeType } from "../../pages/room/ui_types/onchange";
import { NotificationImportant } from '@material-ui/icons';

interface MessangerProps {
    self:SelfManager,
    selectedVideos:number[],
    unSeenPrivateMessages:Map<number,number>;
    unSeenPublicMessages:{count:number};
    setSelected:Function,
    publicMessages:MessageCardProps[],
    peers:Map<number,PeerManager>,
    privateMessages:Map<number,MessageCardProps[]>,
    onChange:(props:OnChangeType) => void
}
export const Activity = (props:MessangerProps) => {
    const {
        self,publicMessages,peers,
        privateMessages,selectedVideos,setSelected,
        onChange,unSeenPrivateMessages,unSeenPublicMessages
    } = props;
    const classes = useStyles();
    const [activeComponent,setActiveComponent]:[ActiveComponentType,Dispatch<SetStateAction<ActiveComponentType>>] = useState({
        state:ActiveComponent.USER_LIST,
        data:{},
    });
    const renderComponent = (data:ActiveComponentType) => {
        switch (data.state) {
            case ActiveComponent.USER_LIST:
                return <UserList unSeenPrivateMessages={unSeenPrivateMessages} 
                peers={peers} setAcitveComponent={setActiveComponent} 
                messageCardObjects={privateMessages}/>
            case ActiveComponent.VIDEO_SELECTION:
                return <VideoSelectionList selected={selectedVideos}
                 setSelected={setSelected} peers={peers} />
            case ActiveComponent.PRIVATE_CHAT:
                let messages = privateMessages.get(data.data.senderId);
                if(!messages){
                    messages = [];
                    privateMessages.set(data.data.senderId,messages);
                }
                const peer = peers.get(data.data.senderId);
                if (peer)
                    return <ChatComponent 
                    unSeenPublicMessages={unSeenPublicMessages}
                    unSeenPrivateMessages={unSeenPrivateMessages}
                    type="PrivateMessage" 
                    self={self} peer={peer} onChange={onChange} messages={messages} />
                else 
                    throw new Error("peer doesn't exist anymore");
            case ActiveComponent.PUBLIC_CHAT:
                return <ChatComponent unSeenPublicMessages={unSeenPublicMessages}
                unSeenPrivateMessages={unSeenPrivateMessages}
                 type="PublicMessage" self={self} onChange={onChange}   messages={publicMessages}/>
        }
    }
    const numberOfBadge = useMemo(()=>{
        const number = unSeenPublicMessages.count + Array.from(unSeenPrivateMessages.values()).reduce((prev,curr)=>prev+curr,0)
        return number;
    },[])

    return (
        <AppPaper transition={{type:'Zoom',duration:1500}} className={classes.root} elevation={2}>
            <Container className={classes.container_body} >
                <div style={{paddingTop:'15px'}}>
                    <ProfileCard isSelf={
                        <div>
                            <Badge style={{display: !numberOfBadge || numberOfBadge < 1 ? 'none':'block'}} 
                                className={classes.badge}  badgeContent={numberOfBadge} />
                            <NotificationImportant />
                        </div>
                    } isLeft={false} 
                    // TODO: add light-mode styles
                    // MagicComponent={
                    //     <div style={{display:'flex'}}>
                    //             <ThemeChanger/>
                    //     </div>
                    // }
                    title={self.name} subTitle={""} />
                    </div>
                <UserListPreview value={Number(activeComponent.state)} setValue={setActiveComponent} peers={peers} />

                {
                    Number(activeComponent.state) === ActiveComponent.VIDEO_SELECTION &&  <AppTypography  style={{fontSize:'14px',textAlign:'start', marginTop:'10px'}} color="secondary">
                    The maximum of Video selection is 3.
                    </AppTypography> 
                }
                { renderComponent(activeComponent) }

            </Container>
        </AppPaper>
    );
}