export type Footer = {
  copyright: string,
  year: number,
  name: string,
  content: string,
};

const Footers = {
  new: function(footer: Footer): Footer {
    return footer;
  }
}

export default Footers;
