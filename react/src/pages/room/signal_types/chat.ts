export interface PrivateMessage {
    event:'private_chat';
    message:string;
    receiver_id:number;
    sender_id:number;
    timestamp:string;
 }
 
export interface PublicMessage {
     event:'broadcast_message'
     sender_id:number;
     message:string;
     user_name:string;
     timestamp:string;
 }
 