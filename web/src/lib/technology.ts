export type Technology = {
  name: string,
  src: string,
};

const Technologies = {
  new: function(technology: Technology[]): Technology[] {
    return technology;
  }
};

export default Technologies;
