

/**
 * https://www.npmjs.com/package/react-chat-elements
 */
declare module "*.png"
declare module "@material-ui/core/styles/createBreakpoints" {
    interface BreakpointOverrides {
      xs: false; // removes the `xs` breakpoint
      sm: false;
      md: false;
      lg: false;
      xl: false;
      tablet: true; // adds the `tablet` breakpoint
      laptop: true;
      desktop: true;
    }
  }
declare module 'react-chat-elements' {
    interface MessageBoxProps {
        position?:string;
        type?:string;
        text?:string;
        title?:string;
        titleColor?:string;
        date?:Date;
        dateString?:string;
        avatar?:string;
        notch?:boolean;
        data?:{
            uri:string;
            status:{
                click:false;
                loading:number
            }
        }
        
    }
    export function MessageBox(props:MessageBoxProps):any;
    interface MessageListProps {
        className?:string;
        dataSource?:MessageBoxProps[];
        lockable?:boolean;
        toBottomHeight?:number | string;

    }
    export function MessageList(props:MessageListProps):any;
    interface InputProps {
        className?:string;
        onKeyPress?:(e:any) => void;
        placeholder?:string;
        defaultValue?:string;
        value:string;
        onChange?:(e:any) => void;
        multiline?:boolean;
        minHeight?:number;
        maxHeight?:number;
        inputStyle?:object;
        leftButtons?:any
        ref?:Function;
        rightButtons?:any;
        autofocus?:boolean;
    }
    export function Input(props:InputProps):any;
    interface ButtonProps {
        type?:string;
        text?:string;
        color?:string;
        backgroundColor?:string;
        onClick:(e:any)=>void;
        title?:string;
        buttonRef?:Function;
        disabled?:boolean;
    }
    export function Button(props:ButtonProps):any;
    interface ChatListProps {
        className?:string;
        dataSource?:object[];
        onClick?:(e:any)=>void;
    }
    export function ChatList(props:ChatListProps):any;
}