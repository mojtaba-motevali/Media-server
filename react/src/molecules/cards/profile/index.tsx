import {  Container, Grid } from "@material-ui/core";
import { useRef } from "react";
import { useEffect } from "react";
import { useMemo } from "react";
import { AppAvatar, AppTypography } from "../../../atoms";
import { useStyles } from './styles';
import { ProfileCardProps } from './types';
export const ProfileCard = ({isLeft,isSelf,avatar,title,subTitle,MagicComponent}:ProfileCardProps) => {
    const classes = useStyles();
    const firstWord = useMemo(()=>[title.charAt(0).toUpperCase(),title.slice(1)],[title])
    const avatarRef = useRef({className:''});
    useEffect(()=>{
        const {current} =avatarRef;
        if(current !== null){
            current.className = `${current.className} ${classes.avatar}`
        }
    },[classes.avatar])
    return (
        <Container className={classes.container}>
            <Grid direction={isLeft ? 'row':"row-reverse"} spacing={4} container >
                <Grid style={{justifyContent:'center',display:'flex'}} xs={2} item>
                    <AppAvatar ref={avatarRef} useBorder={true} firstWord={firstWord[0]} src={avatar}/>
                </Grid>
                <Grid xs={6} style={{textAlign:isLeft ? 'start':'end',placeSelf:typeof isSelf !== 'undefined' ? 'center':'start'}}   item>
                        {
                            isSelf ? isSelf :(
                                <div>
                                    <AppTypography  className={classes.title}>
                                        { firstWord[0] + firstWord[1] }
                                    </AppTypography>
                                    <AppTypography style={ isLeft ? {marginLeft:'5px'}:{marginRight:'5px'}} className={classes.subtitle}>
                                            {subTitle}
                                    </AppTypography>
                                </div>
                            )
                        }
                </Grid>
                <Grid style={{justifyContent:'center',placeSelf:'center'}} xs={4} item>
                    {MagicComponent}
                </Grid>
            </Grid>
        </Container>
    );
}