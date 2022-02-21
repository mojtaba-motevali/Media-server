import { IconButton } from "@material-ui/core";
import { NightsStay, WbSunny } from "@material-ui/icons";
import { useContext } from "react";
import { themeContext } from '../../contexts/'


export const ThemeChanger = () => {
    const [theme,setTheme]:any = useContext(themeContext);

    return (

        <IconButton onClick={()=> {
            setTheme(!theme);
       }} > { theme ? <WbSunny style={{color:'#ffae00'}} /> : <NightsStay/> } </IconButton>
    );
}