import {  InputBaseProps,InputBase, IconButton  } from "@material-ui/core"
import {Appear,TransitionProps} from './transitions';
import { makeStyles} from '@material-ui/core/styles'
import { forwardRef } from "react";
import { Search } from "@material-ui/icons";
const styles = makeStyles(theme=>({
    default:{
        fontSize:'16px',
    },
}))

interface AppSearchInputProps extends InputBaseProps {
    transition?:TransitionProps,
    searchIconClasName?:any,
    iconButtonClassName?:any
}
export const AppSearchInput  = forwardRef(({iconButtonClassName,searchIconClasName,transition,className,...rest}:AppSearchInputProps,ref:any) => {
    const classes = styles();
    if (typeof transition !== 'undefined'){
        return (
            <Appear {...transition} >
               <InputBase
                {...rest}
                ref={ref}
                className={`${classes.default} ${className}`}
                endAdornment={
                    <IconButton className={iconButtonClassName}><Search className={`${searchIconClasName ? searchIconClasName:""}`}/></IconButton>
                }
            />
            </Appear>
        );
    }else {
        return (
            <InputBase
            {...rest}
            ref={ref}
            className={`${classes.default} ${className}`}
            endAdornment={
                <IconButton className={iconButtonClassName}><Search className={`${searchIconClasName ? searchIconClasName:""}`}/></IconButton>
            }
        />
        );
    }
    
})