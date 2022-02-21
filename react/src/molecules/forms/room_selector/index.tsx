import { Fragment, useCallback, useState } from 'react';

import {Container,FormControl,InputAdornment} from '@material-ui/core';
import RoomIcon from '@material-ui/icons/Room';
import { AppInput } from '../../../atoms/input';
import { AppButton } from '../../../atoms/button';
import {useStyles} from './styles';


export const RoomSelection = ({history}:{history:any}) => {
    const classes = useStyles();
    const [room,setRoomName]:[string,Function] = useState('');
    const onChange = useCallback((e)=>{
        setRoomName(e.target.value);
    },[]);
    const onSubmit = (e:any) => {
        e.preventDefault();
        history.push(`/room/${room}`);
    }
    return (
        <Fragment>
                <FormControl onSubmit={onSubmit} className={classes.formControl}  variant="outlined">
                        <AppInput
                        onKeyUp={(event: React.KeyboardEvent<HTMLTextAreaElement | HTMLInputElement>)=>{
                            if ( event.key === 'Enter' ) {
                                onSubmit(event);
                            }}}
                            transition={{type:'Grow',duration:1000}}
                            placeholder={`room's name`}
                            onChange={onChange}
                            className={classes.input}
                            endAdornment={<InputAdornment position="end"><RoomIcon/></InputAdornment>}
                            labelWidth={0}
                        />
                    <Container className={classes.container}>
                            <AppButton transition={{type:'Grow',duration:1200}} onClick={onSubmit} className={classes.button}>
                                Enter Room
                            </AppButton>
                    </Container>

                </FormControl>

        </Fragment>
    )
}