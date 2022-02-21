export  interface PrivateMessage {
    id?:number;
    sender_id:number;
    receiver_id:number;
    message:string;
    timestamp:Date;
}