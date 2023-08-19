<script setup lang="ts">
import { ref, watch } from 'vue';
import BoxSlot from './BoxSlot.vue'
import draggable from 'vuedraggable'
import { EMPTY_PK5, PK5, currentBox } from '../utils'
import { db, isLoggedIn } from '../data';
import { LockClosedIcon } from '@heroicons/vue/24/solid'
import { useCookies } from '@vueuse/integrations/useCookies.mjs';

const cookies = useCookies(['jwt'])

const PROPS = defineProps<{
    boxes: PK5[],
    index: number
}>();

const emit = defineEmits<{
    (event: "update:boxes", box: PK5[]): PK5[]
}>()

let boxes = ref(Array.from(PROPS.boxes)) // Is this fine?
const BOXES_LENGTH = boxes.value.length;
const draggedContext = ref();
const relatedContext = ref();
const receiverId = ref("");

watch(PROPS, watched => {
    boxes.value = Array.from(watched.boxes)
})

for (let i = 0; i < 30 - BOXES_LENGTH; i++) {
    boxes.value.push(EMPTY_PK5)
}

function onMove(e: any) {
    draggedContext.value = e.draggedContext
    relatedContext.value = e.relatedContext
    receiverId.value = e.to.id  // We store the id in a variable, as the onEnd event sets the receiver to the sender

    return false;   // We always return false so we keep the swapping behaviour even inside the Box


}

async function onEnd() {

    if(receiverId.value == "party") {

        db.authenticate(cookies.get('jwt'))
        let pkm = await db.query<string[]>(
            "DELETE pkm WHERE <-boxed.slot = [$slot] AND <-boxed.box = [$box]",
            {
                slot: draggedContext.value.index,
                box: currentBox.value
            }
        )

        if (pkm[0].status == "ERR") { alert("There was an error moving back the pokemon from the server"); return; }

    }

    relatedContext.value.component.alterList((list: PK5[]) => { list[relatedContext.value.index] = draggedContext.value.element })
    boxes.value[draggedContext.value.index] = relatedContext.value.element

    emit("update:boxes", boxes.value)

}

</script>

<template>
    <draggable id="box" v-model="boxes" class="md:grid grid-cols-6 grid-rows-5 gap-5 flex flex-wrap justify-evenly" :class="{'blur': !isLoggedIn}"
        :group="isLoggedIn ? 'pkm' : ''" filter=".no-drag" :move="onMove" @end="onEnd">
        <template #item="{ element: pkm }">
            <BoxSlot :pkm="pkm" :class="{'no-drag': !isLoggedIn }" />
        </template>
    </draggable>
    <div v-if="!isLoggedIn" class="absolute z-50 flex items-center gap-2 text-xl p-4 h-16 font-semibold rounded-full shadow bg-white top-1/2 -translate-y-1/2">
        <LockClosedIcon class="w-8" />
        You must be logged in
    </div>
</template>