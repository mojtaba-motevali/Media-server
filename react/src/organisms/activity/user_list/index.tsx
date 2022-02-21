import { Badge, Container, IconButton, ListItem } from "@material-ui/core";
import { ProfileCard } from '../../../molecules/cards/profile';
import {useStyles} from './styles';
import {  ListChildComponentProps } from 'react-window';
import { ActiveComponent, ActiveComponentType } from  '../active_components';
import { PeerManager } from "../../../pages/room/managers";
import { MessageCardProps } from "../../../molecules/cards/message";
import { Dispatch, SetStateAction, useContext, useEffect, useState } from "react";
import { useMemo } from "react";
import { SearchList } from '../../../molecules/list_search';
import { shouldReRenderContext } from "../../../contexts";
import { Chat } from "@material-ui/icons";

interface UserListProps {
    peers:Map<number,PeerManager>,
    messageCardObjects:Map<number,MessageCardProps[]>,
    setAcitveComponent:Dispatch<SetStateAction<ActiveComponentType>>,
    unSeenPrivateMessages:Map<number,number>;
}
export const UserList = (props:UserListProps) => {
    const {
        peers,setAcitveComponent,messageCardObjects,unSeenPrivateMessages
    } = props;
    const classes = useStyles();
    const shouldRender = useContext(shouldReRenderContext);
    const [,render] = useState(0);
    useEffect(()=> {
        const peerPM = ()=> {
            render((prev) => prev+1);
        }
        const userListPrivateMessage = ()=> {
            render((prev) => prev+1);
        }
        shouldRender.on("PeerManager",peerPM);
        shouldRender.on("PrivateMessage",userListPrivateMessage);
        return () => {
            shouldRender.removeListener("PeerManager",peerPM);
            shouldRender.removeListener("PrivateMessage",userListPrivateMessage);
        }
        // it's not going to rerun anyway!
    },[shouldRender])
    const [searchFilter,setFilter] = useState('');
    const sortedPeers = useMemo(()=> {
        return Array.from(peers.values()).sort((p1:PeerManager,p2:PeerManager):number => {
            const l1 = messageCardObjects.get(p1.id);
            const l2 = messageCardObjects.get(p2.id);
            if(l1 && l2){
                const l1Length = l1.length;
                const l2Length = l2.length;
                if(l1Length > 0  && l2Length > 0 ){
                    const msg1 = l1[l1Length-1].messages;
                    const msg2 = l2[l2Length-1].messages;
                    const t1 = new Date (msg1[msg1.length-1].timestamp);
                    const t2 = new Date(msg2[msg2.length-1].timestamp)
                    if (t1 > t2 ){
                        return -1;
                    }else if (t1 < t2){
                        return 1;
                    }else {
                        return 0;
                    }
                }else if (l1Length < 1 && l2Length > 0) {
                    return 1;
                }else if (l1Length > 0 && l2Length < 1){
                    return -1;
                }else {
                    return 0;
                }
            }else {
                return 0;
            }
 
        });
    },[ messageCardObjects,peers.size ]);
    const filteredPeer = useMemo(()=>{
        return Array.from(peers.values()).filter((peer) => peer.name.includes(searchFilter));
    },[searchFilter,peers.size])

    const renderList =(peerList:PeerManager[]) => {
        return (props:ListChildComponentProps)=> {
            const { index, style } = props;
            let subTitle = "";
            const messageObject  = messageCardObjects.get(peerList[index].id)
            if (messageObject && messageObject?.length > 0){
                const messages = messageObject[messageObject.length - 1].messages;
                const text = messages[messages.length -1 ].text;
                subTitle = text.length < 15 ? text:text.substring(0,15) + "..."
            }
            // TODO: badge number counter on receive message
            const badgeNumber = unSeenPrivateMessages.get(peerList[index].id);
            return (
                <ListItem className={classes.listItem} key={index} style={style}>
                    <ProfileCard isLeft={true}  title={peerList[index].name} subTitle={subTitle}
                    MagicComponent={
                        <Container className={classes.listIconsContainer}>
                            <Badge style={{display: !badgeNumber || badgeNumber < 1 ? 'none':'block'}} 
                                className={classes.badge}  badgeContent={badgeNumber} />
                            <IconButton onClick={e=> {
                                setFilter('');
                                setAcitveComponent({
                                    state:ActiveComponent.PRIVATE_CHAT,
                                    data:{
                                        senderId:peerList[index].id
                                    }
                                })
                            }}>
                                <Chat/>
                            </IconButton>
                        </Container>
                    }
                      />
                </ListItem>
            )
        }
    } 
    const electedPeerList = searchFilter.length > 0 ? filteredPeer:sortedPeers;
    return (    
        <SearchList 
            value={searchFilter} onChange={(e)=>{
                setFilter(e.target.value);
            }}
            renderList={renderList(electedPeerList)} 
            listCount={electedPeerList.length}
        />
    );
}