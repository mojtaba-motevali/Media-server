import { IconButton } from "@material-ui/core";
import { FeaturedVideo, GridOn } from "@material-ui/icons";
import { GridType } from "../video/grid";

import { useStyles }  from './styles'
interface NavbarProps {
    setGrid:(props:GridType) => void,
    currentGrid:GridType
}

export const Navbar = ({setGrid,currentGrid}:NavbarProps) => {

    const classes = useStyles();
    const navbarIcons = [
        {value:GridType.ACTIVE_SPEAKER,Icon:<FeaturedVideo
            className={`${classes.gridIcons} ${currentGrid === GridType.ACTIVE_SPEAKER ? classes.active:''}`}
        />},
        {value:GridType.GRID_VIEW,Icon:<GridOn 
            className={`${classes.gridIcons} ${currentGrid === GridType.GRID_VIEW ? classes.active:''}`}
        />},
    ];
    const onIconClick = (value:GridType) => {
        return (e:any)=> {
            e.preventDefault();
            if(value !== currentGrid)
                setGrid(value);
        }
    }
    return (
        <div className={classes.root} >
            <div className={classes.gridSelection}>
                {
                    navbarIcons.map((icon,i)=> (
                        <IconButton key={i} onClick={onIconClick(icon.value)}>
                            {icon.Icon}
                        </IconButton>
                    ))
                }
            </div>
        </div>
    );
}
//Thanks for having me here! 😍
//Hello guys, I appreciate 