
import {PublicMessage} from '../../@types/public_message';
import {PrivateMessage} from '../../@types/private_message';
import { MessageCardProps } from '../../molecules/cards/message';
import { PeerManager, SelfManager } from '../../pages/room/managers';
import { UserManager } from '../../pages/room/managers/usermanager';

interface ICreateOrAddMessage {
    messageCards:MessageCardProps[],
    message:PrivateMessage|PublicMessage,
    self:SelfManager,
    peer:PeerManager | undefined
}
const addOrCreateMessage = ({messageCards,message,self,peer}:ICreateOrAddMessage) => {
    // if there has been messages between two
    const lastIndex = messageCards.length - 1;
    const currentUserId = messageCards[lastIndex]?.userId;
    // last card's user id is the sender_id then create new card
    // otherwise add message to it's list
    if( currentUserId !== message.sender_id){
        let isSelf = false;
        if (message.sender_id === self.id){
            isSelf = true;
        }
        if(!isSelf && peer)
            messageCards.push(
                createMessageCard({messageObject:message,isLeft:!isSelf,user:peer})
            );
        else if (isSelf){
            messageCards.push(
                createMessageCard({messageObject:message,isLeft:!isSelf,user:self})
            );
        }
    }else {
        messageCards[lastIndex].messages.push({
            text:message.message,
            timestamp:typeof message.timestamp === 'string' ? new Date(message.timestamp):message.timestamp
        });
    }
}

interface ICreateMessageCard {
    messageObject:PrivateMessage | PublicMessage,
    user:UserManager,
    isLeft:boolean
}
const createMessageCard = ({messageObject,user,isLeft}:ICreateMessageCard):MessageCardProps => {
    const {message,timestamp} = messageObject;

    const newMessageCard:MessageCardProps = {
        userId: user.id,
        avatar: user.avatar,
        isLeft,
        name:user.name,
        messages:[{ text:message, timestamp:timestamp, }]
    };
    return newMessageCard
}

interface ISerializePublic {
    publicMessages:PublicMessage[],
    publicMessagesCards:MessageCardProps[],
    peers:Map<number,PeerManager>,
    selfManager:SelfManager
}

export const serializePublicChat = ({publicMessages,publicMessagesCards,peers,selfManager}:ISerializePublic) => {

    for(const message of publicMessages) {
        const peer = peers.get(message.sender_id);
        if (peer)
            addOrCreateMessage({message,messageCards:publicMessagesCards,self:selfManager,peer})
        else if (message.sender_id === selfManager.id){
            addOrCreateMessage({message,messageCards:publicMessagesCards,self:selfManager,peer:undefined})
        }
    }
}

interface ISerializePrivateChat {
    privateChats:PrivateMessage[],
    privateMessages:Map<number,MessageCardProps[]>,
    peers:Map<number,PeerManager>,
    selfManager:SelfManager
}

export const serializePrivateChat = ({
    privateChats,privateMessages,peers,selfManager
}:ISerializePrivateChat) =>  {

    for(let i = 0 ; i< privateChats.length ; ++i) {
        const message = privateChats[i];
        let privateMessageCards:MessageCardProps[] | undefined;
        let peer:PeerManager| undefined;
        // if sender id is self
        if (message.sender_id === selfManager.id){
            // then get peer messages 
            privateMessageCards = privateMessages.get(message.receiver_id);
            peer = peers.get(message.receiver_id);
        }else {
            privateMessageCards = privateMessages.get(message.sender_id);
            peer = peers.get(message.sender_id);
        }
        // if there has been no messages by now and peer exists
        if(!privateMessageCards && peer){
            // create a message Card
            let isSelf = false;
            let privateMessage; 
            if (message.sender_id === selfManager.id){
                isSelf = true;
            }
            if(!isSelf){
                privateMessage = createMessageCard({messageObject:message,isLeft:!isSelf,user:peer});
            }
            else {
                privateMessage = createMessageCard({messageObject:message,isLeft:!isSelf,user:selfManager});
            }
            // set it
            privateMessages.set(peer.id,[privateMessage]);
        } else if (privateMessageCards && peer){
            addOrCreateMessage({message,messageCards:privateMessageCards,self:selfManager,peer})
        }
    }
}