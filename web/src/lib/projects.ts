export type Project = {
  name: string,
  icon: {
    src: string,
    alt: string,
  },
  description: string,
  keywords: string[],
  language: {
    name: string,
    color: string,
  },
  license: string,
  url: string,
  target: string,
};

const Projects = {
  new: (project: Project): Project => project,
};

export default Projects;
