export type Footer = {
  copyright: string,
  year: number,
  name: string,
  content: string,
};

const Footers = {
  new: (footer: Footer): Footer => footer,
};

export default Footers;
