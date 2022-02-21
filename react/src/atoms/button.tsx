import { Button, ButtonProps } from "@material-ui/core"
import {Appear,TransitionProps} from './transitions';
import {makeStyles} from '@material-ui/core/styles'
import { forwardRef } from "react";
const styles = makeStyles(theme=>({
    default:{
        outline:'none',
        borderRadius:'20px',
        color:'white',    
        textTransform:'uppercase',
    }
}))

interface AppButtonProps extends ButtonProps {
    transition?:TransitionProps
}
export const AppButton  = forwardRef(({children,transition,className,...rest}:AppButtonProps,ref:any) => {
    const classes = styles();
    if (typeof transition !== 'undefined'){
        return (
            <Appear {...transition} >
                <Button {...rest} ref={ref} className={`${classes.default} ${className}`}>
                    {children}
                </Button>
            </Appear>
        );
    }else {
        return (
            <Button {...rest} ref={ref} className={`${classes.default} ${className}`}>
                {children}
            </Button>
        );
    }
    
})