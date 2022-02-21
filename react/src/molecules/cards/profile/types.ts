import { ReactNode } from "react";


export enum ProfileType {
    VideoSelection = 0,
    ChatList = 1
}
export interface ProfileCardProps {
    isLeft:boolean,
    title:string,
    isSelf?:ReactNode,
    avatar?:string,
    subTitle?:string,
    MagicComponent?:ReactNode
}   