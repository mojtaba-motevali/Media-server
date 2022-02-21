
import {makeStyles} from '@material-ui/core/styles';
import {Theme} from '@material-ui/core';
import { COLORS } from '../../../colors';

export const useStyles = makeStyles((theme:Theme)=> ({
    [theme.breakpoints.up('xs')]:{
        root:{
            width:'100%',
            height:'50px',
            borderRadius:'20px',
            paddingRight:'5px',
            backgroundColor:theme.palette.type === 'light' ? COLORS.WHITE:COLORS.DARK_GRAY_3,
            '&:focus-within':{
                '& > fieldset':{
                    border:'2px solid #9d9ab8 !important',
                    outline:'none'
                }
            }
        },
        iconButton:{
            backgroundColor: theme.palette.type === 'light'? COLORS.LIGHT_BG:COLORS.DARK_GRAY,
            padding:'10px', 
            width:'35px',
            height:'35px',
            borderRadius:'30px',
            '&:hover':{
                backgroundColor:'#359e05 !important',
            }
        },
        sendIcon:{
            color:COLORS.DARK_GREEN,
            fontSize:'20px'
        }
    },
}));
