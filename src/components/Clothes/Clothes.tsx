import React, { useState } from 'react';
import data from '../../data/repositories'

const Clothes: React.FC = () => {
  const respositoryData = data()
  const [query, setQuery] = useState('')
  const [repositories, setRepositories] = useState(respositoryData);

  function handleQueryChange(event: React.ChangeEvent<HTMLInputElement>) {
    setQuery(event.target.value)
    if(event.target.value === ''){
      setRepositories(respositoryData)
      return
    }
    setRepositories(
      respositoryData.filter(
        ({name})=>(name.toLowerCase()).includes(query.toLowerCase())
      )
    )
  }

  return <div className=''>
    {
      respositoryData.length > 5 && 
      <div className="relative w-full md:w-2/5 mt-4">
        <div className="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
          <svg aria-hidden="true" className="w-5 h-5 text-gray-500 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
        </div>
        <input 
          type="search" 
          id="default-search" 
          className="block w-full p-3 pl-10 text-sm text-gray-900 rounded-lg bg-gray-50 focus:ring-blue-500" 
          placeholder="Search repository..."
          onChange={handleQueryChange}
        />
        {/* <button 
          type="submit"
          className="text-white absolute right-1 bottom-1 bg-blue-500 hover:bg-blue-700 focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 active:scale-105"
        >
          Search
        </button> */}
      </div>
    }


    <div className='overflow-x-auto flex pb-2 mt-4'>
      {
        repositories.map(({name, openGraphImageUrl, url}, index) =>
        <div className='flex-none w-64 h-32 md:w-80 md:h-40 bg-white mr-1 hover:my-2' key={index}>
          <a href={url} rel='noreferrer noopener' target={'_blank'}>
            <img src={openGraphImageUrl} alt={name} />
          </a>
        </div>)
      }
    </div>
  </div>
}

export default Clothes;
