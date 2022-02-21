import { createStyles, makeStyles, Theme } from '@material-ui/core';
import { COLORS } from '../../colors';

export const useStyles = makeStyles(({palette,spacing}:Theme)=>
    createStyles({
        root:{
            marginTop:'30px',
            marginBottom:'15px',
            display:'flex',
            justifyContent:'space-between',
            padding:'0'
        },
        avatarContainer:{
            display:'inline-flex',
            justifyItems:'center',
            placeItems:'center'
        },
        moreAvatar:{
            backgroundColor: palette.type === 'light' ? COLORS.WHITE: COLORS.DARK_GRAY_2 + '!important',
            color: palette.type === 'light' ? COLORS.BLACK :COLORS.LIGHT_SILVER + '!important',
            width:'50px',
            height:'50px',
            marginLeft:'5px',
            marginRight:'5px'
        },
        formControl: {
            width:'100px',
            borderRadius:'20px',
            placeItems:'center',
            border:'0',
            '& .MuiFilledInput-underline':{
                '&:before':{
                    border:'0',
                },
                '&:after':{
                    border:'0'
                }
            },
            backgroundColor:palette.type === 'light' ? COLORS.WHITE:COLORS.BLACK
        },
        icon:{
            width:'25px',
            height:'25px',
        },
        selectItem:{
            display:'flex',
            width:'100%',
            height:'100%',
            justifyContent:'center',
            padding:'5px 0px',
        },
        iconSelected:{
            color:palette.type === 'light' ? COLORS.LIGHT_GREEN:COLORS.DARK_GREEN
        },
        select:{
            width:'100%',
            justifyContent:'center'
        },
        selectEmpty: {
          marginTop: spacing(2),
        },
      }),
);
