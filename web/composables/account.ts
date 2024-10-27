export interface Account {
    id: string,
    name: string,
    alias: string,
    owner: string,
    kind: string
};

export const kindOptions = [
    'NRA',
    'TFSA',
    'RRSP',
    'FHSA'
]
