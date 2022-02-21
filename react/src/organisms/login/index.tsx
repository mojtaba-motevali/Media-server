import {useState} from 'react';
import { useStyles } from './styles';
import { LoginForm, RoomSelection } from '../../molecules/forms';

export const Login = (props:any) => {
    const [stage,setStage]:[number,Function] = useState(1);
    const classes = useStyles();
    return (
        <div className={classes.root}>
            <div className={classes.container}>
                <div>
                    { 
                        stage === 1 ?  <LoginForm setStage={setStage} /> :
                        <RoomSelection history={props.history}/> 
                    }
                </div>

            </div>
        </div>
    );
}
