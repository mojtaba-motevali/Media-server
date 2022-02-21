import { makeStyles, Theme } from '@material-ui/core';

export const useStyles = makeStyles((theme:Theme)=>({
    [theme.breakpoints.up('xs')]:{
        activeSpeaker:{
            width:'320px',
            margin:'auto'
        },
    },
    [theme.breakpoints.up('sm')]:{
        activeSpeaker:{
            width:'720px',
        },
    },
    [theme.breakpoints.up('md')]:{

    },
    [theme.breakpoints.up('lg')]:{

    },

}));
