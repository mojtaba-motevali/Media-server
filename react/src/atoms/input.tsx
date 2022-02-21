import {  OutlinedInputProps, OutlinedInput } from "@material-ui/core"
import {Appear,TransitionProps} from './transitions';
import {makeStyles} from '@material-ui/core/styles'
import { forwardRef } from "react";
const styles = makeStyles(theme=>({
    default:{
        backgroundColor:'white',
        fontSize:'16px',
        marginTop:'5px',
        marginBottom:'10px',
    }
}))

interface AppInputProps extends OutlinedInputProps {
    transition?:TransitionProps
}
export const AppInput  = forwardRef(({transition,className,...rest}:AppInputProps,ref:any) => {
    const classes = styles();
    if (typeof transition !== 'undefined'){
        return (
            <Appear {...transition} >
               <OutlinedInput
                {...rest}
                ref={ref}
                className={`${classes.default} ${className}`}
            />
            </Appear>
        );
    }else {
        return (
            <OutlinedInput
                {...rest}
                ref={ref}
                className={`${classes.default} ${className}`}
            />
        );
    }
    
})