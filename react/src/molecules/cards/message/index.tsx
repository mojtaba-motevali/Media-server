import { CardContent, Container, Grid } from '@material-ui/core';
import { AppAvatar, AppCard, AppTypography } from '../../../atoms/';
import {useStyles} from './styles'

export interface MessageCardProps {
    userId:number,
    isLeft:boolean,
    avatar?:string,
    name:string,
    messages:{
        timestamp:Date,
        text:string,
    }[],

}

export const MessageCard = ({isLeft,avatar,name,messages}:MessageCardProps) => {
    const classes = useStyles();
    return (
        <Grid spacing={3} direction={isLeft ? 'row':'row-reverse'}  container>
            <Grid xs={4} sm={2}   item>
                <AppAvatar borderClassName={classes.avatar} className={classes.avatar} 
                useBorder={true} firstWord={name.charAt(0).toUpperCase()} src={avatar}/> 
            </Grid>
            <Grid  xs={4} md={10} style={{alignItems:isLeft ? 'flex-start':'flex-end'}}  className={classes.textGrid} item>
                {
                    messages.map( (messageObject,i)=> {
                        const {timestamp,text} = messageObject;
                        const hour = timestamp.getHours();
                        const min = timestamp.getMinutes(); 
                        return (
                        <AppCard transition={{type:'Fade',duration:1000}} key={i} 
                        className={`${classes.card} ${isLeft ? classes.peerBackgroundCard: classes.selfBackgroundCard}`} >
                            <CardContent className={classes.cardContent}>
                                <Container style={{padding:'8px',paddingBottom:'3px'}}>
                                    <AppTypography className={classes.message} >
                                            {text.trim()}
                                    </AppTypography>
                                </Container>
                                <Container style={{textAlign:'end',paddingBottom:'2px'}}>
                                    <AppTypography className={classes.messageTime}>
                                            {`${hour}:${min}`}
                                    </AppTypography>
                                </Container>
                            </CardContent>
                        </AppCard>
                        )
                    })
                }
            </Grid>
        </Grid>
    );
}