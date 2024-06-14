export type RedIsGaming = {
  name: string,
  src?: string,
  description: string[],
};

const Reddy = {
  new: function(redisgaming: RedIsGaming[]): RedIsGaming[] {
    return redisgaming;
  }
}

export default Reddy;
