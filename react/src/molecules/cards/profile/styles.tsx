
import {makeStyles} from '@material-ui/core/styles';
import {Theme} from '@material-ui/core';
import { COLORS } from '../../../colors';

export const useStyles = makeStyles((theme:Theme)=> ({
    [theme.breakpoints.up('xs')]:{
        container:{
            display:'flex',
        },

        avatar:{
            justifySelf:'flex-start',
            width:'45px',
            height:'45px'         
        },
        title:{
            fontSize:'14px',
        },
        subtitle:{
            fontSize:'12px',
            fontWeight:'300',
            color:COLORS.LIGHT_SILVER
        },
    },
    [theme.breakpoints.up('md')]:{
        title:{
            fontSize:'18px',
        },
        avatar:{
            width:'50px',
            height:'50px'         
        },
        subtitle:{
            fontSize:'12px',
            fontWeight:'300',
            color:COLORS.LIGHT_SILVER
        },
    },
}));
