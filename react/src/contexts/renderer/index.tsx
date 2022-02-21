import EventEmitter from "events";
import { createContext } from "react";


export const shouldReRenderContext:React.Context<EventEmitter> = createContext(new EventEmitter());