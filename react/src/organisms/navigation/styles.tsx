
import {makeStyles} from '@material-ui/core/styles';
import {Theme} from '@material-ui/core';
import { COLORS } from '../../colors';

export const useStyles = makeStyles(({breakpoints,palette}:Theme)=> ({
    [breakpoints.up('xs')]:{
        icons:{
            
        },
        row:{
            display:'flex',
            justifyContent:'space-evenly',
        },
        column:{
            display:'flex',
            flexFlow:'column',
            justifyContent:'center',
        },
        icon:{
            color:COLORS.LIGHT_SILVER,
            width:'40px',
            height:'40px'
        },
        iconButton: {
            width:'fit-content',
        },
        active:{
            color:`${palette.type === 'light' ? COLORS.LIGHT_GREEN:COLORS.DARK_GREEN}`
        }
    }
}));
