import './App.css';
import ProfileCard from './components/Cards/ProfileCard';
import SimpleBox from './components/Boxes/SimpleBox';

function App() {
  return (
    <div className='bg-blue-gray-900 p-4 md:p-10 font-serif text-blue-gray-50'>

      <SimpleBox firstColorClass='from-deep-orange-900' secondColorClass='to-deep-orange-500'>
        <ProfileCard 
            name={'Mauricio Matias C.'}
            avatar={'/images/mauri.jpeg'}
            avatarHover={'/images/cr0wg4n.png'}
            description={<div>
                Hi, I'm Mauricio. Technology Building and designing is my main passion. 
                Currently, I still learning about systems programming and
                <span className='text-light-blue-200'>
                  &nbsp;
                  <a href="https://www.rust-lang.org/" rel="nofollow noopener noreferrer" target={'_blank'}>@rust</a> 
                  &nbsp;
                </span>
                for fun. I love and follow 
                the Open Source and Free Sofware initiatives. I occasionally instruct 
                programming and share my day-to-day tech experience as Software Developer through talks.
            </div>} 
            role={'Software Developer'}
        />
      </SimpleBox>

      <SimpleBox firstColorClass="from-teal-900" secondColorClass="to-teal-500" title='Side Projects' orientation>
        <div className='h-64 w-64 bg-yellow-200 mt-4'>
          asdasdsd
        </div>
      </SimpleBox>

      <SimpleBox firstColorClass="from-deep-purple-900" secondColorClass={'to-deep-purple-500'} title='My Blog Posts'>

      </SimpleBox>

      <div className="bg-black h-48 w-12/12 mr-0 ml-auto rounded-xl text-white shadow-xl hover:shadow-2xl p-10">
        
      </div>
    </div>
  );
}

export default App;
