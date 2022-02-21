import { PopoverProps, Popover } from "@material-ui/core"
import {Appear,TransitionProps} from './transitions';
import {makeStyles} from '@material-ui/core/styles';
import { forwardRef } from "react";

const styles = makeStyles(theme=>({
    default:{
        '& div':{
            borderRadius:'5px',
        },
        '& *':{
            padding:'5px'
        } 
    }
}))

interface AppPopoverProps extends PopoverProps {
    transition?:TransitionProps
}
export const AppPopover  = forwardRef(({children,transition,className,...rest}:AppPopoverProps,ref:any) => {
    const classes = styles();
    if (typeof transition !== 'undefined'){
        return (
            <Appear {...transition} >
                <Popover {...rest} ref={ref} className={`${classes.default} ${className}`}>
                    {children}
                </Popover>
            </Appear>
        );
    }else {
        return (
            <Popover {...rest} ref={ref} className={`${classes.default} ${className}`}>
                {children}
            </Popover>
        );
    }
    
})