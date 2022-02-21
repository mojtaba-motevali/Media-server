
import {makeStyles} from '@material-ui/core/styles';
import {Theme} from '@material-ui/core';

export const useStyles = makeStyles((theme:Theme)=> ({
    [theme.breakpoints.up('xs')]:{
        typography:{
            fontSize:'20px',
            fontWeight:'bold',
            paddingBottom:'5px',
            color:'white',
            textAlign:'start'
        },
        forgetPass:{
            marginTop:'10px',
            cursor:'pointer',
            fontSize:'18px',
            color:'white',
            '&:hover':{
                color:'#ebe4e4'
            }
        },
        formControl:{
            display:'block',
        },
        input:{
            backgroundColor:'white',
            fontSize:'20px',
            marginTop:'5px',
            marginBottom:'10px',
        },
        container:{
            display:'flex',
            justifyContent:'space-between'
        },
        button:{
            marginTop:'2px',
            borderRadius:'20px',
            marginRight:'auto',
            marginLeft:'auto',
            justifySelf:'center',
            backgroundColor:'#5118de',
            color:'white',
            width:'100px',
            height:'50px',
            '&:hover':{
                backgroundColor:"#1853de"
            },
            '&:disabled':{
                backgroundColor:'#1e0659',
                color:'gray'
            }
        }
    },
    [theme.breakpoints.up('md')]:{
        root:{
            '& p':{
                fontSize:'25px',
            },

        },
       input:{
            width:'500px'
       }
    }
}));
