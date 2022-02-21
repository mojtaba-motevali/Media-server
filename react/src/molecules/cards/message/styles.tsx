
import {makeStyles} from '@material-ui/core/styles';
import {Theme} from '@material-ui/core';
import { COLORS } from '../../../colors';

export const useStyles = makeStyles((theme:Theme)=> ({
    [theme.breakpoints.up('xs')]:{
        selfBackgroundCard:{
            backgroundColor: theme.palette.type === 'light' ? COLORS.LIGHT_GREEN : COLORS.DARK_GREEN,
        },
        peerBackgroundCard:{
            backgroundColor: theme.palette.type === 'light' ?  COLORS.LIGHT_BLACK : COLORS.DARK_GRAY_2,
        },
        avatar:{
            width:'45px',
            height:'45px',
        },
        textGrid:{
            placeSelf:'start',
            display:'flex',
            flexDirection:'column'
        },
        card:{
            lineBreak:'anywhere',
            margin:'10px',
            borderRadius:'10px 0px 10px 10px',
            display:'flex',
            flexDirection:'column'
        },
        cardContent:{
            padding:'0 !important',
            display:'flex',
            flexDirection:'row',
            flexWrap:'wrap',
            height:'100%',
            width:'100%'
        },
        message:{
            fontSize:'14px',
            textAlign:'center'
        },
        messageTime:{
            fontSize:'12px',
        }
    }
}));
