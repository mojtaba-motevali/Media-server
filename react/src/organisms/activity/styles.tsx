import { makeStyles } from '@material-ui/core';
import { COLORS } from '../../colors';

export const useStyles = makeStyles(({palette})=>({
    root:{
        height:'100%',
        width:'100%',
        display:'flex',
        flexFlow:'column',
        backgroundColor:`${palette.type === 'light' ? COLORS.LIGHT_BG:COLORS.DARK_GRAY} !important`

    },
    activeChatIcon:{
        color:`${palette.type === 'light' ? COLORS.LIGHT_GREEN:COLORS.DARK_GREEN}`
    },
    container:{
        display:'flex',
        justifyContent:'center'
    },
    header:{
        marginTop:'20px',
        fontWeight:'bold',
        fontSize:'20px',
        margin:'0'
    },
    container_body:{
        width:'90%',
        flexGrow:1,
        display:'flex',
        flexFlow:'column'
    },
    tab_container:{
        '& > div':{
            '& > span':{
                backgroundColor:`${palette.type === 'light' ? COLORS.LIGHT_GREEN:COLORS.DARK_GREEN} !important`
            }
        },
        '& .Mui-selected':{
            color:`${palette.type === 'light' ? COLORS.LIGHT_GREEN:COLORS.DARK_GREEN} !important`
        },
    },
    tab_button:{
        textTransform:'unset',
        alignItems:'flex-end',
        minWidth:'unset',
        width:'33.33%',
        '& > span':{
            fontSize:'14px',
        }
    },
    list:{
        display:'block',
        flexGrow:1,
        marginTop:'10px',
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
