import React from "react";
import {classNames} from '../../utils';

type SimpleBoxProps = {
  children: React.ReactNode,
  id?: string,
  firstColorClass: string,
  secondColorClass: string,
  title?: string,
  orientation?: boolean
}

const SimpleBox: React.FC<SimpleBoxProps> = ({
  id,
  children,
  firstColorClass,
  secondColorClass,
  title,
  orientation=true
}: SimpleBoxProps)=>{
  // const [className,setClassName]= React.useState('');
  const optTextOrientation = orientation?'text-left':'text-right';

  const optOrientation = orientation?'mr-0 ml-auto':'';
  const className = (
    classNames(
        'bg-gradient-to-t',
        firstColorClass,
        secondColorClass,
        optOrientation,
        `w-12/12 sm:w-12/12 md:w-10/12 xl:w-8/12 mb-4 md:mb-10 rounded-xl shadow-xl hover:shadow-2xl p-3 lg:p-5`
    )
  );

  return (
    <div className={className} {...id && {id}}>
      {
        title &&
        <div className={'font-medium text-xl md:text-2xl text-white '+optTextOrientation}>
          {title}
        </div>
      }
      {children}
    </div>
  )
}


export default SimpleBox;
