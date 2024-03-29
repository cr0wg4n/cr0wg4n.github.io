import React from 'react';

type MainGameProps =  {
  children?: React.ReactNode,
  id?: string
};

const MainGame: React.FC<MainGameProps> = ({id}: MainGameProps)=>{
  const gameScreen = `
cr0wg4n@webpage: ./build
  ...building
`;
  return <>
    <div className="
      bg-black
      w-full 
      mr-0 
      ml-auto 
      rounded-xl 
      text-green-600 
      shadow-xl 
      hover:shadow-2xl 
      font-monospace
      text-xs
      md:text-sm
      lg:text-base
      p-3
      md:p-5
    "
      {...id && {id}}
    >
      {/* <div className='text-center'>
        <pre>Starman   (space bar or touch to jump)</pre>
      </div> */}
      <pre>
        { gameScreen }
      </pre>
      <div className='text-right'>
      <a href="https://github.com/cr0wg4n" target={'_blank'} rel={'noreferrer'} className='hover:underline'>
        <pre>Made with ❤️ by cr0wg4n</pre>
      </a>
      </div>

    </div> 
  </>
}

export default MainGame;
