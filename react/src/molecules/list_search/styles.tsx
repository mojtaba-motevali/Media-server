import { makeStyles, Theme } from '@material-ui/core';
import { COLORS } from '../../colors';

export const useStyles = makeStyles(({palette}:Theme)=>({
    root:{
        height:'100%',
        width:'100%',
        display:'flex',
        flexFlow:'column',
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
    list:{
        display:'block',
        flexGrow:1,
        marginTop:'10px',
    },
    autoSizer:{
        overflow:'none',
        '& > div':{
            '&::-webkit-scrollbar':{
                width:'4px'
            },
            '&::-webkit-scrollbar-track': {
                boxShadow: '10px 0 0 6px  #d3d1e8',
                webkitBoxShadow: 'inset 0 0 6px #d3d1e8'
            },
            '&::-webkit-scrollbar-thumb': {
                backgroundColor: '#d3d1e8',
                outline: '80px solid #d3d1e8'
            }
        }
    },
    searchInput:{
        marginLeft:'5px',fontSize:'12px',
    },
    search:{
        marginTop:'20px',
        width:'100%',
        borderRadius:'10px',
        backgroundColor:palette.type === 'light' ? COLORS.WHITE:COLORS.DARK_GRAY_3,
        border:`1px solid ${palette.type === 'light' ? COLORS.LIGHT_GREEN_2:COLORS.LIGHT_GRAY}`,
        '& > input':{
            padding:'0'
        },
        '&:focus-within':{
            borderColor: palette.type === 'light' ? COLORS.LIGHT_GREEN:COLORS.DARK_GREEN
        }
    },
    searchIcon:{
        color:palette.type === 'light' ? COLORS.LIGHT_GREEN:COLORS.DARK_GREEN,
        fontSize:'20px'
    },
    searchIconButton:{
        padding:'8px',
        paddingRight:'5px'
    }
}));
