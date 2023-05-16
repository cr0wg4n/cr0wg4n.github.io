import ProjectCard from "../Cards/ProjectCard"
import data from '../../data/projects'


const ProjectsList = () => {
  return <div className='grid xl:grid-cols-4 md:grid-cols-3 grid-cols-2 gap-3 md:gap-5'>
    {
      data()
        .map(({ name, url, cover, description }) =>
        <ProjectCard
          key={name}
          name={name}
          url={url}
          cover={cover}
          description={description}
        />)
    }
  </div>
}

export default ProjectsList
