
import {makeStyles} from '@material-ui/core/styles';
import {Theme} from '@material-ui/core';
import { COLORS } from '../../colors';


export const useStyles = makeStyles((theme:Theme)=>({
    [theme.breakpoints.up('xs')]:{
        root:{
            height:'100vh',
            overflow:'hidden',
            backgroundColor: theme.palette.type === 'light'? COLORS.WHITE: '#140c14',
            backgroundImage: theme.palette.type === 'light'? 'unset':'radial-gradient(at 47% 33%, hsl(220.00, 33%, 37%) 0, transparent 59%),radial-gradient(at 82% 65%, hsl(326.94, 49%, 20%) 0, transparent 55%)',
        },
        sideBar:{
            display:'flex',
            flexFlow:'column',
            justifyContent:'center'
        },
        my_grid:{
            height:'100%',
        },
        container:{
            display:'flex',
            justifyContent:'center'
        }
    },
    [theme.breakpoints.up('sm')]:{
    },
    [theme.breakpoints.up('md')]:{
        container:{
            display:'flex',
            justifyContent:'center'
        }
    },

}));
