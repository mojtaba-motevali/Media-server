import {  Card, CardProps } from "@material-ui/core"
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

interface AppCardProps extends CardProps {
    transition?:TransitionProps
}
export const AppCard  = forwardRef(({transition,children,className,...rest}:AppCardProps,ref:any) => {
    const classes = styles();
    if (typeof transition !== 'undefined'){
        return (
            <Appear {...transition} >
               <Card
                {...rest}
                ref={ref}
                className={`${classes.default} ${className}`}
            >
                {children}
            </Card>
            </Appear>
        );
    }else {
        return (
            <Card
                {...rest}
                ref={ref}
                className={`${classes.default} ${className}`}
            >
                 {children}
            </Card>
        );
    }
    
})