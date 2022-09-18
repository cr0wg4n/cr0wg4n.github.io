import React from 'react';

const MainGame: React.FC = ()=>{
  const gameScreen = 'cr0wg4n@demo: demo \n demo\n\n\n\n\n\n\n';
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
    ">
      <div className='text-center'>
        <pre>Starman   (space bar or touch to jump)</pre>
      </div>
      <pre>
        { gameScreen }
      </pre>
      <a href="https://github.com/cr0wg4n" target={'_blank'} rel={'noreferrer'} className='hover:underline'>
        <div className='text-right'>
          <pre>Made with ❤️ by cr0wg4n</pre>
        </div>
      </a>
    </div> 
  </>
}

export default MainGame;
