<script setup lang="ts">
import BoxSelector from './BoxSelector.vue'
import Box from './Box.vue'
import { Boxed, currentBox, emitter, EMPTY_PK5, PK5 } from '../utils';
import { Ref, ref } from 'vue';
import { db } from '../data';
import { useSession } from '../composables/useSession';

const { isLoggedIn } = useSession()
const maxBoxes = 12
const boxes: Ref<PK5[][]> = ref(Array(maxBoxes).fill(undefined).map(() => Array(30).fill(EMPTY_PK5)))

if (isLoggedIn) {
    type Response = {
        box: number,
        out: PK5,
        slot: number
    }

    const pkms = await db.query<[Response[]]>("SELECT * FROM boxed FETCH out").catch(() => {
        alert("There was an error when querying server boxes");
        return;
    })

    if (pkms && pkms[0].result) {
        pkms[0].result.forEach(pkm => boxes.value[pkm.box][pkm.slot] = pkm.out)
    }
}

emitter.on("boxChange", async (boxChange) => {

    boxes.value[boxChange.box][boxChange.slot] = boxChange.pkm

    const [record] = await db.create<PK5>("pkm", boxChange.pkm)


    await db.query<Boxed[]>(
        'RELATE (SELECT * FROM user WHERE id = $auth.id)->boxed->$pkm CONTENT { box: $box, slot: $slot };',
        {
            pkm: record.id,
            box: boxChange.box,
            slot: boxChange.slot
        }
    )

})

</script>

<template>
    <BoxSelector :max-boxes="maxBoxes" v-model:current-box="currentBox" />
    <template v-for="i in boxes.length">
        <Box v-model:boxes="boxes[i - 1]" v-if="i - 1 == currentBox" :key="i" :index="i - 1" />
    </template>
</template>