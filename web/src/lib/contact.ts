export type Contact = {
  description: string[],
};

const Contacts = {
  new: function(contact: Contact): Contact {
    return contact;
  },
};

export default Contacts;
