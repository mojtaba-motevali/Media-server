import { createContext, Dispatch, SetStateAction } from "react";


const setTheme:Dispatch<SetStateAction<boolean>> = (props) => {

}
export const themeContext = createContext([true,setTheme]);