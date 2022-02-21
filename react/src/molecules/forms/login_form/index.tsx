import { useState } from 'react';
import {Container,FormControl,InputAdornment} from '@material-ui/core';
import PermIdentityOutlinedIcon from '@material-ui/icons/PermIdentityOutlined';
import { useCallback } from 'react';
import {userContext} from '../../../contexts/auth';
import { useContext } from 'react';
import { AppInput, AppTypography,AppButton,
} from '../../../atoms';
import {useStyles} from "./styles";


export const LoginForm = ({setStage}:{setStage:Function}) => {
    const [username,setUsername]:[string,Function] = useState('');
    const [,setUser]:any = useContext(userContext);
    const emailOnChange= useCallback((e)=>{
        setUsername(e.target.value);
    },[]);
    const onLoginSubmit = (e:any) => {
        e.preventDefault();
        setUser({
            id:1,
            name:username,
            is_login:true,
            token:""
        })
        setStage(2);
    }
    const classes = useStyles();
    return (
        <Container onSubmit={onLoginSubmit} component="form">
            <AppTypography className={classes.typography} transition={{duration:1000,type:'Grow'}}>
                Login
            </AppTypography>
            <FormControl  className={classes.formControl}  variant="outlined">
                <AppInput 
                    transition={{type:'Grow',duration:1100}}
                    placeholder={'your name'}
                    className={classes.input}
                    value={username}
                    // onBlur={onBlur}
                    onChange={emailOnChange}
                    endAdornment={<InputAdornment position="end"><PermIdentityOutlinedIcon/></InputAdornment>}
                    inputProps={{
                    
                    }}
                    labelWidth={0}
                />
            </FormControl>

            <Container className={classes.container}>
                <AppButton 
                 transition={{type:'Grow',duration:1300}} onClick={onLoginSubmit} className={classes.button}>
                    Login
                </AppButton>
            </Container>   
        </Container>
    );
}