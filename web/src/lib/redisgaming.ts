export type RedIsGaming = {
  name: string,
  src?: string,
  description: string[],
};

const Reddy = {
  new: (redisgaming: RedIsGaming[]): RedIsGaming[] => redisgaming,
};

export default Reddy;
