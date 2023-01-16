import React from "react";
import {FaNewspaper, FaStar, FaTerminal, FaUser} from 'react-icons/fa'
import {HiOutlinePresentationChartBar, } from 'react-icons/hi'



const Control: React.FC = () => {
  return <div className='fixed z-10 right-3 top-1/2 w-10 -mt-24 bg-gray-900 rounded-full flex flex-col justify-center items-center p-1 opacity-75 hover:opacity-100 hover:scale-110'>
    <div className='h-10 flex items-center hover:scale-150 hover:cursor-pointer hover:text-orange-500 active:scale-150'>
      <a href="#profile">
        <FaUser/>
      </a>
    </div>
    <div className='h-10 flex items-center hover:scale-150 hover:cursor-pointer hover:text-yellow-500 active:scale-150'>
      <a href="#top-repos">
        <FaStar />
      </a>
    </div>
    <div className='h-10 flex items-center hover:scale-150 hover:cursor-pointer hover:text-green-400 active:scale-150'>
      <a href="#slides">
        <HiOutlinePresentationChartBar />
      </a>
    </div>
    <div className='h-10 flex items-center hover:scale-150 hover:cursor-pointer hover:text-blue-700 active:scale-150'>
      <a href="#articles">
        <FaNewspaper />
      </a>
    </div>
    <div className='h-10 flex items-center hover:scale-150 hover:cursor-pointer hover:text-green-900 active:scale-150'>
      <a href="#terminal">
        <FaTerminal />
      </a>
    </div>
  </div>
}

export default Control;
