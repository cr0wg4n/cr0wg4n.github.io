import React from "react";
import {classNames} from '../../utils';

type SimpleBoxProps = {
  children: React.ReactNode,
  firstColor: string,
  secondColor: string,
  title?: string,
  orientation?: boolean
}

const SimpleBox: React.FC<SimpleBoxProps> = ({children,firstColor,secondColor,title, orientation}: SimpleBoxProps)=>{
  const optOrientation = orientation?'mr-0 ml-auto':'';
  const optTextOrientation = !orientation?'text-left':'text-right';
  const fColor = `from-${firstColor}`;
  const sColor = `to-${secondColor}`;
  const className=classNames(
    'bg-gradient-to-t',
    fColor,
    sColor,
    optOrientation,
    `w-12/12 sm:w-12/12 md:w-10/12 xl:w-8/12 mb-10 rounded-xl shadow-xl hover:shadow-2xl`
  );
  
  return (
    <div className={className}>
      {
        title &&
        <div className={'font-medium text-xl md:text-3xl p-10 text-white '+optTextOrientation}>
          {title}
        </div>
      }
      {children}
    </div>
  )
}


export default SimpleBox;
