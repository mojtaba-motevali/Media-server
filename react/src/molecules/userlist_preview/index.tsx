import { Container, FormControl, IconButton, MenuItem, Tooltip } from "@material-ui/core";
import { useStyles } from './styles';
import { PeerManager } from '../../pages/room/managers';
import { AppAvatar } from "../../atoms/avatar";
import { Dispatch, SetStateAction, useCallback, useContext, useEffect, useState } from "react";
import { isMobileViewContext, shouldReRenderContext } from "../../contexts";
import { AppSelect } from "../../atoms";
import { ViewComfy, Group, Chat , ArrowBack } from '@material-ui/icons';
import { ActiveComponent, ActiveComponentType } from "../../organisms/activity/active_components";

interface UserListPreviewProps {
    peers:Map<number,PeerManager>,
    value:number,
    setValue:Dispatch<SetStateAction<ActiveComponentType>>
}
export const UserListPreview = ({peers,setValue,value}:UserListPreviewProps) => {
    const classes = useStyles();
    const shouldRender = useContext(shouldReRenderContext);
    const isMobile = useContext(isMobileViewContext);
    const [,render] = useState(0);
    useEffect(()=> {
        const userListPreviewRenderer = ()=> {
            render((prev) => prev+1);
        }
        shouldRender.on("PeerManager",userListPreviewRenderer);
        return () => {
            shouldRender.removeListener("PeerManager",userListPreviewRenderer);
        }
    },[shouldRender]);
    const handleOnChange = useCallback((e:any)=> {
        const eventValue:ActiveComponent.PUBLIC_CHAT |
        ActiveComponent.USER_LIST | ActiveComponent.VIDEO_SELECTION = e.target.value;
        setValue({
            state:eventValue,
            data:{}
        });
    },[setValue])
    const styles = {marginRight:'3px',marginLeft:'2px'};
    const menuItems = [
        {value:ActiveComponent.PUBLIC_CHAT ,title:'Group Chat',
        icon:<Chat  className={`${classes.icon} ${value === ActiveComponent.PUBLIC_CHAT ? classes.iconSelected:''}`}/>},
        {value:ActiveComponent.USER_LIST ,title:'User List',
        icon: <Group className={`${classes.icon} ${value === ActiveComponent.USER_LIST ? classes.iconSelected:''}`}/>},
        {value:ActiveComponent.VIDEO_SELECTION,title:'Video Selection',
        icon:<ViewComfy className={`${classes.icon} ${value === ActiveComponent.VIDEO_SELECTION ? classes.iconSelected:''}`}/>}
    ];
    return (
        <Container className={classes.root}>
                {
                    value === ActiveComponent.PRIVATE_CHAT ? <IconButton onClick={e=>{
                        setValue({state:ActiveComponent.USER_LIST,data:{}})
                    }}>
                            <ArrowBack/>
                    </IconButton>: 
                    <FormControl variant="filled" className={classes.formControl}> 
                        <AppSelect value={value} className={classes.select}
                            onChange={handleOnChange}>
                                {menuItems.map((item,i) => <MenuItem key={i} value={item.value}>
                                            <Tooltip arrow placement="right"  style={{fontSize:20}} title={item.title}>
                                                <div className={classes.selectItem} >
                                                    {item.icon}
                                                </div>
                                            </Tooltip>
                                        </MenuItem>
                                ) }
                            </AppSelect>
                    </FormControl>
                }
             
            <div className={classes.avatarContainer}>
                {
                    Array.from(peers.values()).slice(0,isMobile ? 2:3).map((peer,i)=>  
                    <AppAvatar key={i} style={{...styles}}
                        useBorder={true}
                        src={peer.avatar}
                        firstWord={peer.name.charAt(0).toUpperCase()}/>)
                }
                {
                    peers.size > 3 && <AppAvatar className={classes.moreAvatar}
                        useBorder={false}
                        firstWord={"+" + (peers.size-(isMobile ? 2:3)).toString()}
                        />
                }
            </div>
        </Container>
    );
}