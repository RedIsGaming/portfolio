export type About = {
  description: string[],
};

const Abouts = {
  new: (about: About[]): About[] => about,
};

export default Abouts;
