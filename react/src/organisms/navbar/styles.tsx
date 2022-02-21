import { makeStyles, Theme } from '@material-ui/core';
import { COLORS } from '../../colors';

export const useStyles = makeStyles(({palette}:Theme)=>({
    root:{
        justifyContent:'space-between',
    },
    gridSelection:{
        display:'flex',
        justifyContent:'flex-end',
        paddingRight:"30px",
        paddingTop:"30px",
        paddingBottom:'5px'
    },
    gridIcons:{
        color:COLORS.LIGHT_SILVER,
        width:'20px',
        height:'20px'
    },
    active:{
        color:`${palette.type === 'light' ? COLORS.LIGHT_GREEN:COLORS.DARK_GREEN}`
    }
}));
