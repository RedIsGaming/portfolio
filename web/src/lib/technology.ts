export type Technology = {
  name: string,
  src: string,
};

const Technologies = {
  new: (technology: Technology[]): Technology[] => technology,
};

export default Technologies;
