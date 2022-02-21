import * as PropTypes from 'prop-types';
import Grow from '@material-ui/core/Grow';
import { ReactElement } from 'react';
import { createRef } from 'react';
import { Collapse, Fade, Slide, Zoom } from '@material-ui/core';

export interface TransitionProps {
	duration?:number,
	type?:"Grow" | "Fade" | "Collapse" | "Slide" | "Zoom"
}
interface AppearProps extends TransitionProps {
	children:ReactElement<any, any>
}
const Appear = ({ duration, children,type }:AppearProps) => {
	const ref = createRef();
	const props = {
		mountOnEnter:true,unmountOnExit:true,
		ref:ref,in:true,exit:true,timeout:duration || 500
	}
	let Wrapper;
	switch(type) {
		case 'Collapse':
			Wrapper = (<Collapse {...props}>{children}</Collapse>);
			break;
		case 'Fade':
			Wrapper = (<Fade {...props}>{children}</Fade>);
			break;
		case 'Grow':
			Wrapper = (<Grow {...props}>{children}</Grow>);
			break;
		case 'Slide':
			Wrapper = (<Slide {...props}>{children}</Slide>);
			break;
		case 'Zoom':
			Wrapper = (<Zoom {...props}>{children}</Zoom>);
			break;
		default:
			throw new Error("INVALID OPTION ON TRANSITION")
	}
	return  Wrapper
}

Appear.propTypes =
{
	duration : PropTypes.number,
	children : PropTypes.any
};

export { Appear };