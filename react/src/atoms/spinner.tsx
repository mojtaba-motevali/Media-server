import {  CircularProgress,CircularProgressProps } from "@material-ui/core"
import {Appear,TransitionProps} from './transitions';
import {makeStyles} from '@material-ui/core/styles'
import { forwardRef } from "react";
const styles = makeStyles(theme=>({
    default:{

    }
}))

interface AppSpinnerProps extends CircularProgressProps {
    transition?:TransitionProps
}
export const AppSpinner  = forwardRef (({transition,className,...rest}:AppSpinnerProps,ref:any) => {
    const classes = styles();
    if (typeof transition !== 'undefined'){
        return (
            <Appear {...transition} >
               <CircularProgress  size={40} thickness={4} value={100}   {...rest} ref={ref}  className={`${classes.default} ${className}`}/>
            </Appear>
        );
    }else {
        return (
            <CircularProgress  size={40} thickness={4} value={100} {...rest} ref={ref} className={`${classes.default} ${className}`}/>
        );
    }
    
})