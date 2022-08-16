import React from 'react';
import { IconContext } from 'react-icons';

type IconItemProps =  {
    children: React.ReactNode
};

const IconItem: React.FC<IconItemProps> = ({children}:IconItemProps)=>{
    return (
        <IconContext.Provider value={{
            color:'white',
            size:'1.3rem',
            className:'hover:cursor-pointer hover:scale-125'
        }}> 
            {children}
        </IconContext.Provider>
    );
}

export default IconItem; 
