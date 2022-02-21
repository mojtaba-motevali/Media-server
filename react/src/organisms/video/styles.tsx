import { makeStyles, Theme } from '@material-ui/core';

export const useStyles = makeStyles((theme:Theme)=>({
    [theme.breakpoints.up('xs')]:{
        root:{
            width:'100%',
            height:'100%'
        },
        activeSpeakerVideo:{
            marginTop:'5px',
            display:'flex',
            height:'240px',
            flexFlow:'column',
            alignItems:'center',
            backgroundColor:'blue'
        },
        peerVideo:{
            height:'240px'
        },
        iconButton:{
            backgroundColor:`rgb(58, 69, 87,0.4)`,
            // border:`3px solid ${COLORS.RED}`,
            marginRight:'30px',
            marginLeft:'30px',
        },
        buttonsContainer:{
            position:'absolute',
            top:'90%',
            right:'50%',
            left:'50%',
            display:'flex',
            justifyContent:'center',
        },
        svg:{
            color:'white',
            width:'35px',
            height:'35px'
        }
    },
    [theme.breakpoints.up('sm')]:{
        activeSpeakerVideo:{
            height:'350px',
        },

    },
    [theme.breakpoints.up('md')]:{
        activeSpeakerVideo:{
            height:'520px',
        },
    },
    [theme.breakpoints.up('lg')]:{
    },

}));
