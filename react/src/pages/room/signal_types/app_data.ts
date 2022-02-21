
export enum SERVICE_TYPE {
    VOICE = "VOICE",
    CAMERA = "CAMERA",
    SCREENSHARE = "SCREENSHARE"
} 

export interface AppData {
    service_type: SERVICE_TYPE.VOICE | SERVICE_TYPE.CAMERA | SERVICE_TYPE.SCREENSHARE;
    user_id:number
}