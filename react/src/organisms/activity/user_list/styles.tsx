import { makeStyles } from '@material-ui/core';
import { COLORS } from '../../../colors';

export const useStyles = makeStyles(({palette})=>({
    listItem:{
        padding:'0',
        backgroundColor:palette.type === 'light' ? COLORS.LIGHT_BG : COLORS.DARK_GRAY_2,
    },
    listIconsContainer:{
        display:'flex',
        alignItems:'center'
    },
    badge:{
        marginRight:'15px',
        '& .MuiBadge-badge':{
            backgroundColor:COLORS.RED,
            color:COLORS.WHITE
        },
    },
}));
