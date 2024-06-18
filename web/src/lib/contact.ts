export type Contact = {
  description: string[],
};

const Contacts = {
  new: (contact: Contact): Contact => contact,
};

export default Contacts;
