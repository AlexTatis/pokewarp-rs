import mitt from "mitt"
import { ref } from "vue"

export interface PK5 {
    id: number,
    species: String,
    item_id: number,
    exp: number,
    ability: String,
    moves: Move[],
    ivs: Stats,
    evs: Stats,
    nature: String,
    nickname: String,
    happiness: number,
    gender: "F" | "M" | "Genderless",
    level: number,
    is_egg: boolean,
    item: Item,
    pokeball: Item,
    ot_name: String,
    ot_id: number,
    ot_sid: number
}

interface Stats {
    hp: number,
    atk: number,
    def: number,
    spa: number,
    spd: number,
    spe: number,
}

interface Move {
    num: number,
    name: String,
    alias: String,
    type: String,
    pp: number,
}

interface Item {
    num: number,
    name: String,
    alias: String,
    sprite: String,
}

export const EMPTY_PK5: PK5 = {
    id: 0,
    species: "MissingNo.",
    item_id: 0,
    exp: 0,
    ability: "",
    moves: [],
    ivs: {
        hp: 0,
        atk: 0,
        def: 0,
        spa: 0,
        spd: 0,
        spe: 0,
    },
    evs: {
        hp: 0,
        atk: 0,
        def: 0,
        spa: 0,
        spd: 0,
        spe: 0,
    },
    nature: "",
    nickname: "MissingNo.",
    happiness: 0,
    gender: "M",
    level: 0,
    is_egg: false,
    item: {
        num: 0,
        name: "",
        alias: "",
        sprite: "",
    },
    pokeball: {
        num: 0,
        name: "",
        alias: "",
        sprite: "",
    },
    ot_id: 0,
    ot_sid: 0,
    ot_name: ""
}

export type BoxChange = {
    box: number,
    slot: number,
    pkm: PK5
}

export type Events = {
    setOverview: PK5,
    newSlotSelected: undefined,
    saveParsed: PK5[],
    boxChange: BoxChange
}

export const emitter = mitt<Events>()
export const currentBox = ref(0)