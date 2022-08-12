import './App.css';

function App() {
  return (
    <div className='bg-blue-gray-900 p-10 font-serif'>
      <div 
        //  from-cyan-500 to-blue-500
        // className="bg-deep-orange-800 text-blue-gray-50 w-6/12 mb-10 rounded-xl shadow-xl hover:shadow-2xl"
        className="bg-gradient-to-t from-deep-orange-900 to-deep-orange-500 text-blue-gray-50 w-6/12 mb-10 rounded-xl shadow-xl hover:shadow-2xl"
      >
        <div className="grid grid-cols-12 gap-1 pl-10 py-10">
          <div className='col-span-1 m-auto'>
            a 
            <br />
            a 
            <br />
            a 
            <br />
            a 
          </div>
          <div className='m-auto col-span-5'>
            <img 
              src='/images/mauri.jpeg'
              alt='Mauricio Matias, cr0wg4n' 
              width={'240'} 
              height={'240'}
              className="rounded-full"
            />
          </div>
          <div className='my-auto mr-10 col-span-6'>
            <div className='font-normal text-normal text-left text-blue-gray-50'>
              Who am I?
            </div>
            <div className='font-medium text-4xl'>
              Mauricio Matias C.
            </div>
            <div className='font-normal text-lg text-left text-blue-gray-50'>
              Software developer
            </div>
            <br />
            <div className='font-normal text-sm text-center text-white font-sans'>
              Hi, I'm Mauricio. Building and design technology is my passion. 
              Actually, I looking for systems programming and
              <span className='text-light-blue-200'>
                &nbsp;
                <a href="https://www.rust-lang.org/" rel='nofollow noopener'>@rust</a> 
                &nbsp;
              </span>
              for fun. I love and follow 
              the Open Source and Free Sofware initiatives. I occasionally instruct 
              programming and share my day-to-day tech experience as Software Developer through talks.
            </div>
          </div>
        </div>
      </div>
      <div className="bg-teal-700 h-128 w-9/12 mr-0 ml-auto mb-10 rounded-xl shadow-xl hover:shadow-2xl">

      </div>
      <div className="bg-deep-purple-700  h-128 w-9/12 mb-10 rounded-xl shadow-xl hover:shadow-2xl">
      </div>

      <div className="bg-black h-48 w-9/12 mr-0 ml-auto rounded-xl text-white shadow-xl hover:shadow-2xl p-10">
        
      </div>
    </div>
  );
}

export default App;
