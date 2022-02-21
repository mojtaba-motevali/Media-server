import { useState } from 'react';

import {IconButton} from '@material-ui/core';
import { AppInput } from '../../../atoms/input';
import {useStyles} from './styles';
import SendOutlinedIcon from '@material-ui/icons/SendOutlined';

interface ChatInputProps {
    onSubmit: (message:string) => void,
}
export const ChatInput = ({onSubmit}:ChatInputProps) => {
    const classes = useStyles();
    const [message,setMessage]:[string,Function] = useState('');
    const onChange = (e:any)=>{
        setMessage(e.target.value);
    };
    return (
        <div>
            <AppInput onKeyUp={(event: React.KeyboardEvent<HTMLTextAreaElement | HTMLInputElement>)=>{
                if ( event.key === 'Enter' ) {
                    if(message.length > 0){
                        setMessage('');
                        onSubmit(message)
                    }
                }
            }} placeholder="Enter your message here" onChange={onChange} value={message} className={classes.root}  endAdornment={
                <IconButton  onClick={()=> {
                    if(message.length > 0){
                        setMessage('');
                        onSubmit(message)
                    }
                }} className={classes.iconButton}>
                    <SendOutlinedIcon  className={classes.sendIcon}/>
                </IconButton>
            }/>
        </div>
    )
}