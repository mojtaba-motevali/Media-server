import { makeStyles, Theme } from '@material-ui/core';
import { COLORS } from '../../colors';
export const useStyles = makeStyles(({palette,breakpoints}:Theme)=>({
    [breakpoints.up('xs')]:{
        root:{
            backgroundColor:palette.type === 'light' ? COLORS.LIGHT_BG:COLORS.DARK_GRAY ,
            height:'100%'
        },
        header_container:{
            backgroundColor:palette.type === 'light' ? COLORS.LIGHT_BG:COLORS.DARK_GRAY,
            borderRadius:'10px 10px 0 0',
            display:'flex',
            justifyContent:'space-between'
        },
        message_list:{
            display:'block',
            height:'100%',
            maxHeight:'54vh',
            overflow:'scroll',
            overflowX:'hidden',
            '&::-webkit-scrollbar':{
                width:'6px'
            },
            '&::-webkit-scrollbar-thumb': {
                backgroundColor: palette.type === 'light' ? COLORS.LIGHT_GRAY_2:COLORS.DARK_GRAY_4,
                outline: `50px solid ${palette.type === 'light' ? COLORS.LIGHT_GRAY_2:COLORS.DARK_GRAY_4}`
            }
            
        }
    },
    [breakpoints.up('sm')]:{
        message_list:{
            maxHeight:'72vh',
        }
    },
    [breakpoints.up('md')]:{
        message_list:{
        }
    },
    [breakpoints.up('lg')]:{
        message_list:{
        }
    },

}));
