import { Paper ,PaperProps } from "@material-ui/core"
import {Appear,TransitionProps} from './transitions';
import {makeStyles} from '@material-ui/core/styles'
import { forwardRef } from "react";
const styles = makeStyles(theme=>({
    default:{
        borderRadius:'20px',
        '& > div':{
            padding:'5px'
        }
    }
}))

interface AppPropsProps extends PaperProps {
    transition?:TransitionProps
}
export const AppPaper  = forwardRef(({transition,children,className,...rest}:AppPropsProps,ref:any) => {
    const classes = styles();
    if (typeof transition !== 'undefined'){
        return (
            <Appear {...transition} >
                <Paper {...rest} ref={ref} className={`${classes.default} ${className}`}>
                    {children}
                </Paper>
            </Appear>
        );
    }else {
        return (
            <Paper
                {...rest}
                ref={ref}
                className={`${classes.default} ${className}`}
            >
                {children}
            </Paper>
        );
    }
    
})