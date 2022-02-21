import {  Typography,TypographyProps } from "@material-ui/core"
import {Appear,TransitionProps} from './transitions';
import {makeStyles} from '@material-ui/core/styles'
import { forwardRef } from "react";
const styles = makeStyles(theme=>({
    default:{
        
    }
}))

interface AppTypographyProps extends TypographyProps {
    transition?:TransitionProps
}
export const AppTypography  = forwardRef (({children,transition,className,...rest}:AppTypographyProps,ref:any) => {
    const classes = styles();
    if (typeof transition !== 'undefined'){
        return (
            <Appear {...transition} >
               <Typography
                {...rest}
                ref={ref}
                className={`${classes.default} ${className}`}
            >
                {children}
            </Typography>
            </Appear>
        );
    }else {
        return (
            <Typography
                {...rest}
                ref={ref}
                className={`${classes.default} ${className}`}
            >
                {children}
            </Typography>
        );
    }
    
})