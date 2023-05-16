
interface ProjectCardProps {
  url: string
  cover: string
  name: string
  description?: string
}

const ProjectCard: React.FC<ProjectCardProps> = ({
  url, 
  cover,
  name,
  description
}: ProjectCardProps) => {
  return <div className='mt-4 relative hover:cursor-pointer hover:border-b-2' title={description}>
    <a href={url} target={'_blank'} rel={"noreferrer"}>
      <img src={cover} width={'100%'} alt='ehc group main page' className='h-40 rounded-t-lg object-cover' />
      <div className='absolute before:content-[""] bottom-0 bg-black w-full opacity-80 px-2 py-1 font-quicksand md:text-sm text-xs'>
        {name}
      </div>
    </a>
  </div>
}

export default ProjectCard