import { Container, IconButton } from "@material-ui/core";
import { LocalActivity, Videocam } from "@material-ui/icons";
import { useContext } from "react";
import { isMobileViewContext } from "../../contexts";
import { ActiveParts } from "../../pages/room/ui_types/active_parts";
import { useStyles } from './styles'

interface NavigationProps {
    activePars:ActiveParts[],
    setActiveParts:(props:ActiveParts[]) => void
}

export const Navigation = ({activePars,setActiveParts}:NavigationProps) => {
    const classes = useStyles();
    const isMobile = useContext(isMobileViewContext);
    const icons = [
        {value:ActiveParts.VIDEO,icon:<Videocam
            className={`${classes.icon} ${activePars.includes(ActiveParts.VIDEO) ? classes.active:''}`} 
            />},
        {value:ActiveParts.ACTIVITY,icon:<LocalActivity
            className={`${classes.icon} ${activePars.includes(ActiveParts.ACTIVITY) ? classes.active:''}`} 
        />},
    ];
    const onClick = (value:ActiveParts) => {
        return () => {
            const isIncluded = activePars.includes(value);
            if(isMobile){   
                if(isIncluded){
                    return;
                }else {
                    setActiveParts([value]);
                }
            }else {
                if(value === ActiveParts.VIDEO){
                    if(isIncluded){
                        return;
                    }else {
                        setActiveParts([...activePars,value]);
                    }
                }else {
                    if(isIncluded){
                        setActiveParts(activePars.filter(part => part !== value));
                    }else {
                        setActiveParts([...activePars,value]);
                    }
                }
            }
        }
    }
    return (
        <Container className={isMobile ? classes.row:classes.column} >
            {
                icons.map((icon,i)=> (
                    <IconButton  className={classes.iconButton} key={i} onClick={onClick(icon.value)}>
                        {icon.icon}
                    </IconButton>
                ))
            }
        </Container>
    );
}