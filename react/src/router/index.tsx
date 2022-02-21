import {Route,Switch,BrowserRouter} from 'react-router-dom';
import Login from '../pages/login/index';
import { userContext , isMobileViewContext, themeContext} from '../contexts';
import Room from '../pages/room/index';
import { useContext, useState } from 'react';
import { createTheme, ThemeProvider, useMediaQuery } from '@material-ui/core';
import { useMemo } from 'react';
import { SnackbarProvider } from 'notistack';
const ContextRouter = ({component,MobileContext,AuthContext,...rest}:any) => {
    const Component = component;
    const [value,] = useContext(AuthContext);
    const mobileContext = useContext(MobileContext);
    return (
        <Route {...rest} render={(props)=> <Component {...props} mobileContext={{isMobile:mobileContext}} userContext={value} />} />
    )
}
const  AppRoutes = () => {
    

    const themeState = useState(false);
    const userHook = useState({id:0,name:'default',is_login:false,token:''});
    const theme = useMemo(() =>{
        return  createTheme({
            palette:{
                type:themeState[0] ? 'light':'dark'
            }
        });
    },[themeState]);
    const isMobile = useMediaQuery(theme.breakpoints.down('sm'));
    return (
            <userContext.Provider value={userHook}>
                <isMobileViewContext.Provider value={isMobile}>
                    <SnackbarProvider maxSnack={3}>

                        <BrowserRouter>    
                            <Switch>
                                <Route path="/" exact={true} component={Login}  />
                                <themeContext.Provider value={themeState}>
                                    <ThemeProvider theme={theme}>
                                        <ContextRouter path="/room/:room_id" AuthContext={userContext} MobileContext={isMobileViewContext}  exact={true} component={Room}  />
                                    </ThemeProvider>

                                </themeContext.Provider>

                            </Switch>
                        </BrowserRouter>
                    </SnackbarProvider>

                </isMobileViewContext.Provider>
            </userContext.Provider>
    )
}

export default AppRoutes;