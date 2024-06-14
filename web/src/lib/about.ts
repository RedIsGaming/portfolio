export type About = {
  description: string[],
};

const Abouts = {
  new: function(about: About[]): About[] {
    return about;
  }
}

export default Abouts;
