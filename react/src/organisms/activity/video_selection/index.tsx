import { Checkbox, Container, FormControlLabel, ListItem } from "@material-ui/core";
import { ProfileCard } from '../../../molecules/cards/profile';
import {useStyles} from './styles';
import {  ListChildComponentProps } from 'react-window';
import { PeerManager } from "../../../pages/room/managers";
import { useContext, useEffect, useState } from "react";
import { useMemo } from "react";
import { SearchList } from '../../../molecules/list_search';
import { VideocamOutlined, VideocamRounded } from "@material-ui/icons";
import { useCallback } from "react";
import { shouldReRenderContext } from "../../../contexts";
import { SERVICE_TYPE } from "../../../pages/room/signal_types/app_data";

interface PrivateMessageListProps {
    selected:number[],
    peers:Map<number,PeerManager>,
    setSelected:Function,
}
export const VideoSelectionList = ({peers,setSelected,selected}:PrivateMessageListProps) => {
    const classes = useStyles();
    const [searchFilter,setFilter] = useState('');
    const shouldRender = useContext(shouldReRenderContext);
    const [,render] = useState(0);

    useEffect(()=> {
        const VideoRenderer = ()=> {
            render((prev) => prev+1);
        }
        shouldRender.on("PeerManager",VideoRenderer);
        return () => {
            shouldRender.removeListener("PeerManager",VideoRenderer);;
        }
        // it's not going to run anyway! 
    },[shouldRender]);
    const peersWithVideo = Array.from(peers.values()).filter(peer => {
            return peer.hasConsumerWithType(SERVICE_TYPE.CAMERA) || peer.hasConsumerWithType(SERVICE_TYPE.SCREENSHARE)
    });
    const sortedPeers = useMemo(()=> {
        return Array.from(peersWithVideo.values()).sort((p1:PeerManager,p2:PeerManager):number => {
            const has = [false,false];
            for(let i = 0 ; i < selected.length ; ++i){
                if(p1.id === selected[i])
                    has[0] = true;
                if(p2.id === selected[i])
                    has[1] = true;
            }
            if(has[0] && has[1])
                return 0;
            else if(has[0])
                return -1;
            else 
                return 1;
        });
    },[ peersWithVideo.length,selected ]);
    const filteredPeer = useMemo(()=>{
        return Array.from(peersWithVideo.values()).filter((peer) => peer.name.includes(searchFilter));
        // no need to re calculate filter when user joins, so peers.length is not provided in deps
    },[searchFilter,peersWithVideo.length])
    const renderList = (peerList:PeerManager[]) => {
        return (props:ListChildComponentProps)=> {
            const { index, style } = props;
            const peer = peerList[index];
            let subTitle = "";
            return (
                <ListItem  button onClick={e=> {
                    if(searchFilter.length > 0)
                        setFilter('');
                    setSelected({peerId:peer.id})
                }} className={classes.listItem} key={index} style={style}>
                    <ProfileCard isLeft={true} title={peerList[index].name} subTitle={subTitle}
                    MagicComponent={
                        <Container style={{display:'flex',justifyContent:'flex-end'}}>
                            <FormControlLabel
                                checked={selected.indexOf(peer.id) >= 0}
                                label=""
                                control={<Checkbox icon={<VideocamOutlined />} checkedIcon={<VideocamRounded />}/>}
                            />    
                        </Container>
                    }
                    />
                </ListItem>
            )
        }
    } 
    const onFilterChange = useCallback((e)=> {
        setFilter(e.target.value);
    },[])
    const electedPeerList = searchFilter.length > 0 ? filteredPeer:sortedPeers;
    return (    
        <SearchList 
            value={searchFilter} onChange={onFilterChange}
            renderList={renderList(electedPeerList)} 
            listCount={electedPeerList.length}
        />
    );
}