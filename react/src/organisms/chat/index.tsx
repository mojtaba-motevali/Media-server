import { Container } from '@material-ui/core';
import { useContext, useState } from 'react';
import { createRef } from 'react';
import { AppPaper } from '../../atoms';
import { shouldReRenderContext } from '../../contexts';
import { MessageCardProps,MessageCard } from '../../molecules/cards/message';
import { ChatInput } from '../../molecules/forms';
import { PeerManager,SelfManager } from '../../pages/room/managers';
import { OnChangeType } from '../../pages/room/ui_types/onchange';
import { useStyles } from './styles';
import { useEffect } from 'react';
import { useCallback } from 'react';

interface ChatProps {
    messages:MessageCardProps[],
    onChange:(message:OnChangeType) => void,
    self:SelfManager,
    peer?:PeerManager,
    unSeenPrivateMessages:Map<number,number>;
    unSeenPublicMessages:{count:number};
    type:"PrivateMessage" | "PublicMessage",
}

export const Chat = ({messages,unSeenPublicMessages,unSeenPrivateMessages,onChange,peer,type,self}:ChatProps) => {
    const classes = useStyles();
    const shouldRender = useContext(shouldReRenderContext);
    const [,render] = useState(0);
    const C2 = useCallback(()=> {
        render((prev) => prev+1);
    },[render]);
    const C1 = useCallback(()=> {
        render((prev) => prev+1);
    },[render]);
    if(peer)
        unSeenPrivateMessages.set(peer.id,0);
    else 
        unSeenPublicMessages.count = 0;
    useEffect(()=> {
        if(type === 'PrivateMessage'){
            shouldRender.on("PrivateMessage",C2);
        } else{
            shouldRender.on("PublicMessage",C1);
        } 
        return () => {
            if(type === 'PrivateMessage'){
                shouldRender.removeListener("PrivateMessage",C2);
            } else{
                shouldRender.removeListener("PublicMessage",C1);
            } 
        }
    },[shouldRender,C1,C2])
    const messageListRef:React.RefObject<any> = createRef();
    const scrollDown = useCallback(() => {
        if (messageListRef && messageListRef.current )
            messageListRef.current.scrollTop = messageListRef.current.scrollHeight;
    },[messageListRef]);
    useEffect(()=>{
        scrollDown();
    },[scrollDown])
    return (
        <AppPaper className={classes.root} elevation={2}>
            <Container ref={messageListRef} className={classes.message_list} >
                {
                    messages.map((message:MessageCardProps,i) => <MessageCard key={i}  {...message} />)
                }
            </Container>
            <Container>
                <ChatInput  onSubmit={(message:string) => {
                    if(type === "PrivateMessage" && peer){
                        const pMessage = peer.sendMessage(message,self.id);
                        onChange({
                            event:type,
                            data:[pMessage],
                            action:""
                        })
                    }else if (type === "PublicMessage") {
                        const pMessage = self.sendMessage(message);
                        onChange({
                            event:type,
                            data:[pMessage],
                            action:""
                        })
                    }
                }}/>
            </Container>
        </AppPaper>
    );
}
