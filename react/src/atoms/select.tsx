import {  Select, SelectProps } from "@material-ui/core"
import { Appear,TransitionProps } from './transitions';
import { makeStyles, Theme } from '@material-ui/core/styles'
import { forwardRef } from "react";
import { COLORS } from "../colors";

const styles = makeStyles(({palette}:Theme)=>({
    default:{
        backgroundColor:palette.type === 'light' ? COLORS.WHITE:'black',
        borderRadius:'20px',
        border:'0',
        '& > .MuiFilledInput-input':{
            paddingTop:'15px',
        },
        '& .MuiSelect-icon':{
            color: palette.type === 'light' ? COLORS.LIGHT_BG:COLORS.LIGHT_SILVER
        },
        '& .MuiSelect-select':{
            backgroundColor:palette.type === 'light' ? COLORS.WHITE:COLORS.BLACK,
            borderRadius:'20px',
        },
        '&:hover':{
            backgroundColor:palette.type === 'light' ? COLORS.LIGHT_BG:COLORS.BLACK
        },
    },
}))

interface AppSelectProps extends SelectProps {
    transition?:TransitionProps,
}
export const AppSelect  = forwardRef(({transition,children,className,...rest}:AppSelectProps,ref:any) => {
    const classes = styles();
    
    if (typeof transition !== 'undefined'){
        return (
            <Appear {...transition} >
                <Select  {...rest} ref={ref} className={`${classes.default} ${className}`}>
                    {children}
                </Select>
            </Appear>
        );
    }else {
        return (
            <Select {...rest} ref={ref} className={`${classes.default} ${className}`}>
                   {children}
            </Select>
        )
    }
    
})