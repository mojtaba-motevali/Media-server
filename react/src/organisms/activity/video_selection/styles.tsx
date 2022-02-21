import { makeStyles } from '@material-ui/core';
import { COLORS } from '../../../colors';

export const useStyles = makeStyles(({palette})=>({
    listItem:{
        borderRadius:'10px',
        padding:'0',
        cursor:'pointer',
        backgroundColor:palette.type === 'light' ? COLORS.LIGHT_BG : COLORS.DARK_GRAY_2,
        '&:hover':{
            backgroundColor:palette.type === 'light' ? COLORS.LIGHT_GRAY_2 : COLORS.DARK_GRAY_4,
            border:`1px solid ${palette.type === 'light' ? COLORS.LIGHT_GREEN_2:COLORS.LIGHT_GRAY}`,
        },
        '&:focus':{
            backgroundColor:palette.type === 'light' ? COLORS.LIGHT_GRAY_2 : COLORS.DARK_GRAY_4,
            border:`1px solid ${palette.type === 'light' ? COLORS.LIGHT_GREEN_2:COLORS.LIGHT_GRAY}`,
        }
    },
}));
