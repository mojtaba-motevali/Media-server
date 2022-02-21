
import {makeStyles} from '@material-ui/core/styles';
import {Theme} from '@material-ui/core';

export const useStyles = makeStyles((theme:Theme)=> ({
    [theme.breakpoints.up('xs')]:{
        root:{
            paddingTop:'35vh',
            display:'flex',
            justifyContent:'center',
            '& p':{
                fontSize:'20px',
                fontWeight:'bold',
                paddingBottom:'5px',
                color:'white',
                textAlign:'start'
            },

        },
        input:{
            backgroundColor:'white',
            fontSize:'20px',
            marginTop:'5px',
            marginBottom:'10px'
        },
        container:{
            display:'flex',
            justifyContent:'center'
        },
        button:{
            marginTop:'2px',
            borderRadius:'20px',
            backgroundColor:'#c9022d',
            color:'white',
            width:'120px',
            height:'50px',
            '&:hover':{
                backgroundColor:"#e6204b"
            }
        }
    },
}));
