

export enum ActiveComponent {
    PUBLIC_CHAT = 0,
    USER_LIST = 1,
    VIDEO_SELECTION = 2,
    PRIVATE_CHAT = 3
}

interface PublicChat {
    state:0,
    data:{}
} 
interface UserList   {
    state:1,
    data:{}
} 
interface VideoSelection {
    state:2,
    data:{}
} 
interface PrivateChat   {
    state:3,
    data:{
        senderId:number
    }
} 
export type ActiveComponentType = PrivateChat | PublicChat | UserList | VideoSelection ;
