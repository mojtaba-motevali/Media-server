import {  Avatar, AvatarProps } from "@material-ui/core"
import {Appear,TransitionProps} from './transitions';
import {makeStyles} from '@material-ui/core/styles'
import { forwardRef } from "react";
import { COLORS } from "../colors";
const styles = makeStyles(theme=>({
    default:{
        backgroundColor:theme.palette.type === 'light' ? COLORS.BLUE:COLORS.DARK_GRAY,
        color:theme.palette.type === 'light' ? COLORS.WHITE:COLORS.WHITE
    },
    gradient:{
        border: "double 4px transparent",
        borderRadius:'80px',
        backgroundImage:'linear-gradient(white, white), radial-gradient(circle at top left, #f00,#3020ff)',
        backgroundOrigin:'border-box',
        backgroundClip:'content-box, border-box',
    }
}))

interface AppAvatarProps extends AvatarProps {
    transition?:TransitionProps,
    firstWord?:string,
    useBorder:boolean,
    borderClassName?:string;
}
/**
 * user should use Ref to style inner component
 */
export const AppAvatar  = forwardRef(({style,borderClassName,useBorder,firstWord,transition,src,className,...rest}:AppAvatarProps,ref:any) => {
    const classes = styles();
    if (typeof transition !== 'undefined'){
        return (

            <Appear {...transition} >
                {useBorder ? (
                    <div style={style} className={`${classes.gradient} ${borderClassName}`}>
                        <Avatar src={src} {...rest} ref={ref} className={`${classes.default}`}>
                                {!src && firstWord}
                        </Avatar>
                    </div>

                ):(
                    <Avatar style={style} src={src} {...rest} ref={ref} className={`${classes.default} ${className}`}>
                            {!src && firstWord}
                    </Avatar>
                )}
            </Appear>
        );
    }else {
        return  useBorder ? (
            <div style={style} className={`${classes.gradient} ${borderClassName}`}>
                <Avatar src={src} {...rest} ref={ref} className={`${classes.default} ${className}`}>
                        {!src && firstWord}
                </Avatar>
            </div>

        ):(
            <Avatar style={style} src={src} {...rest} ref={ref} className={`${classes.default} ${className}`}>
                    {!src && firstWord}
            </Avatar>
        )
    }
    
})