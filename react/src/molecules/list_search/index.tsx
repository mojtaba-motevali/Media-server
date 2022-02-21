

import { AppSearchInput } from "../../atoms";
import { FixedSizeList, ListChildComponentProps } from 'react-window';
import AutoSizer from 'react-virtualized-auto-sizer';
import { FunctionComponent } from "react";
import { useStyles } from "./styles";

interface SearchListProps {
    renderList:FunctionComponent<ListChildComponentProps<any>>,
    listCount:number,
    value:string,
    onChange:(e:React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void
}
export const SearchList = ({renderList,listCount,value,onChange}:SearchListProps) => {
    const classes = useStyles();
    return (
        <div style={{height:'100%',display:'contents'}}>
            <AppSearchInput inputProps={{ className:classes.searchInput }}
                    placeholder="Search people"  className={classes.search}
                    value={value}
                    onChange={onChange}
                    searchIconClasName={classes.searchIcon} iconButtonClassName={classes.searchIconButton}
                />
            <div className={classes.list}>
                    <AutoSizer className={classes.autoSizer}>
                        {({height,width}) => (
                            <FixedSizeList style={{overflowX:'hidden'}} height={height} width={width} itemSize={75}  itemCount={listCount}>
                                {renderList}
                            </FixedSizeList>
                        )}
                    </AutoSizer>
            </div>

        </div>
    )
}