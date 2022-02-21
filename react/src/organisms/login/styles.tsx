import { makeStyles } from '@material-ui/core';
import BackgroundImage  from '../../images/login_background.png';

export const useStyles = makeStyles(()=>({
    root:{
        position:'absolute',
        width:'100%',
        minHeight:'100%',
        background: `url(${BackgroundImage})`,
        height:'auto',
        top:0,
        left:0
    },
    container:{
        paddingTop:'35vh',
        display:'flex',
        justifyContent:'center',
    },
}));
