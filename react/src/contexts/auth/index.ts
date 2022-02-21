
import {createContext,Dispatch,SetStateAction} from 'react';
import {User} from '../../@types/user';


const contextUser:User =  {
    id:0,
    name:'default',
    token:'',
    is_login:false
};
const setContextUser:Dispatch<SetStateAction<User>> = (props)=> {

};
export const userContext = createContext([contextUser,setContextUser]);